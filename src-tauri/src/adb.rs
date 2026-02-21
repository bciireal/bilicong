use std::{path::Path, process::Command};

#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;

use anyhow::anyhow;
use serde::Serialize;
use tauri::Result;
use tokio::{
    sync::{Semaphore, SemaphorePermit},
    task::spawn_blocking,
};

use crate::{tauri_bail, tauri_ensure};

// TODO: is value 6 suitable?
static ADB_CONCURRENT_LOCK: Semaphore = Semaphore::const_new(6);

async fn get_adb_command(sid: Option<&str>) -> (Command, SemaphorePermit<'static>) {
    let mut command = Command::new("adb");

    if let Some(s) = sid {
        command.args(["-s", s]);
    }

    // https://learn.microsoft.com/en-us/windows/win32/procthread/process-creation-flags#:~:text=CREATE_NO_WINDOW,0x08000000
    #[cfg(target_os = "windows")]
    command.creation_flags(0x0800_0000);

    let permit = ADB_CONCURRENT_LOCK
        .acquire()
        .await
        .expect("closed semaphore");

    (command, permit)
}

#[derive(Debug, Serialize)]
pub struct AdbDeviceInfo {
    sid: String,
    label: String,
}

#[tauri::command]
pub async fn get_devices() -> Result<Vec<AdbDeviceInfo>> {
    let (mut command, permit) = get_adb_command(None).await;
    let proc = spawn_blocking(move || command.arg("devices").output()).await??;
    drop(permit);

    tauri_ensure!(
        proc.status.success(),
        "ADB invoke error: {}",
        String::from_utf8_lossy(&proc.stderr)
    );

    Ok(String::from_utf8(proc.stdout)
        .map_err(|e| anyhow!(e))?
        .lines()
        .skip(1) // for `List of devices attached` line
        .filter_map(|l| l.split_once('\t'))
        .map(|(sid, label)| AdbDeviceInfo {
            sid: sid.into(),
            label: label.into(),
        })
        .collect())
}

pub async fn ls(sid: &str, path: &str) -> Result<Vec<String>> {
    let (mut command, permit) = get_adb_command(Some(sid)).await;
    let p = path.to_string();
    let proc = spawn_blocking(move || command.arg("shell").arg("ls").arg(&p).output()).await??;
    drop(permit);

    tauri_ensure!(
        proc.status.success(),
        "ADB invoke error: {}",
        String::from_utf8_lossy(&proc.stderr)
    );

    Ok(String::from_utf8(proc.stdout)
        .map_err(|e| anyhow!(e))?
        .lines()
        .map(|child| format!("{path}/{child}"))
        .collect())
}

#[tauri::command]
pub async fn get_all_pages(sid: &str) -> Result<Vec<String>> {
    let root_dir = "/sdcard/Android/data/tv.danmaku.bili/download";

    let video_dirs = ls(sid, root_dir).await?;

    let mut tasks = vec![];
    for p in video_dirs {
        let s = sid.to_string();
        tasks.push(tokio::spawn(async move { ls(&s, &p).await }));
    }

    let mut page_dirs = vec![];

    for t in tasks {
        page_dirs.extend(t.await??);
    }

    Ok(page_dirs)
}

pub async fn cat(sid: &str, path: &str) -> Result<String> {
    let (mut command, permit) = get_adb_command(Some(sid)).await;
    let p = path.to_string();
    let proc = spawn_blocking(move || command.arg("shell").arg("cat").arg(&p).output()).await??;
    drop(permit);

    tauri_ensure!(
        proc.status.success(),
        "ADB invoke error: {}",
        String::from_utf8_lossy(&proc.stderr)
    );

    Ok(String::from_utf8(proc.stdout).map_err(|e| anyhow!(e))?)
}

pub async fn pull(sid: &str, from: &str, to: &Path) -> Result<()> {
    let (mut command, permit) = get_adb_command(Some(sid)).await;
    let (f, t) = (from.to_string(), to.to_path_buf());
    let proc = spawn_blocking(move || command.arg("pull").arg(&f).arg(&t).output()).await??;
    drop(permit);

    tauri_ensure!(
        proc.status.success(),
        "ADB invoke error: {}",
        String::from_utf8_lossy(&proc.stderr)
    );

    Ok(())
}

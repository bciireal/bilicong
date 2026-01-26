use std::{path::Path, process::Command};

#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;

use anyhow::anyhow;
use serde::Serialize;
use tauri::Result;

use crate::{tauri_bail, tauri_ensure};

fn get_adb_command(sid: Option<&str>) -> Command {
    let mut command = Command::new("adb");

    if let Some(s) = sid {
        command.args(["-s", s]);
    }

    // https://learn.microsoft.com/en-us/windows/win32/procthread/process-creation-flags#:~:text=CREATE_NO_WINDOW,0x08000000
    #[cfg(target_os = "windows")]
    command.creation_flags(0x0800_0000);

    command
}

#[derive(Debug, Serialize)]
pub struct AdbDeviceInfo {
    sid: String,
    label: String,
}

#[tauri::command(async)]
pub fn get_devices() -> Result<Vec<AdbDeviceInfo>> {
    let proc = get_adb_command(None).arg("devices").output()?;

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

pub fn ls(sid: &str, path: &str) -> Result<Vec<String>> {
    let proc = get_adb_command(Some(sid))
        .arg("shell")
        .arg("ls")
        .arg(path)
        .output()?;

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

#[tauri::command(async)]
pub fn get_all_pages(sid: &str) -> Result<Vec<String>> {
    let root_dir = "/sdcard/Android/data/tv.danmaku.bili/download";

    let video_dirs = ls(sid, root_dir)?;

    let mut page_dirs = vec![];

    for video_dir in video_dirs {
        page_dirs.extend(ls(sid, &video_dir)?);
    }

    Ok(page_dirs)
}

pub fn cat(sid: &str, path: &str) -> Result<String> {
    let proc = get_adb_command(Some(sid))
        .arg("shell")
        .arg("cat")
        .arg(path)
        .output()?;

    tauri_ensure!(
        proc.status.success(),
        "ADB invoke error: {}",
        String::from_utf8_lossy(&proc.stderr)
    );

    Ok(String::from_utf8(proc.stdout).map_err(|e| anyhow!(e))?)
}

pub fn pull(sid: &str, from: &str, to: impl AsRef<Path>) -> Result<()> {
    let proc = get_adb_command(Some(sid))
        .arg("pull")
        .arg(from)
        .arg(to.as_ref())
        .output()?;

    tauri_ensure!(
        proc.status.success(),
        "ADB invoke error: {}",
        String::from_utf8_lossy(&proc.stderr)
    );

    Ok(())
}

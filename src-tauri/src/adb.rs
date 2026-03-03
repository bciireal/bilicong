use std::path::Path;

use anyhow::anyhow;
use serde::Serialize;
use tauri::Result;

mod command;

#[derive(Debug, Serialize)]
pub struct AdbDeviceInfo {
    sid: String,
    label: String,
}

#[tauri::command]
pub async fn get_devices() -> Result<Vec<AdbDeviceInfo>> {
    let mut adb_command = command::AdbCommand::new();
    adb_command.as_mut_inner().args(["reconnect", "offline"]);
    adb_command.run().await?;

    let mut adb_command = command::AdbCommand::new();
    adb_command.as_mut_inner().args(["devices"]);
    let proc = adb_command.run().await?;

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
    let mut adb_command = command::AdbCommand::new_with_sid(sid);
    adb_command
        .as_mut_inner()
        .arg("shell")
        .args(["ls", "-tr"])
        .arg(path);
    let proc = adb_command.run().await?;

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
    let mut adb_command = command::AdbCommand::new_with_sid(sid);
    adb_command.as_mut_inner().arg("shell").arg("cat").arg(path);
    let proc = adb_command.run().await?;

    Ok(String::from_utf8(proc.stdout).map_err(|e| anyhow!(e))?)
}

pub async fn pull(sid: &str, from: &str, to: &Path) -> Result<()> {
    let mut adb_command = command::AdbCommand::new_with_sid(sid);
    adb_command.as_mut_inner().arg("pull").arg(from).arg(to);
    adb_command.run().await?;

    Ok(())
}

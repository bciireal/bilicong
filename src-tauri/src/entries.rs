use std::path::PathBuf;

use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use tauri::Result;

use crate::adb;
use crate::mix;
use crate::temp_path::TempDir;

mod fallback;
mod v26_avid;
mod v26_bvid;
mod v26_ep_avid;
mod v26_ep_bvid;

#[derive(Debug, Serialize, Deserialize)]
pub struct EntryInfo {
    title: String,
    page: u32,
    page_name: String,
    video_id: String,
    uploader: String,
    cover_url: String,
    video_path: String,
    audio_path: Option<String>,
}

impl EntryInfo {
    pub fn file_name(&self) -> String {
        let raw_name = if self.title == self.page_name {
            format!("{}_P{}_{}.mp4", self.title, self.page, self.video_id)
        } else {
            format!(
                "{}_P{}_{}_{}.mp4",
                self.title, self.page, self.page_name, self.video_id
            )
        };

        raw_name
            .trim_matches(|c| c == '.' || c == ' ')
            .chars()
            .map(|c| {
                let code = c as u32;
                if code < 32 || code == 127 || "\"*/:<>?\\|".contains(c) {
                    '_'
                } else {
                    c
                }
            })
            .collect()
    }
}

#[tauri::command(async)]
pub fn probe_entry(sid: &str, page_path: &str) -> Result<EntryInfo> {
    let entry_data = adb::cat(sid, &format!("{page_path}/entry.json"))?;

    let quality_paths = adb::ls(sid, page_path)?;

    let max_quality_path = quality_paths
        .iter()
        .map(|p| p.trim_end_matches('/'))
        .filter_map(|p| p.rsplit_once('/'))
        .filter_map(|(base, q)| {
            if let Ok(q) = q.parse::<i32>() {
                Some((base, q))
            } else {
                None
            }
        })
        .max_by(|a, b| a.1.cmp(&b.1))
        .ok_or_else(|| anyhow!("path {page_path} have no video dir"))?;

    let quality_path = format!("{}/{}", max_quality_path.0, max_quality_path.1);

    // avoid mistyping, haha
    drop(quality_paths);

    let mut entry_info = if let Ok(entry_info) = v26_bvid::parse(&quality_path, &entry_data) {
        entry_info
    } else if let Ok(entry_info) = v26_avid::parse(&quality_path, &entry_data) {
        entry_info
    } else if let Ok(entry_info) = v26_ep_bvid::parse(&quality_path, &entry_data) {
        entry_info
    } else if let Ok(entry_info) = v26_ep_avid::parse(&quality_path, &entry_data) {
        entry_info
    } else {
        fallback::fallback_parser(&quality_path, &entry_data)
    };

    entry_info.cover_url = entry_info.cover_url.replace("http://", "https://");

    Ok(entry_info)
}

#[tauri::command(async)]
pub fn pull_media(sid: &str, target_path: &str, entry_info: EntryInfo) -> Result<()> {
    let target_path = PathBuf::from(target_path);
    let temp_path = TempDir::new(&entry_info)?;

    let video_temp_path = temp_path.as_path().join("video.m4s");
    let audio_temp_path = entry_info
        .audio_path
        .as_ref()
        .map(|_| temp_path.as_path().join("audio.m4s"));

    adb::pull(sid, &entry_info.video_path, &video_temp_path)?;
    if let Some(p) = &entry_info.audio_path
        && let Some(a) = &audio_temp_path
    {
        adb::pull(sid, p, a)?;
    }

    mix::mix_media(
        video_temp_path,
        audio_temp_path.as_ref(),
        target_path.join(entry_info.file_name()),
    )?;

    drop(entry_info);
    drop(temp_path);
    Ok(())
}

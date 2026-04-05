use std::path::PathBuf;

use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use tauri::Result;
use tokio::task::spawn_blocking;
use tracing::warn;

use crate::adb;
use crate::mix_media;

mod fallback;
mod v1_episode;
mod v1_video;

#[derive(Debug, Serialize, Deserialize)]
pub struct EntryInfo {
    title: String,
    page: u32,
    page_name: String,
    video_id: String,
    uploader: String,
    cover_url: String,
    media_path: String,
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

#[tauri::command]
pub async fn probe_entry(sid: &str, page_path: &str) -> Result<EntryInfo> {
    let entry_data = adb::cat(sid, &format!("{page_path}/entry.json")).await?;
    let quality_paths = adb::ls(sid, page_path).await?;

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

    let parsers = [v1_video::parse, v1_episode::parse];

    let mut entry_info = None;
    for parser in parsers {
        if let Ok(e) = parser(&quality_path, &entry_data) {
            entry_info = Some(e);
            break;
        }
    }

    let mut entry_info =
        entry_info.unwrap_or_else(|| fallback::fallback_parser(&quality_path, &entry_data));

    entry_info.cover_url = entry_info.cover_url.replace("http://", "https://");

    Ok(entry_info)
}

#[tauri::command]
pub async fn pull_media(sid: &str, target_path: &str, entry_info: EntryInfo) -> Result<()> {
    let target_path = PathBuf::from(target_path);
    let temp_path = tempfile::Builder::new()
        .prefix(env!("CARGO_PKG_NAME"))
        .tempdir()?;

    let video_temp_path = temp_path.path().join("video.m4s");
    let audio_temp_path = temp_path.path().join("audio.m4s");

    let has_audio = if let Err(e) = adb::pull(
        sid,
        &format!("{}/audio.m4s", entry_info.media_path),
        &audio_temp_path,
    )
    .await
    {
        warn!(
            "No audio file found or error happend when pulling {:?}: {:?}",
            entry_info, e
        );
        false
    } else {
        true
    };

    adb::pull(
        sid,
        &format!("{}/video.m4s", entry_info.media_path),
        &video_temp_path,
    )
    .await?;

    spawn_blocking(move || {
        mix_media::mix_media(
            &video_temp_path,
            if has_audio {
                Some(&audio_temp_path)
            } else {
                None
            },
            &target_path.join(entry_info.file_name()),
        )
    })
    .await??;

    // Ensure `temp_path` are dropped after transcoding is complete
    drop(temp_path);
    Ok(())
}

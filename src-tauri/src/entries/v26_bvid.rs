use anyhow::{Result, ensure};
use serde::Deserialize;

use super::EntryInfo;

#[derive(Debug, Deserialize)]
struct Base {
    title: String,
    page_data: PageData,
    bvid: String,
    owner_name: String,
    cover: String,
    has_dash_audio: bool,
}

#[derive(Debug, Deserialize)]
struct PageData {
    page: u32,
    part: String,
}

pub fn parse(quality_path: &str, entry_data: &str) -> Result<EntryInfo> {
    let data: Base = serde_json::from_str(entry_data)?;

    ensure!(!data.bvid.is_empty(), "not bvid entry");

    Ok(EntryInfo {
        title: data.title,
        page: data.page_data.page,
        page_name: data.page_data.part,
        video_id: data.bvid,
        uploader: data.owner_name,
        cover_url: data.cover,
        video_path: format!("{quality_path}/video.m4s"),
        audio_path: if data.has_dash_audio {
            Some(format!("{quality_path}/audio.m4s"))
        } else {
            None
        },
    })
}

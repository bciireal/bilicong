use anyhow::{Result, ensure};
use serde::Deserialize;

use super::EntryInfo;

#[derive(Debug, Deserialize)]
struct Base {
    title: String,
    ep: Ep,
    cover: String,
    has_dash_audio: bool,
}

#[derive(Debug, Deserialize)]
struct Ep {
    bvid: String,
    page: u32,
    index_title: String,
}

pub fn parse(quality_path: &str, entry_data: &str) -> Result<EntryInfo> {
    let data: Base = serde_json::from_str(entry_data)?;

    ensure!(!data.ep.bvid.is_empty(), "not bvid entry");

    let uploader = data.title.clone();

    Ok(EntryInfo {
        title: data.title,
        page: data.ep.page,
        page_name: data.ep.index_title,
        video_id: data.ep.bvid,
        uploader,
        cover_url: data.cover,
        video_path: format!("{quality_path}/video.m4s"),
        audio_path: if data.has_dash_audio {
            Some(format!("{quality_path}/audio.m4s"))
        } else {
            None
        },
    })
}

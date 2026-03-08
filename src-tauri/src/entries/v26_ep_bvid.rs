use anyhow::{Result, ensure};
use serde::Deserialize;

use super::EntryInfo;

// {
//   "media_type": 2,
//   "has_dash_audio": true,
//   "is_completed": true,
//   "total_bytes": 79569122,
//   "downloaded_bytes": 79569122,
//   "title": "明日方舟：黎明前奏",
//   "type_tag": "112",
//   "cover": "http:\/\/i0.hdslb.com\/bfs\/archive\/4099eb331bd4b7ee9e3dc8bc5e436ee4075f28fd.jpg",
//   "video_quality": 112,
//   "prefered_video_quality": 112,
//   "guessed_total_bytes": 0,
//   "total_time_milli": 261673,
//   "danmaku_count": 145,
//   "time_update_stamp": 1771662684298,
//   "time_create_stamp": 1771662501005,
//   "can_play_in_advance": true,
//   "interrupt_transform_temp_file": false,
//   "quality_pithy_description": "1080P",
//   "quality_superscript": "高码率",
//   "variable_resolution_ratio": false,
//   "cache_version_code": 8460300,
//   "preferred_audio_quality": 0,
//   "audio_quality": 0,
//   "ep": {
//     "av_id": 562747846,
//     "page": 0,
//     "danmaku": 895154865,
//     "cover": "http:\/\/i0.hdslb.com\/bfs\/archive\/4099eb331bd4b7ee9e3dc8bc5e436ee4075f28fd.jpg",
//     "episode_id": 702954,
//     "index": "片头曲MV",
//     "index_title": "《Alive》 Music Video",
//     "from": "bangumi",
//     "season_type": 4,
//     "width": 3840,
//     "height": 2160,
//     "rotate": 0,
//     "link": "https:\/\/www.bilibili.com\/bangumi\/play\/ep702954",
//     "bvid": "BV1Rv4y1m71c",
//     "sort_index": 2000001
//   },
//   "season_id": "43057"
// }

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

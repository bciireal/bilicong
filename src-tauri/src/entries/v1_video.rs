use anyhow::Result;
use serde::Deserialize;

use super::EntryInfo;

// Entry format for normal video at 2026-03-09
// {
//   "media_type": 2,
//   "has_dash_audio": true,
//   "is_completed": true,
//   "total_bytes": 276284233,
//   "downloaded_bytes": 276284233,
//   "title": "待兼诗歌剧全LIVE合集【赛马娘】",
//   "type_tag": "120",
//   "cover": "http://i2.hdslb.com/bfs/archive/30aa9b27e51cbc276da86e56c7215960aea1f7b1.jpg",
//   "video_quality": 120,
//   "prefered_video_quality": 120,
//   "guessed_total_bytes": 0,
//   "total_time_milli": 153837,
//   "danmaku_count": 34,
//   "time_update_stamp": 1760607439799,
//   "time_create_stamp": 1760607129603,
//   "can_play_in_advance": true,
//   "interrupt_transform_temp_file": false,
//   "quality_pithy_description": "4K",
//   "quality_superscript": "",
//   "variable_resolution_ratio": false,
//   "cache_version_code": 8630300,
//   "preferred_audio_quality": 0,
//   "audio_quality": 0,
//   "avid": 767583163,
//   "spid": 0,
//   "season_id": 0,
//   "bvid": "BV1Lr4y1W7Gn", <-- maybe empyt: ""
//   "owner_id": 256237451,
//   "owner_name": "ALTNOIR",
//   "is_charge_video": false,
//   "verification_code": 0,
//   "page_data": {
//     "cid": 830372782,
//     "page": 3,
//     "from": "vupload",
//     "part": "ユメヲカケル!",
//     "link": "bilibili://video/767583163?cid=830372782",
//     "rich_vid": "",
//     "has_alias": false,
//     "tid": 0,
//     "width": 3840,
//     "height": 2160,
//     "rotate": 0,
//     "download_title": "",
//     "download_subtitle": ""
//   },
//   "ep": null
// }

#[derive(Debug, Deserialize)]
struct Base {
    title: String,
    page_data: PageData,
    avid: u64,
    bvid: String,
    owner_name: String,
    cover: String,
}

#[derive(Debug, Deserialize)]
struct PageData {
    page: u32,
    part: Option<String>,
}

pub fn parse(quality_path: &str, entry_data: &str) -> Result<EntryInfo> {
    let data: Base = serde_json::from_str(entry_data)?;

    let video_id = if data.bvid.is_empty() {
        format!("av{}", data.avid)
    } else {
        data.bvid
    };

    let page_name = if let Some(p) = data.page_data.part {
        p
    } else {
        data.title.clone()
    };

    Ok(EntryInfo {
        title: data.title,
        page: data.page_data.page,
        page_name,
        video_id,
        uploader: data.owner_name,
        cover_url: data.cover,
        media_path: quality_path.to_string(),
    })
}

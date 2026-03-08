use anyhow::Result;
use serde::Deserialize;

use super::EntryInfo;

// {
//   "media_type": 2,
//   "has_dash_audio": true,
//   "is_completed": true,
//   "total_bytes": 17961757,
//   "downloaded_bytes": 17961757,
//   "title": "我收到了个不寻常的快递！",
//   "type_tag": "80",
//   "cover": "http:\/\/i2.hdslb.com\/bfs\/archive\/f4a563b4553038fe6fe932b278e4ddb62540a3ae.jpg",
//   "video_quality": 80,
//   "prefered_video_quality": 80,
//   "guessed_total_bytes": 0,
//   "total_time_milli": 353965,
//   "danmaku_count": 1200,
//   "time_update_stamp": 1760786487517,
//   "time_create_stamp": 1760786473918,
//   "can_play_in_advance": true,
//   "interrupt_transform_temp_file": false,
//   "quality_pithy_description": "1080P",
//   "quality_superscript": "",
//   "variable_resolution_ratio": false,
//   "cache_version_code": 8460300,
//   "preferred_audio_quality": 0,
//   "audio_quality": 0,
//   "avid": 370848988,
//   "spid": 0,
//   "seasion_id": 0,
//   "bvid": "",
//   "owner_id": 53456,
//   "owner_name": "Warma",
//   "is_charge_video": false,
//   "verification_code": 0,
//   "page_data": {
//     "cid": 193078446,
//     "page": 1,
//     "from": "vupload",
//     "part": "【warma】我收到了个不寻常的快递！",
//     "link": "",
//     "rich_vid": "",
//     "has_alias": false,
//     "tid": 47,
//     "width": 1920,
//     "height": 1080,
//     "rotate": 0,
//     "download_title": "",
//     "download_subtitle": ""
//   }
// }

#[derive(Debug, Deserialize)]
struct Base {
    title: String,
    page_data: PageData,
    avid: u64,
    owner_name: String,
    cover: String,
    has_dash_audio: bool,
}

#[derive(Debug, Deserialize)]
struct PageData {
    page: u32,
    part: Option<String>,
}

pub fn parse(quality_path: &str, entry_data: &str) -> Result<EntryInfo> {
    let data: Base = serde_json::from_str(entry_data)?;

    let video_id = format!("av{}", data.avid);

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
        video_path: format!("{quality_path}/video.m4s"),
        audio_path: if data.has_dash_audio {
            Some(format!("{quality_path}/audio.m4s"))
        } else {
            None
        },
    })
}

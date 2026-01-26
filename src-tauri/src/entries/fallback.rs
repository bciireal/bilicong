use super::EntryInfo;

pub fn fallback_parser(quality_path: &str, entry_data: &str) -> EntryInfo {
    let data_hash = blake3::hash(entry_data.as_bytes()).to_string();

    EntryInfo {
        title: data_hash,
        page: 0,
        page_name: "--".into(),
        video_id: "--".into(),
        uploader: "--".into(),
        cover_url: "data:,".into(),
        video_path: format!("{quality_path}/video.m4s"),
        audio_path: Some(format!("{quality_path}/audio.m4s")),
    }
}

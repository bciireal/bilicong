use std::{path::Path, process::Command};

#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;

use anyhow::{Result, anyhow, ensure};

fn get_ffmpeg_command() -> Command {
    let mut command = Command::new("ffmpeg");

    // https://learn.microsoft.com/en-us/windows/win32/procthread/process-creation-flags#:~:text=CREATE_NO_WINDOW,0x08000000
    #[cfg(target_os = "windows")]
    command.creation_flags(0x0800_0000);

    command
}

pub fn mix_media(
    video_path: impl AsRef<Path>,
    audio_path: Option<&impl AsRef<Path>>,
    output_path: impl AsRef<Path>,
) -> Result<()> {
    std::fs::create_dir_all(
        output_path
            .as_ref()
            .parent()
            .ok_or_else(|| anyhow!("`output_path` should be a file"))?,
    )?;

    let mut proc = get_ffmpeg_command();

    // General Parameters
    proc.args(["-y", "-loglevel", "warning"]);

    // Input
    proc.arg("-i");
    proc.arg(video_path.as_ref());
    if let Some(p) = audio_path {
        proc.arg("-i");
        proc.arg(p.as_ref());
    }

    // Stream Mapping
    proc.args(["-map", "0:v:0"]);
    if audio_path.is_some() {
        proc.args(["-map", "1:a:0"]);
    }

    // Codec Config
    proc.args(["-c", "copy"]);
    proc.args(["-movflags", "+faststart"]);
    proc.args(["-map_chapters", "-1"]);
    proc.args(["-map_metadata", "-1"]);

    // Output
    proc.arg(output_path.as_ref());

    let stat = proc.status()?;
    ensure!(stat.success(), "FFmpeg stream mixing not success");

    Ok(())
}

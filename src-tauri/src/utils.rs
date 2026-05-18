use std::fs::{FileTimes, OpenOptions};
use std::path::Path;
use std::time::{Duration, UNIX_EPOCH};

#[cfg(windows)]
use std::os::windows::fs::FileTimesExt;

// TODO: Untested platform
#[cfg(target_vendor = "apple")]
use std::os::darwin::fs::FileTimesExt;

use anyhow::{Result, anyhow};

pub fn file_set_time(path: impl AsRef<Path>, timestamp: u64) -> Result<()> {
    let target_time = UNIX_EPOCH
        .checked_add(Duration::from_secs(timestamp))
        .ok_or_else(|| anyhow!("timestamp {timestamp} overflow"))?;

    let mut file_time = FileTimes::new()
        .set_accessed(target_time)
        .set_modified(target_time);

    #[cfg(any(windows, target_vendor = "apple"))]
    {
        file_time = file_time.set_created(target_time);
    }

    OpenOptions::new()
        .write(true)
        .open(path)?
        .set_times(file_time)?;

    Ok(())
}

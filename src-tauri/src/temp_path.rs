use std::fmt::Debug;
use std::path::{Path, PathBuf};

use anyhow::Result;

fn current_millis_ts() -> u128 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("SystemTime before UNIX EPOCH!")
        .as_millis()
}

pub struct TempDir {
    path: PathBuf,
}

impl TempDir {
    pub fn new(ident: &impl Debug) -> Result<Self> {
        let path = std::env::temp_dir().join(format!(
            "{}-{}-{}",
            env!("CARGO_PKG_NAME"),
            blake3::hash(format!("{ident:?}").as_bytes()),
            current_millis_ts()
        ));

        std::fs::create_dir(&path)?;

        Ok(Self { path })
    }

    pub fn as_path(&self) -> &Path {
        self.path.as_path()
    }
}

impl Drop for TempDir {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.path);
    }
}

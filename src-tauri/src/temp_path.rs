use std::fmt::Debug;
use std::path::{Path, PathBuf};

use anyhow::Result;

pub struct TempDir {
    path: PathBuf,
}

impl TempDir {
    pub fn new(ident: &impl Debug) -> Result<Self> {
        let path = std::env::temp_dir().join(format!(
            "{}-{}",
            env!("CARGO_PKG_NAME"),
            blake3::hash(format!("{ident:?}").as_bytes())
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

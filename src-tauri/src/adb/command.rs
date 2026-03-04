use std::ffi::OsString;
use std::process::{Command, Output};

#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;

use anyhow::{Result, ensure};
use tokio::sync::Semaphore;
use tokio::task::spawn_blocking;

// TODO: is value 4 suitable?
static ADB_CONCURRENT_LOCK: Semaphore = Semaphore::const_new(4);

pub struct AdbCommand {
    command: Command,
}

impl AdbCommand {
    pub fn new_with_sid<T, S>(sid: T) -> Self
    where
        T: Into<Option<S>>,
        S: Into<OsString>,
    {
        let sid = sid.into().map(Into::into);

        let mut command = Command::new("adb");

        if let Some(s) = sid {
            command.arg("-s");
            command.arg(s);
        }

        // https://learn.microsoft.com/en-us/windows/win32/procthread/process-creation-flags#:~:text=CREATE_NO_WINDOW,0x08000000
        #[cfg(target_os = "windows")]
        command.creation_flags(0x0800_0000);

        Self { command }
    }

    pub fn new() -> Self {
        Self::new_with_sid::<Option<OsString>, OsString>(None)
    }

    pub fn as_mut_inner(&mut self) -> &mut Command {
        &mut self.command
    }

    pub async fn run(mut self) -> Result<Output> {
        let permit = ADB_CONCURRENT_LOCK.acquire().await?;

        let output = spawn_blocking(move || self.command.output()).await??;

        drop(permit);

        ensure!(
            output.status.success(),
            "ADB invoke error: {}",
            String::from_utf8_lossy(&output.stderr)
        );

        Ok(output)
    }
}

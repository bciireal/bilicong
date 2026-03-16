#![warn(clippy::pedantic)]

use anyhow::{Result, ensure};

fn get_git_commit_short_id() -> Result<String> {
    let proc_output = std::process::Command::new("git")
        .args(["log", "-1", "--format=%h"])
        .output()?;

    ensure!(
        proc_output.status.success(),
        "Faild to get git describe: {}",
        String::from_utf8_lossy(&proc_output.stderr)
    );

    Ok(String::from_utf8(proc_output.stdout)?)
}

fn main() {
    let commit_id = get_git_commit_short_id().unwrap_or_else(|e| {
        println!("cargo::warning=Faild to get git log: {e}");
        "UNKNOWN".into()
    });
    println!(
        "cargo::rustc-env=PROJECT_VERSION=v{}+g{commit_id}",
        env!("CARGO_PKG_VERSION")
    );

    tauri_build::build();
}

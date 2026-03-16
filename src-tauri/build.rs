#![warn(clippy::pedantic)]

use anyhow::{Result, ensure};

fn get_git_describe() -> Result<String> {
    let proc_output = std::process::Command::new("git")
        .args(["describe", "--tags", "--always", "--dirty"])
        .output()?;

    ensure!(
        proc_output.status.success(),
        "Faild to get git describe: {}",
        String::from_utf8_lossy(&proc_output.stderr)
    );

    Ok(String::from_utf8(proc_output.stdout)?)
}

fn main() {
    let git_describe = get_git_describe().unwrap_or_else(|e| {
        println!("cargo::warning=Faild to get git describe: {e}");
        format!("v{}+UNKNOWN", env!("CARGO_PKG_VERSION"))
    });
    println!("cargo::rustc-env=PROJECT_VERSION={git_describe}");

    tauri_build::build();
}

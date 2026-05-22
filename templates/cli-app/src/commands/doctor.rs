use anyhow::{Context, Result};
use std::process::Command;

use crate::config::Config;

pub fn run(cfg: &Config) -> Result<()> {
    println!("=== Environment Doctor ===\n");

    check_rustc(cfg)?;
    check_cargo(cfg)?;
    check_target_dir(cfg)?;

    println!("\nAll checks passed.");
    Ok(())
}

fn check_rustc(_cfg: &Config) -> Result<()> {
    let output = Command::new("rustc")
        .args(["--version"])
        .output()
        .context("failed to run rustc")?;

    let version = String::from_utf8_lossy(&output.stdout).trim().to_string();
    let version = version.lines().next().unwrap_or("unknown");

    println!("rustc: {}", version);
    Ok(())
}

fn check_cargo(_cfg: &Config) -> Result<()> {
    let output = Command::new("cargo")
        .args(["--version"])
        .output()
        .context("failed to run cargo")?;

    let version = String::from_utf8_lossy(&output.stdout).trim().to_string();
    let version = version.lines().next().unwrap_or("unknown");

    println!("cargo: {}", version);
    Ok(())
}

fn check_target_dir(cfg: &Config) -> Result<()> {
    println!("target dir: {}", cfg.target_dir);

    let path = std::path::Path::new(&cfg.target_dir);
    if path.exists() {
        println!("  [OK] directory exists");
    } else {
        println!("  [INFO] directory does not exist yet");
    }

    Ok(())
}
use std::process;
use crate::error;
use tempfile::NamedTempFile;
use anyhow::{ Context, bail };
use crate::structures::{ NiuxConfig };
use crate::utils::get_privilege_type;
pub fn run_bash_interactive(args: &[&str]) -> anyhow::Result<()> {
    let first = if args[0] == "sudo" { NiuxConfig::get()?.environment.su_type }
    else { args[0].to_string()};
    process::Command::new(first)
        .args(&args[1..])
        .env("PATH", std::env::var("PATH").unwrap_or_default())
        .status()?;
    Ok(())
}
fn bash(args: &[&str], type_bash: bool) -> anyhow::Result<String> {
    let first = if type_bash {
        if args[0] == "sudo" { NiuxConfig::get()?.environment.su_type }
        else { args[0].to_string() }
    } else {
        if args[0] == "sudo" { get_privilege_type()? }
        else { args[0].to_string() }
    };
    let result = process::Command::new(first)
        .args(&args[1..])
        .env("PATH", std::env::var("PATH").unwrap_or_default())
        .output()
        .context("Failed to run bash command")?;
    if !result.status.success() {
        error!("{}", String::from_utf8_lossy(&result.stderr));
        process::exit(1);
    }
    Ok(String::from_utf8(result.stdout).unwrap().trim().to_string())
}
pub fn run_bash(args: &[&str]) -> anyhow::Result<String> {
    bash(args, true)
    }

pub fn run_early_bash(args: &[&str]) -> anyhow::Result<String> {
    bash(args, false)
}
pub fn writer_init(config_path: &str, hooks_path: &str) -> anyhow::Result<()> {
    run_early_bash(&["sudo", "niux-writer", "init", config_path, hooks_path])?;
    Ok(())
}
pub fn writer_write(tmp_path: &str, dest_path: &str) -> anyhow::Result<()> {
    run_early_bash(&["sudo", "niux-writer", "write", tmp_path, dest_path])?;
    Ok(())
}
pub fn command_exists(cmd: &str) -> bool {
    process::Command::new("which")
        .arg(cmd)
        .env("PATH", std::env::var("PATH").unwrap_or_default())
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}
pub fn write_changes_to_config(content: &str, dest_path: &str) -> anyhow::Result<()> {
    let tmp = NamedTempFile::new().context("Failed to create tmp file")?;
    std::fs::write(tmp.path(), content).context("Failed to write content in tmp")?;
    writer_write(tmp.path().to_str().context("Invalid tmp path")?, dest_path)?;
    Ok(())
}
pub fn user_input() -> String {
    let mut user_input = String::new();
    print!("> ");
    std::io::Write::flush(&mut std::io::stdout()).ok();
    std::io::stdin()
        .read_line(&mut user_input)
        .unwrap_or_else(|e| { error!("{e}"); process::exit(1); });
    user_input
}
#[allow(clippy::ptr_arg)]
pub fn search_range(lines: &Vec<String>, marker: bool) -> anyhow::Result<Vec<String>> {
    let config = NiuxConfig::get()?;
    let config_marker = if marker { config.config_markers.marker_system } else { config.config_markers.marker_home };
    let config_marker_end = if marker { config.config_markers.marker_system_end } else { config.config_markers.marker_home_end };
    let Some(marker_start) = lines.iter().position(|l| l.contains(&config_marker)) else {
        bail!("Marker is not found: {config_marker}");
    };
    let Some(marker_end) = lines.iter().position(|l| l.contains(&config_marker_end)) else {
        bail!("Marker is not found: {config_marker_end}");
    };
    if marker_start >= marker_end {
        bail!("marker end goes earlier marker home, please move your packages in separate config or use custom markers");
    }
    Ok(lines[marker_start+1..marker_end].to_vec())
}

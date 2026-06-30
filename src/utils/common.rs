use std::process;
use crate::error;
use tempfile::NamedTempFile;
use anyhow::{ 
    Context, 
    bail 
};
use crate::structures::{
    NiuxConfig,
    PackageType,
};
use crate::structures::niux_config::{
    ConfigMarkers,
    ConfigPaths,
};
use crate::utils::get_privilege_type;
pub fn run_bash_interactive(args: &[&str]) -> anyhow::Result<()> {
    let first = if args[0] == "sudo" { NiuxConfig::get()?.environment.su_type }
    else { args[0].to_string()};
    let status = process::Command::new(first)
        .args(&args[1..])
        .env("PATH", std::env::var("PATH").unwrap_or_default())
        .status()?;
    if !status.success() {
        bail!("Command executed unsuccessfully (exit code: {}), (command: {})", status.code().unwrap_or(-1), args.join(" "));
    }
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
pub fn search_range(lines: &Vec<String>, ptype: &PackageType) -> anyhow::Result<Vec<String>> {
    let config = NiuxConfig::get()?;
    let (marker_start, marker_end) = ptype.get_markers(&config.config_markers);
    let Some(marker_start) = lines.iter().position(|l| l.contains(&marker_start)) else {
        bail!("Marker is not found: {marker_start}");
    };
    let Some(marker_end) = lines.iter().position(|l| l.contains(&marker_end)) else {
        bail!("Marker is not found: {marker_end}");
    };
    if marker_start >= marker_end {
        bail!("marker end comes before the home marker. Please move your packages to a separate config or use custom markers");
    }
    Ok(lines[marker_start+1..marker_end].to_vec())
}
pub trait GetCfgData {
    fn get_marker_start<'a>(&self, markers: &'a ConfigMarkers) -> &'a str;
    fn get_marker_end<'a>(&self, markers: &'a ConfigMarkers) -> &'a str;
    fn get_config_path<'a>(&self, paths: &'a ConfigPaths) -> &'a str;

    fn get_markers<'a>(&self, markers: &'a ConfigMarkers) -> (&'a str, &'a str) {
        (self.get_marker_start(markers), self.get_marker_end(markers))
    }
}
impl GetCfgData for PackageType {
    fn get_marker_start<'a>(&self, markers: &'a ConfigMarkers) -> &'a str {
        match self {
            PackageType::Home => &markers.marker_home,
            PackageType::System => &markers.marker_system,
        }
    }
    fn get_marker_end<'a>(&self, markers: &'a ConfigMarkers) -> &'a str {
        match self {
            PackageType::Home => &markers.marker_home_end,
            PackageType::System => &markers.marker_system_end,
        }
    }
    fn get_config_path<'a>(&self, paths: &'a ConfigPaths) -> &'a str {
        match self {
            PackageType::Home => &paths.config_path_home,
            PackageType::System => &paths.config_path_system,
        }
    }
}

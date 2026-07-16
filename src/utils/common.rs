use crate::error;
use crate::structures::models::{Early, Home, Just, System};
use crate::structures::niux_config::ConfigMarkers;
use crate::structures::{AutoGenNiuxConfig, NiuxConfig};
use anyhow::{Context, bail};
use std::borrow::Cow;
use std::path::PathBuf;
use std::process;
use tempfile::NamedTempFile;
pub trait BashType {
    fn otype(first: &str) -> Cow<'_, str>;
}
impl BashType for Just {
    fn otype(first: &str) -> Cow<'_, str> {
        if first == "sudo" {
            Cow::Borrowed(&NiuxConfig::get().environment.su_type)
        } else {
            Cow::Borrowed(first)
        }
    }
}

impl BashType for Early {
    fn otype(first: &str) -> Cow<'_, str> {
        if first == "sudo" {
            Cow::Owned(get_privilege_type())
        } else {
            Cow::Borrowed(first)
        }
    }
}

pub fn run_bash_interactive<T>(args: &[&str]) -> anyhow::Result<()>
where
    T: BashType,
{
    let first = T::otype(args[0]);
    let status = process::Command::new(&*first)
        .args(&args[1..])
        .env("PATH", std::env::var("PATH").unwrap_or_default())
        .status()?;
    if !status.success() {
        bail!(
            "Command executed unsuccessfully (exit code: {}), (command: {})",
            status.code().unwrap_or(-1),
            args.join(" ")
        );
    }
    Ok(())
}

pub fn bash<T>(args: &[&str]) -> anyhow::Result<String>
where
    T: BashType,
{
    let first = T::otype(args[0]);
    let result = process::Command::new(&*first)
        .args(&args[1..])
        .env("PATH", std::env::var("PATH").unwrap_or_default())
        .output()
        .context("Failed to run bash command")?;
    if !result.status.success() {
        error!("{}", String::from_utf8_lossy(&result.stderr));
        process::exit(1);
    }
    Ok(String::from_utf8(result.stdout)
        .with_context(|| "Invalid UTF-8 symbols")?
        .trim()
        .to_string())
}

pub fn writer_init(paths: AutoGenNiuxConfig) -> anyhow::Result<()> {
    bash::<Early>(&[
        "sudo",
        "niux-writer",
        "init",
        paths.config_path.to_str().context("Invalid config path")?,
        paths
            .hooks_config_path
            .to_str()
            .context("Invalid hook config path")?,
    ])?;
    Ok(())
}

pub fn writer_write(tmp_path: &str, dest_path: PathBuf) -> anyhow::Result<()> {
    bash::<Early>(&[
        "sudo",
        "niux-writer",
        "write",
        tmp_path,
        dest_path.to_str().with_context(|| "Invalid config path")?,
    ])?;
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

pub fn write_changes_to_config(content: &str, dest_path: PathBuf) -> anyhow::Result<()> {
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
        .unwrap_or_else(|e| {
            error!("{e}");
            process::exit(1);
        });
    user_input
}

pub trait ConfigTypeKind {
    fn get_markers(markers: &ConfigMarkers) -> (&str, &str);
}

impl ConfigTypeKind for Home {
    fn get_markers(markers: &ConfigMarkers) -> (&str, &str) {
        (&markers.marker_home, &markers.marker_home_end)
    }
}

impl ConfigTypeKind for System {
    fn get_markers(markers: &ConfigMarkers) -> (&str, &str) {
        (&markers.marker_system, &markers.marker_system_end)
    }
}

impl NiuxConfig {
    pub fn get_range<T>(&self, content: &str) -> anyhow::Result<Vec<String>>
    where
        T: ConfigTypeKind,
    {
        let (marker_start, marker_end) = T::get_markers(&self.config_markers);
        let lines: Vec<&str> = content.lines().collect();

        let Some(marker_start) = lines.iter().position(|l| l.contains(marker_start)) else {
            bail!("Marker is not found: {marker_start}");
        };

        let Some(marker_end) = lines[marker_start..]
            .iter()
            .position(|l| l.contains(marker_end))
        else {
            bail!("Marker is not found: {marker_end}");
        };
        Ok(lines[marker_start + 1..marker_end + marker_start]
            .iter()
            .copied()
            .map(String::from)
            .collect())
    }
}

pub fn get_privilege_type() -> String {
    for su in &["doas", "sudo", "run0", "pkexec"] {
        if command_exists(su) {
            return su.to_string();
        }
    }
    println!("Privilege escalation tool not found. Enter yours (e.g. sudo, doas)");
    user_input().trim().to_string()
}

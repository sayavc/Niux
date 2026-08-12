use crate::error;
use crate::structures::models::{Early, Home, Just, PackagesRange, System};
use crate::structures::niux_config::ConfigMarkers;
use crate::structures::{AutoGenNiuxConfig, NiuxConfig};
use anyhow::{Context, bail};
use colored::{Colorize, CustomColor};
use git_version::git_version;
use std::borrow::Cow;
use std::iter::Peekable;
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
        error!(
            "Failed to execute command: {}\n {}",
            args.join(" "),
            String::from_utf8_lossy(&result.stderr)
        );
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
    pub fn get_range<T>(&self, content: &str) -> anyhow::Result<PackagesRange>
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

        let packages: Vec<String> = lines[marker_start + 1..marker_end + marker_start]
            .iter()
            .copied()
            .map(String::from)
            .collect();

        let Some(indent) = packages
            .first()
            .map(|p| p.len() - p.trim_start().len())
            .or_else(|| {
                lines
                    .get(marker_start)
                    .map(|m| m.len() - m.trim_start().len())
            })
        else {
            bail!("Failed to get indent, markers is wrong");
        };

        Ok(PackagesRange {
            packages,
            indent,
            start: marker_start + 1,
            end: marker_start + marker_end,
        })
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

pub fn version() -> String {
    if let Some(v) = option_env!("NIUX_BUILD_VERSION_STRING") {
        return String::from(v);
    }

    const MAJOR: &str = env!("CARGO_PKG_VERSION_MAJOR");
    const MINOR: &str = env!("CARGO_PKG_VERSION_MINOR");
    const PATCH: &str = env!("CARGO_PKG_VERSION_PATCH");

    let commit =
        option_env!("NIUX_BUILD_COMMIT").unwrap_or(git_version!(fallback = "unknown commit"));

    if PATCH == "0" {
        format!("{MAJOR}.{MINOR:0>2} ({commit})")
    } else {
        format!("{MAJOR}.{MINOR:0>2}.{PATCH} ({commit})")
    }
}
pub trait Color {
    fn cold_white(self) -> colored::ColoredString;
}
impl Color for &str {
    fn cold_white(self) -> colored::ColoredString {
        self.custom_color(CustomColor {
            r: 240,
            g: 246,
            b: 252,
        })
    }
}
pub fn print_packages<'a>(
    ptype: &str,
    mut packages: Peekable<impl Iterator<Item = &'a String>>,
    whitespace: bool,
) -> bool {
    if packages.peek().is_some() {
        println!(" {}:", ptype.cyan().bold());
        for p in packages {
            println!(" {} {}", "-".blue(), p.trim().cold_white());
        }
        if whitespace {
            println!();
        }
        true
    } else {
        false
    }
}
pub fn print_raw_packages(packages: &Vec<String>) {
    for p in packages {
        println!("{}", p.trim())
    }
}
pub trait SortExt {
    fn sorted(self) -> Self;
}
impl<T: Ord> SortExt for Vec<T> {
    fn sorted(mut self) -> Self {
        self.sort();
        self
    }
}
pub trait SanitizePackages {
    fn sanitize_packages(self) -> Self;
}
impl SanitizePackages for Vec<String> {
    fn sanitize_packages(self) -> Self {
        self.into_iter()
            .filter(|p| !p.contains(['(', ')', '[', ']', '$', '{', '}', ',']))
            .collect()
    }
}

use crate::structures::models::{Early, Home, Just, PackagesRange, System};
use crate::structures::niux_config::ConfigMarkers;
use crate::structures::{AutoGenNiuxConfig, NiuxConfig};
use colored::{Colorize, CustomColor};
use git_version::git_version;
use std::borrow::Cow;
use std::iter::Peekable;
use std::path::PathBuf;
use std::process;
use tempfile::NamedTempFile;
pub trait BashType {
    fn otype(first: &str) -> crate::NiuxResult<Cow<'_, str>>;
}
impl BashType for Just {
    fn otype(first: &str) -> crate::NiuxResult<Cow<'_, str>> {
        if first == "sudo" {
            Ok(Cow::Borrowed(&NiuxConfig::get().environment.su_type))
        } else {
            Ok(Cow::Borrowed(first))
        }
    }
}

impl BashType for Early {
    fn otype(first: &str) -> crate::NiuxResult<Cow<'_, str>> {
        if first == "sudo" {
            Ok(Cow::Owned(get_privilege_type()?))
        } else {
            Ok(Cow::Borrowed(first))
        }
    }
}

pub fn run_bash_interactive<T>(args: &[&str]) -> crate::NiuxResult<()>
where
    T: BashType,
{
    let first = T::otype(args[0])?;
    let status = process::Command::new(&*first)
        .args(&args[1..])
        .env("PATH", std::env::var("PATH").unwrap_or_default())
        .status()
        .map_err(|e| crate::ExecuteErr::Io { e })?;
    if !status.success() {
        return Err(crate::ExecuteErr::ExitStatus {
            code: status.code().unwrap_or(-1),
            command: args.join(" "),
        }
        .into());
    }
    Ok(())
}

pub fn bash<T>(args: &[&str]) -> crate::NiuxResult<String>
where
    T: BashType,
{
    let first = T::otype(args[0])?;
    let result = process::Command::new(&*first)
        .args(&args[1..])
        .env("PATH", std::env::var("PATH").unwrap_or_default())
        .output()
        .map_err(|e| crate::ExecuteErr::Io { e })?;
    if !result.status.success() {
        return Err(crate::ExecuteErr::ExitStatus {
            code: result.status.code().unwrap_or(-1),
            command: args.join(" "),
        }
        .into());
    }
    Ok(String::from_utf8(result.stdout)
        .map_err(|e| crate::Utf8Err::InvalidUtf8String { e })?
        .trim()
        .to_string())
}

pub fn writer_init(paths: AutoGenNiuxConfig) -> crate::NiuxResult<()> {
    let config_path = paths.config_path;
    let hook_config_path = paths.hooks_config_path;

    let args = [
        "sudo",
        "niux-writer",
        "init",
        config_path.to_str().ok_or(crate::ConfigErr::Invalid {
            path: config_path.clone(),
        })?,
        hook_config_path.to_str().ok_or(crate::ConfigErr::Invalid {
            path: hook_config_path.clone(),
        })?,
    ];

    bash::<Early>(&args)?;

    Ok(())
}

pub fn writer_write(tmp_path: &str, dest_path: PathBuf) -> crate::NiuxResult<()> {
    let args = [
        "sudo",
        "niux-writer",
        "write",
        tmp_path,
        dest_path.to_str().ok_or(crate::ConfigErr::Invalid {
            path: dest_path.clone(),
        })?,
    ];

    bash::<Early>(&args)?;

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

pub fn write_changes_to_config(content: &str, dest_path: PathBuf) -> crate::NiuxResult<()> {
    let tmp = NamedTempFile::new()
        .map_err(|e| crate::TmpErr::Create { e })
        .map_err(crate::IoErr::from)?;

    std::fs::write(tmp.path(), content)
        .map_err(|e| crate::TmpErr::Write { e })
        .map_err(crate::IoErr::from)?;

    writer_write(
        tmp.path().to_str().ok_or(crate::Utf8Err::InvalidUtf8)?,
        dest_path,
    )?;
    Ok(())
}

pub fn user_input() -> crate::NiuxResult<String> {
    let mut user_input = String::new();
    print!("> ");
    std::io::Write::flush(&mut std::io::stdout()).ok();
    std::io::stdin()
        .read_line(&mut user_input)
        .map_err(|e| crate::InputErr::Read { e })?;

    Ok(user_input)
}

pub trait ConfigTypeKind {
    fn get_markers(markers: &ConfigMarkers) -> (&str, &str);
}

impl ConfigTypeKind for Home {
    fn get_markers(markers: &ConfigMarkers) -> (&str, &str) {
        (&markers.home, &markers.home_end)
    }
}

impl ConfigTypeKind for System {
    fn get_markers(markers: &ConfigMarkers) -> (&str, &str) {
        (&markers.system, &markers.system_end)
    }
}

impl NiuxConfig {
    pub fn get_range<T>(&self, content: &str) -> crate::NiuxResult<PackagesRange>
    where
        T: ConfigTypeKind,
    {
        let (marker_start, marker_end) = T::get_markers(&self.config_markers);
        let lines: Vec<&str> = content.lines().collect();

        let Some(marker_start) = lines.iter().position(|l| l.contains(marker_start)) else {
            return Err(crate::NixConfigErr::MarkerNotFound {
                marker: marker_start.to_string(),
            }
            .into());
        };

        let Some(marker_end) = lines[marker_start..]
            .iter()
            .position(|l| l.contains(marker_end))
        else {
            return Err(crate::NixConfigErr::MarkerNotFound {
                marker: marker_end.to_string(),
            }
            .into());
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
            return Err(crate::NixConfigErr::WrongIndent.into());
        };

        Ok(PackagesRange {
            packages,
            indent,
            start: marker_start + 1,
            end: marker_start + marker_end,
        })
    }
}

pub fn get_privilege_type() -> crate::NiuxResult<String> {
    for su in &["doas", "sudo", "run0", "pkexec"] {
        if command_exists(su) {
            return Ok(su.to_string());
        }
    }
    println!("Privilege escalation tool not found. Enter yours (e.g. sudo, doas)");
    Ok(user_input()?.trim().to_string())
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

pub trait PathExt {
    fn read_to_string(&self) -> Result<String, crate::IoErr>;
}

impl PathExt for std::path::Path {
    fn read_to_string(&self) -> Result<String, crate::IoErr> {
        Ok(std::fs::read_to_string(self).map_err(|e| crate::ConfigIoErr::read(self, e))?)
    }
}

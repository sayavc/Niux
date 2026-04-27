use std::process;
use crate::error;
use tempfile::NamedTempFile;
use crate::structures::{ NiuxConfig };
use crate::utils::get_privilege_type;
pub fn run_bash_interactive(args: &[&str]) -> anyhow::Result<()> {
    let first = if args[0] == "sudo" { NiuxConfig::get()?.security.su_type }
    else { args[0].to_string()};
    process::Command::new(first)
        .args(&args[1..])
        .env("PATH", std::env::var("PATH").unwrap_or_default())
        .status()?;
    Ok(())
}
fn bash(args: &[&str], type_bash: bool) -> anyhow::Result<String> {
    let first = if type_bash {
        if args[0] == "sudo" { NiuxConfig::get()?.security.su_type }
        else { args[0].to_string() }
    } else {
        if args[0] == "sudo" { get_privilege_type() }
        else { args[0].to_string() }
    };
    let result = process::Command::new(first)
        .args(&args[1..])
        .env("PATH", std::env::var("PATH").unwrap_or_default())
        .output()
        .unwrap_or_else(|e| { error!("{e}"); process::exit(1); });
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
    let tmp = NamedTempFile::new().unwrap();
    std::fs::write(tmp.path(), content).unwrap();
    writer_write(tmp.path().to_str().unwrap(), dest_path)?;
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

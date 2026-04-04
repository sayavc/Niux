use std::process;
use tempfile::NamedTempFile;
use crate::structures::NiuxConfig;
use crate::utils::get_privilege_type;
pub fn exit_eargs() {
    println!("incorrect arguments");
    process::exit(2);
}
fn bash(args: &[&str], type_bash: bool) -> String {
    let first = if type_bash {
        if args[0] == "sudo" { NiuxConfig::get().config_security.su_type }
        else { args[0].to_string() }
    } else {
        if args[0] == "sudo" { get_privilege_type() }
        else { args[0].to_string() }
    };
    let result = process::Command::new(first)
        .args(&args[1..])
        .env("PATH", std::env::var("PATH").unwrap_or_default())
        .output()
        .unwrap_or_else(|e| { eprintln!("Failed: {e}"); process::exit(1); });
    String::from_utf8(result.stdout).unwrap().trim().to_string()
}
pub fn run_bash(args: &[&str]) -> String {
    bash(args, true)
    }

fn run_early_bash(args: &[&str]) -> String {
    bash(args, false)
}
pub fn writer_init(config_path: &str) {
    run_early_bash(&["sudo", "niux-writer", "init", config_path]);
}
pub fn writer_write(tmp_path: &str, dest_path: &str) {
    run_early_bash(&["sudo", "niux-writer", "write", tmp_path, dest_path]);
}
pub fn write_changes_to_config(content: &str, dest_path: &str) {
    let tmp = NamedTempFile::new().unwrap();
    std::fs::write(tmp.path(), content).unwrap();
    writer_write(tmp.path().to_str().unwrap(), dest_path);
}
pub fn user_input() -> String {
    let mut user_input = String::new();
    std::io::stdin()
        .read_line(&mut user_input)
        .unwrap_or_else(|e| { eprintln!("Failed: {e}"); process::exit(1); });
    user_input
}
pub fn get_home_dir() -> std::path::PathBuf {
dirs::home_dir().unwrap_or_else(|| {
    eprintln!("home directory is not exists"); process::exit(1);
})
}

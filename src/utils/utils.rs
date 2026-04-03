use std::process;
use tempfile::NamedTempFile;
use crate::structures::NiuxConfig;
pub fn exit_eargs() {
    println!("incorrect arguments");
    process::exit(0);
}
pub fn run_bash(args: &[&str]) -> String {
    let first = if args[0] == "sudo" { NiuxConfig::get().config_security.su_type }
    else { args[0].to_string() };
    let result = process::Command::new(first)
        .args(&args[1..])
        .env("PATH", std::env::var("PATH").unwrap_or_default())
        .output()
        .unwrap();
        let result = String::from_utf8(result.stdout).unwrap().trim().to_string();
        result
    }
pub fn write_changes_to_config(config_path: &str, content: &str) {
    let tmp = NamedTempFile::new().unwrap();
    std::fs::write(tmp.path(), content).unwrap();
    run_bash(&["sudo", "cp", tmp.path().to_str().unwrap(), config_path]);
}
pub fn user_input() -> String {
    let mut user_input = String::new();
    std::io::stdin()
        .read_line(&mut user_input)
        .unwrap();
    user_input
}
pub fn get_home_dir() -> std::path::PathBuf {
let path = dirs::home_dir()
    .expect("no home dir");
    path
}

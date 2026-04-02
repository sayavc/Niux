use std::process;
use tempfile::NamedTempFile;
use crate::config::get_config_value;
pub fn exit_eargs() {
    println!("incorrect arguments");
    process::exit(0);
}
pub fn rebuild_nixos_config_home() {
    let user = std::env::var("USER").expect("E to give the env USER");
    process::Command::new(get_config_value().config_paths.path_home_manager) 
        .env("USER", &user)
        .env("HOME", format!("/home/{}", &user))
        .args(["switch", "--flake", &format!("/etc/nixos#{}", user)])
        .status()
        .unwrap();
}
pub fn rebuild_nixos_config_system() {
    let hostname = run_bash(&["hostname"]);
    process::Command::new(get_privilege_type())
        .args(["nixos-rebuild", "switch", "--flake", &format!("/etc/nixos#{}", hostname)])
        .status()
        .unwrap();
}
pub fn run_bash(args: &[&str]) -> String {
    let result = process::Command::new(args[0])
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
    run_bash(&["doas", "cp", tmp.path().to_str().unwrap(), config_path]);
}
pub fn user_input() -> String {
    let mut user_input = String::new();
    std::io::stdin()
        .read_line(&mut user_input)
        .unwrap();
    user_input
}
fn privilege_type(nya: &str) -> String {
    let output = process::Command::new("which")
        .arg(nya)
        .env("PATH", std::env::var("PATH").unwrap_or_default())
        .output()
        .unwrap();
    if output.status.success() {
        let path = String::from_utf8_lossy(&output.stdout).trim().to_string();
        path 
    }
    else {
        String::new()
    }
}
fn get_privilege_type() -> String {
    if privilege_type("sudo").is_empty() {
        if privilege_type("doas").is_empty() {
            println!("unknown privilege type");
            std::process::exit(0);
        } else {
            "doas".to_string()
        }
    } else {
        "sudo".to_string()
    }
}

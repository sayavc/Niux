use std::process;
use crate::structures::NiuxConfig;
use crate::utils::run_bash;
use crate::utils::get_privilege_type;
impl NiuxConfig {
    pub fn rebuild_home() {
    let user = std::env::var("USER").expect("E to give the env USER");
    process::Command::new(NiuxConfig::get().config_paths.path_home_manager) 
        .env("USER", &user)
        .env("HOME", format!("/home/{}", &user))
        .args(["switch", "--flake", &format!("/etc/nixos#{}", user)])
        .status()
        .unwrap();
    }
    pub fn rebuild_system() {
    let hostname = run_bash(&["hostname"]);
    process::Command::new(get_privilege_type())
        .args(["nixos-rebuild", "switch", "--flake", &format!("/etc/nixos#{}", hostname)])
        .status()
        .unwrap();
    }
}

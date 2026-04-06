use std::process;
use crate::structures::NiuxConfig;
use crate::utils::{ run_bash, run_bash_interactive, command_exists };
impl NiuxConfig {
    pub fn rebuild_home() -> Result<(), Box<dyn std::error::Error>>  {
    let user = std::env::var("USER").unwrap_or_else(|e| { eprintln!("Failes: {e}"); process::exit(1); });
    if !Self::check_home_manager() {
        return Self::rebuild_system();
    }
    let flake_arg = &format!("{}#{}", NiuxConfig::get().config_paths.path_nix_flake, user);
    let mut args = vec![
        "home-manager",
        "switch"
    ];
    if Self::check_flakes() {
        args.push("--flake");
        args.push(flake_arg);
    }
        run_bash_interactive(&args)?;
        Ok(())
    }
    pub fn rebuild_system() -> Result<(), Box<dyn std::error::Error>>  {
    let hostname = run_bash(&["hostname"]);
    let flake_arg = &format!("{}#{}", NiuxConfig::get().config_paths.path_nix_flake, hostname);
    let mut args = vec!["sudo", "nixos-rebuild", "switch"];
    if Self::check_flakes() {
        args.push("--flake");
        args.push(flake_arg); 
    }
        run_bash_interactive(&args)?;
        Ok(())
    }
    pub fn update()-> Result<(), Box<dyn std::error::Error>> {
        if Self::check_flakes() {
        run_bash_interactive(&["sudo", "nix", "flake", "update", "--flake", &NiuxConfig::get().config_paths.path_nix_flake])?;
        } else {
            run_bash_interactive(&["nix-channel", "update"])?;
        }
        Ok(())
    }
    pub fn update_flake(package: &str) -> Result<(), Box<dyn std::error::Error>>  {
        if !Self::check_flakes() {
            return Err("Flakes are not enabled".into());
        }
        run_bash_interactive(&["sudo", "nix", "flake", "update", package, "--flake", &NiuxConfig::get().config_paths.path_nix_flake])?;
        Ok(())
    }
    pub fn clear() -> Result<(), Box<dyn std::error::Error>>  {
        run_bash_interactive(&["nix-collect-garbage"])?;
        Ok(())
    }
    fn check_flakes() -> bool {
        std::path::Path::new(&NiuxConfig::get().config_paths.path_nix_flake).join("flake.nix").exists()
    }
    fn check_home_manager() -> bool {
        command_exists("home-manager")
    }
}


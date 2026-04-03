use std::fs;
use crate::structures::AutoGenNiuxConfig;
use std::process;
use crate::utils::{run_bash, get_privilege_type, user_input };
use crate::structures::{ NiuxConfig };

impl NiuxConfig {
    pub fn create() {
        let cfg = AutoGenNiuxConfig::get().unwrap_or_else(|| {
            println!("Failed to get config path");
            process::exit(1);
        }); 
        if cfg.config_path.exists() { 
            println!("Config is exists, rewrite? y/n");
            if user_input().trim() == "n" { process::exit(0); }
        }  
        let home_manager_path = run_bash(&["which", "home-manager"]);
        let default_config = format!(include_str!("../assets/default_config.kdl"), home_manager_path, get_privilege_type());
        fs::write(cfg.config_path, default_config).unwrap();
        process::exit(0);
    }
    pub fn get() -> NiuxConfig {
        let cfg = AutoGenNiuxConfig::get().unwrap_or_else(|| {
            println!("Failed to get config path");
            process::exit(1);
        });
        let content = fs::read_to_string(cfg.config_path).unwrap_or_else(|e| {
            println!("Failed: {e}");
            process::exit(1);
        });
        knuffel::parse::<NiuxConfig>("config.kdl", &content).unwrap_or_else(|e| {
            println!("Failed: {e}");
            process::exit(1);
        })
    }
}

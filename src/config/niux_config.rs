use std::fs;
use crate::error;
use colored::Colorize;
use crate::structures::AutoGenNiuxConfig;
use std::process;
use crate::utils::{get_privilege_type, user_input, writer_write};
#[allow(unused_imports)]
use crate::structures::{ NiuxConfig, Commands };

impl NiuxConfig {
    pub fn create() -> Result<(), Box<dyn std::error::Error>>  {
        let cfg = AutoGenNiuxConfig::get().ok_or("Failed to get config path")?; 
        if cfg.config_path.exists() { 
            println!("{}", "Niux config is exists, rewrite? y/n".blue());
            if user_input().trim() != "y" { return Ok(()); }
        }  
        let commands = NiuxConfig::autodetect();
        let default_config = format!(include_str!("../assets/default_config.kdl"), get_privilege_type(), commands.rebuild_home, commands.rebuild_system, commands.update_flakes);
        let tmp = tempfile::NamedTempFile::new()?;
        fs::write(tmp.path(), default_config)?;
        println!("Config created in {} please edit it", cfg.config_path.to_string_lossy().green());
        writer_write(tmp.path().to_str().unwrap(), cfg.config_path.to_str().unwrap());  
        Ok(())
    }
    pub fn get() -> NiuxConfig {
        let cfg = AutoGenNiuxConfig::get().unwrap_or_else(|| {
            error!("{}", "Failed to get config path");
            process::exit(1);
        });
        let content = fs::read_to_string(cfg.config_path).unwrap_or_else(|e| {
            error!("{e}");
            process::exit(1);
        });
        knuffel::parse::<NiuxConfig>("config.kdl", &content).unwrap_or_else(|e| {
            error!("{e}");
            process::exit(1);
        })
    }
}

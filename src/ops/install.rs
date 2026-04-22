use std::fs;
use colored::Colorize;
use crate::error;
use crate::utils::{ write_changes_to_config, nvd_integration::nvd };
use crate::error;
use crate::structures::{ Package, HookEvent, hook_config::HookConfig, NiuxConfig };
impl Package {
    pub fn install(&self) -> Result<(), Box<dyn std::error::Error>>  {
        HookConfig::run(HookEvent::PreInstall)?;
        let config = NiuxConfig::get();
        let config_path =  if self.is_system { config.config_paths.config_path_system } else { config.config_paths.config_path_home };
        if !std::path::Path::new(&config_path).exists() {
            error!("{}", "Config path is wrong");
            return Ok(())
        }
        let config_marker = if self.is_system { config.config_markers.marker_system } else { config.config_markers.marker_home };
        let content = fs::read_to_string(&config_path)?;
        let mut lines: Vec<String> = content.lines().map(String::from).collect();
        for i in 0..lines.len() {
            if lines[i].contains(&config_marker) {
                let indent = lines[i + 1].len() - lines[i + 1].trim_start().len();
                for name in self.name.iter().rev() {
                    let padded = format!("{}{}", " ".repeat(indent), name);
                    lines.insert(i + 1, padded);
                }
                break;
            }
        }
        let new_content = lines.join("\n");
        write_changes_to_config(&new_content, &config_path);
        if new_content == content {
            println!("{}", "Nothing has changed...".yellow());
            return Ok(());
        }
        println!("{}", "Package added to config".green());
        HookConfig::run(HookEvent::PostInstall)?;
        match (self.rebuild, self.is_system) {
            (true, false) => NiuxConfig::rebuild_home()?,
            (true, true) => NiuxConfig::rebuild_system()?,
            _ => return Ok(()),
        }
        nvd()?;
        Ok(())
    }
}

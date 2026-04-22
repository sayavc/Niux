use crate::structures::{ Package, NiuxConfig, HookEvent, hook_config::HookConfig };
use crate::utils::{ write_changes_to_config, nvd_integration::nvd };
use crate::error;
use colored::Colorize;
use std::fs;
impl Package {
    pub fn remove(&self) -> Result<(), Box<dyn std::error::Error>>  {
        HookConfig::run(HookEvent::PreRemove)?;
        let config = NiuxConfig::get();
        let config_path =  if self.is_system { config.config_paths.config_path_system } else { config.config_paths.config_path_home };
        if !std::path::Path::new(&config_path).exists() {
            error!("{}", "Config path is wrong");
            return Ok(())
        }
        let config_marker = if self.is_system { config.config_markers.marker_system } else { config.config_markers.marker_home };
        let config_marker_end = if self.is_system { config.config_markers.marker_system_end } else { config.config_markers.marker_home_end };
        let content = fs::read_to_string(&config_path)?; 

        let mut lines: Vec<String> = content.lines().map(String::from).collect();
        for i in 0..lines.len() {
            if lines[i].contains(&config_marker) {
                for c in 0..lines.len() {
                    if lines[c].contains(&config_marker_end) {
                        let mut indices_to_remove: Vec<usize> = Vec::new();
                        for (j, line) in lines[i..=c].iter().enumerate() {
                            if self.name.iter().any(|n| line.contains(n.as_str())) {
                                indices_to_remove.push(i + j);
                            }
                        }
                        indices_to_remove.sort_unstable_by(|a, b| b.cmp(a));
                        for idx in indices_to_remove {
                            lines.remove(idx);
                        }
                        break;
                    }
                }
                break;
            }
        }
        let new_content = lines.join("\n");
         if new_content == content {
             println!("{}", "Package not found in config".yellow());
            return Ok(())
        }
        write_changes_to_config(&new_content, &config_path);
        println!("{}", "Package removed with config".green());
        HookConfig::run(HookEvent::PostRemove)?;
        match (self.rebuild, self.is_system) {
        (true, false) => NiuxConfig::rebuild_home()?,
        (true, true) => NiuxConfig::rebuild_system()?,
        _ => return Ok(()),
    }
        nvd()?;
        Ok(())
    }
}

use colored::Colorize;
use anyhow::{ Context, bail };
use crate::structures::{ Package, NiuxConfig, HookEvent, hook_config::HookConfig };
use crate::error;
use crate::utils::{ write_changes_to_config };
use std::fs;
impl Package {
    pub fn remove(&self) -> anyhow::Result<()>  {
        HookConfig::run(HookEvent::PreRemove)?;
        let config = NiuxConfig::get()?;
        let config_path =  if self.is_system { config.config_paths.config_path_system } else { config.config_paths.config_path_home };
        if !std::path::Path::new(&config_path).exists() {
            error!("{}", "Config path is wrong");
            return Ok(())
        }
        let config_marker = if self.is_system { config.config_markers.marker_system } else { config.config_markers.marker_home };
        let config_marker_end = if self.is_system { config.config_markers.marker_system_end } else { config.config_markers.marker_home_end };
        let content = fs::read_to_string(&config_path).with_context(|| format!("Failed to read config: {config_path}"))?; 

        let mut lines: Vec<String> = content.lines().map(String::from).collect();
            let Some(marker_start) = lines.iter().position(|l| l.contains(&config_marker)) else {
                bail!("Marker is not found: {config_marker}");
            };
            let Some(marker_end) = lines.iter().position(|l| l.contains(&config_marker_end)) else {
                bail!("Marker is not found: {config_marker_end}");
            };
            if marker_start > marker_end {
                bail!("Marker end goes earlier marker home, please move your packages in separate config or use custom markers");
            }
            let mut indices_to_remove: Vec<usize> = lines[marker_start..=marker_end]
                .iter()
                .enumerate()
                .filter(|(_, line)| self.name.iter().any(|n| line.contains(n.as_str())))
                .map(|(j, _)| marker_start + j)
                .collect();
            indices_to_remove.sort_unstable_by(|a, b| b.cmp(a));
            for idx in indices_to_remove {
                lines.remove(idx);
            }
        let new_content = lines.join("\n");
         if new_content == content {
             println!("{}", "Package not found in config".yellow());
            return Ok(())
        }
        write_changes_to_config(&new_content, &config_path)?;
        println!("{}", "Package removed with config".green());
        HookConfig::run(HookEvent::PostRemove)?;
        match (self.rebuild, self.is_system) {
        (true, false) => NiuxConfig::rebuild_home(self)?,
        (true, true) => NiuxConfig::rebuild_system(self)?,
        _ => return Ok(()),
    }
        Ok(())
    }
}

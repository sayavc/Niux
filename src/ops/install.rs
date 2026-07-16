use crate::error;
use crate::structures::NiuxConfig;
use crate::structures::models::{Package, Target};
use crate::utils::write_changes_to_config;
use anyhow::{Context, bail};
use colored::Colorize;
use std::fs;
impl Package {
    pub fn install(&self) -> anyhow::Result<()> {
        log::info!(
            "Install is started, rebuild: {}, ptype: {:?}, package: {:?}",
            self.rebuild,
            self.ptype,
            self.name
        );
        let config = NiuxConfig::get();
        let config_path = match self.ptype {
            Target::Home => &config.config_paths.config_path_home,
            Target::System => &config.config_paths.config_path_system,
            _ => unreachable!(),
        };
        if !std::path::Path::new(&config_path).exists() {
            error!("Config path is wrong");
            return Ok(());
        }
        let config_marker = match self.ptype {
            Target::Home => &config.config_markers.marker_home,
            Target::System => &config.config_markers.marker_system,
            _ => unreachable!(),
        };
        let content = fs::read_to_string(config_path)
            .with_context(|| format!("Failed to read config: {}", config_path.display()))?;
        let mut lines: Vec<String> = content.lines().map(String::from).collect();
        for i in 0..lines.len() {
            if lines[i].contains(config_marker) {
                let Some(marker_pos) = lines.iter().position(|l| l.contains(config_marker)) else {
                    bail!("Marker is not found");
                };
                let indent = lines[marker_pos + 1].len() - lines[marker_pos + 1].trim_start().len();
                for name in self.name.iter().rev() {
                    lines.insert(marker_pos + 1, format!("{}{}", " ".repeat(indent), name));
                }
                break;
            }
        }
        let new_content = lines.join("\n");
        if new_content == content {
            println!("{}", "Nothing has changed...".yellow());
            return Ok(());
        }
        write_changes_to_config(&new_content, config_path.to_path_buf())?;
        println!("{}", "Packages removed".green());
        Ok(())
    }
}

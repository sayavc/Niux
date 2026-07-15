use colored::Colorize;
use anyhow::{ 
    Context, 
    bail 
};
use std::fs;
use crate::error;
use crate::utils::{
    write_changes_to_config,
    GetCfgData,
};
use crate::structures::{ 
    Package,
    PackageType,
    NiuxConfig 
};
impl Package {
    pub fn install(&self) -> anyhow::Result<()> {
        log::info!("Install is started, rebuild: {}, ptype: {}, package: {:?}", self.rebuild, match self.ptype { PackageType::System => "System", _ => "Home"}, self.name);
        let config = NiuxConfig::get();
        let config_path = self.ptype.get_config_path(&config.config_paths);
        if !std::path::Path::new(&config_path).exists() {
            error!("Config path is wrong");
            return Ok(())
        }
        let config_marker = self.ptype.get_marker_start(&config.config_markers);
        let content = fs::read_to_string(config_path).with_context(|| format!("Failed to read config: {config_path}"))?;
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
        write_changes_to_config(&new_content, config_path)?;
        println!("{}", "Package added to config".green());
        Ok(())
    }
}

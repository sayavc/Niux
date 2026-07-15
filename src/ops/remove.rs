use colored::Colorize;
use anyhow::{ 
    Context, 
    bail 
};
use crate::structures::{ 
    Package,
    PackageType,
    NiuxConfig,
};
use crate::error;
use crate::utils::{
    write_changes_to_config,
    GetCfgData,
};
use std::fs;
impl Package {
    pub fn remove(&self) -> anyhow::Result<()>  {
        log::info!("Remove is started, rebuild: {}, is_system: {}, package: {:?}", self.rebuild, match self.ptype { PackageType::System => "system", _ => "home"}, self.name);
        let config = NiuxConfig::get();
        let config_path = self.ptype.get_config_path(&config.config_paths);
        if !std::path::Path::new(&config_path).exists() {
            error!("{}", "Config path is wrong");
            return Ok(())
        }

        let (config_marker, config_marker_end) = self.ptype.get_markers(&config.config_markers);

        let content = fs::read_to_string(config_path).with_context(|| format!("Failed to read config: {config_path}"))?; 

        let mut lines: Vec<String> = content.lines().map(String::from).collect();
            let Some(marker_start) = lines.iter().position(|l| l.contains(config_marker)) else {
                bail!("Marker is not found: {config_marker}");
            };
            let Some(marker_end) = lines.iter().position(|l| l.contains(config_marker_end)) else {
                bail!("Marker is not found: {config_marker_end}");
            };
            if marker_start > marker_end {
                bail!("Marker end comes before the home marker. Please move your packages to a separate config or use custom markers");
            }
            let mut indices_to_remove: Vec<usize> = lines[marker_start..=marker_end]
                .iter()
                .enumerate()
                .filter(|(_, line)| self.name.iter().any(|n| line.trim() == n.as_str()))
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
        write_changes_to_config(&new_content, config_path)?;
        println!("{}", "Package removed with config".green());
        Ok(())
    }
}

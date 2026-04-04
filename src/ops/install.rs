use std::fs;
use crate::utils::write_changes_to_config;
use crate::structures::NiuxConfig;
use crate::structures::{ Package };
impl Package {
    pub fn install(&self) {
    let config = NiuxConfig::get();
    let config_path =  if self.is_system { config.config_paths.config_path_system } else { config.config_paths.config_path_home };
    let config_marker = if self.is_system { config.config_markers.marker_system } else { config.config_markers.marker_home };

    let content = fs::read_to_string(&config_path).unwrap_or_else(|e| {
        println!("Failed {e}");
        std::process::exit(1);
    });
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
    println!("package add to config");
    match (self.rebuild, self.is_system) {
        (true, false) => NiuxConfig::rebuild_home(),
        (true, true) => NiuxConfig::rebuild_system(),
        _ => return,
    }
    }
}

use crate::structures::{ Package, NiuxConfig };
use std::fs;
use colored::Colorize;
impl Package {
    pub fn list_all() -> anyhow::Result<()> {
        let config = NiuxConfig::get()?;
        let content_home = fs::read_to_string(config.config_paths.config_path_home)?;
        let content_system = fs::read_to_string(config.config_paths.config_path_system)?;
        let lines_cut_home = Self::search_range(&content_home.lines().map(String::from).collect(), false)?;
        let lines_cut_system = Self::search_range(&content_system.lines().map(String::from).collect(), true)?;
        let mut all_lines: Vec<&String> = lines_cut_home.iter().chain(lines_cut_system.iter()).collect();
            all_lines.sort_by(|a, b| a.trim().cmp(b.trim()));
            for line in &all_lines {
                println!("{}", line.trim());
            }
            Ok(())
    }
    pub fn list_type(&self) -> anyhow::Result<()> {
        let config_path = if self.is_system { NiuxConfig::get()?.config_paths.config_path_system } else { NiuxConfig::get()?.config_paths.config_path_home };
                let content = fs::read_to_string(config_path)?;
                let mut lines = Self::search_range(&content.lines().map(String::from).collect(), self.is_system)?;
            lines.sort();
            for line in &lines {
                println!("{}", line.trim());
            }
            Ok(())
    }

pub fn list_do_package(&self) -> anyhow::Result<()> {
    let config = NiuxConfig::get()?;
    let content_home = fs::read_to_string(config.config_paths.config_path_home)?;
    let content_system = fs::read_to_string(config.config_paths.config_path_system)?;
    let lines_cut_home = Self::search_range(&content_home.lines().map(String::from).collect(), false)?;
    let lines_cut_system = Self::search_range(&content_system.lines().map(String::from).collect(), true)?;
    for name in &self.name {
        let found_home = lines_cut_home.iter().any(|l| l.trim() == name.as_str());
        let found_system = lines_cut_system.iter().any(|l| l.trim() == name.as_str());
        match (found_home, found_system) {
            (true, false) => println!("{}: home", name.blue()),
            (false, true) => println!("{}: system", name.blue()),
            (true, true) => println!("{}: home & system", name.blue()),
            (false, false) => println!("{}: not found", name.blue()),
        }
    } 
    Ok(())
}
pub fn list_do_package_type(&self) -> anyhow::Result<()> {
    let config = NiuxConfig::get()?;
    let config_path = if self.is_system { config.config_paths.config_path_system } else { config.config_paths.config_path_home };
    let content = fs::read_to_string(config_path)?;
    let lines = Self::search_range(&content.lines().map(String::from).collect(), self.is_system)?;
    for name in &self.name {
        let found = lines.iter().any(|l| l.trim() == name.as_str());
        if found {
            println!("{}", name.blue());
        } else {
            println!("{}: not found", name.blue());
        }
    }
    Ok(())
}

#[allow(clippy::ptr_arg)]
fn search_range(lines: &Vec<String>, marker: bool) -> anyhow::Result<Vec<String>> {
    let config = NiuxConfig::get()?;
    let config_marker = if marker { config.config_markers.marker_system } else { config.config_markers.marker_home };
    let config_marker_end = if marker { config.config_markers.marker_system_end } else { config.config_markers.marker_home_end };
    let mut lines_cut: Vec<String> = Vec::new();
    for i in 0..lines.len() {
        if lines[i].contains(&config_marker) {
            for c in 0..lines.len() {
                if lines[c].contains(&config_marker_end) {
                    lines_cut = lines[i+1..c].to_vec();
                    break;
                }
            }
            break;
        }
    }
    Ok(lines_cut)
}
}

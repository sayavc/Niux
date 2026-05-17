use crate::structures::{ Package, NiuxConfig };
use crate::utils::search_range;
use std::fs;
use colored::Colorize;
impl Package {
    pub fn list_all() -> anyhow::Result<()> {
        let config = NiuxConfig::get()?;
        let content_home = fs::read_to_string(config.config_paths.config_path_home)?;
        let content_system = fs::read_to_string(config.config_paths.config_path_system)?;
        let lines_cut_home = search_range(&content_home.lines().map(String::from).collect(), false)?;
        let lines_cut_system = search_range(&content_system.lines().map(String::from).collect(), true)?;
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
                let mut lines = search_range(&content.lines().map(String::from).collect(), self.is_system)?;
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
    let lines_cut_home = search_range(&content_home.lines().map(String::from).collect(), false)?;
    let lines_cut_system = search_range(&content_system.lines().map(String::from).collect(), true)?;
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
}

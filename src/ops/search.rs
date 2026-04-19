use crate::structures::{ Package, HookEvent, hook_config::HookConfig };
use crate::utils::{ run_bash, command_exists };
impl Package { 
    pub fn search(&self) -> Result<(), Box<dyn std::error::Error>>  {
        HookConfig::run(HookEvent::PreSearch)?;
        if !command_exists("nix-search") {
            return Err("nix-search is not installed".into());
        } 
        if self.name.is_empty() {
            return Ok(())
        }
        println!("{}", run_bash(&["nix-search", &self.name[0]]).lines()
            .filter_map(|line| line.split_whitespace().next())
            .collect::<Vec<_>>().join("\n"));
        HookConfig::run(HookEvent::PostSearch)?;
        Ok(())
    }
}

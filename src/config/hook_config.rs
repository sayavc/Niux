use std::process;
use std::fs;
use crate::structures::{ hook_config::HookConfig, models::HookEvent };
use crate::utils::{writer_write, run_bash_interactive };
impl HookConfig {
    pub fn create() -> Result<(), Box<dyn std::error::Error>> {
        if std::path::Path::new("/etc/niux_hooks.kdl").exists() {
            return Ok(());
        }
        let config = include_str!("../assets/hook_config.kdl");
        let tmp = tempfile::NamedTempFile::new()?;
        fs::write(tmp.path(), config)?;
        writer_write(tmp.path().to_str().unwrap(), "/etc/niux_hooks.kdl");
        Ok(())
    }
    pub fn get() -> HookConfig {
        let cfg = "/etc/niux_hooks.kdl";
        let content = fs::read_to_string(cfg).unwrap_or_else(|e| {
            println!("Failed {e}");
            process::exit(1);
        });
        knuffel::parse::<HookConfig>("niux_hooks.kdl", &content).unwrap_or_else(|e| {
            println!("Failed: {e}");
            process::exit(1); 
        })
    }
    pub fn run(event: HookEvent) -> Result<(), Box<dyn std::error::Error>> {
        let config = HookConfig::get();
        let action = match event {
            HookEvent::PreInstall => "pre-install", 
            HookEvent::PostInstall => "post-install",
            HookEvent::PreRemove => "pre-remove",
            HookEvent::PostRemove => "post-remove",
            HookEvent::PreRebuild => "pre-rebuild",
            HookEvent::PostRebuild => "post-rebuild",
            HookEvent::PreUpdate => "pre-update",
            HookEvent::PostUpdate => "post-update",
            HookEvent::PreList => "pre-list",
            HookEvent::PostList => "post-list",
            HookEvent::PreClear => "pre-clear",
            HookEvent::PostClear => "post-clear",
        }; 
        for hook in &config.actions {
            if hook.action == action {
                run_bash_interactive(&["sh", "-c", &hook.run])?;
            }
        }
        Ok(())
    }
}

use std::process;
use std::fs;
use crate::error;
use colored::Colorize;
use crate::structures::{ hook_config::HookConfig, models::HookEvent, AutoGenNiuxConfig };
use crate::utils::{writer_write, run_bash_interactive, user_input };
impl HookConfig {
    pub fn create() -> Result<(), Box<dyn std::error::Error>> {
        let cfg = AutoGenNiuxConfig::get().ok_or("Failed to get config path")?;
        if std::path::Path::new(&cfg.hooks_config_path).exists() {
            println!("{}", "Hooks config os exists, rewrite? y/n".blue());
            if user_input().trim() != "y" { return Ok(()); }
        } else {
            println!("{}", "Create hook config? y/n".blue());
            if user_input().trim() != "y" { return Ok(()); }
        }
        let config = include_str!("../assets/hook_config.kdl");
        let tmp = tempfile::NamedTempFile::new()?;
        fs::write(tmp.path(), config)?;
        writer_write(tmp.path().to_str().unwrap(), cfg.hooks_config_path.to_str().unwrap());
        println!("Config created in {}", cfg.hooks_config_path.to_str().unwrap().green());
        Ok(())
    }
    pub fn get() -> HookConfig {
        let cfg = "/etc/niux_hooks.kdl";
        let content = fs::read_to_string(cfg).unwrap_or_else(|e| {
            error!("{e}");
            process::exit(1);
        });
        knuffel::parse::<HookConfig>("niux_hooks.kdl", &content).unwrap_or_else(|e| {
            error!("{e}");
            process::exit(1); 
        })
    }
    pub fn run(event: HookEvent) -> Result<(), Box<dyn std::error::Error>> {
        let cfg = AutoGenNiuxConfig::get().ok_or("Fauled to get config path")?;
        if !std::path::Path::new(&cfg.hooks_config_path).exists() {
            return Ok(());
        }
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
            HookEvent::PreSearch => "pre-search",
            HookEvent::PostSearch => "post-search",
        }; 
        for hook in &config.actions {
            if hook.action == action {
                run_bash_interactive(&["sh", "-c", &hook.run])?;
            }
        }
        Ok(())
    }
}

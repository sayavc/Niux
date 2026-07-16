use crate::error;
use crate::structures::models::Just;
use crate::structures::{AutoGenNiuxConfig, hook_config::HookConfig, models::HookEvent};
use crate::utils::{run_bash_interactive, user_input, writer_write};
use anyhow::{Context, bail};
use colored::Colorize;
use std::fs;
use std::sync::OnceLock;
impl HookConfig {
    pub fn create() -> anyhow::Result<()> {
        let cfg = AutoGenNiuxConfig::get();
        if cfg.hooks_config_path.exists() {
            println!("{}", "Hooks config already exists, rewrite? y/n".blue());
            if user_input().trim() != "y" {
                return Ok(());
            }
        } else {
            println!("{}", "Create hook config? y/n".blue());
            if user_input().trim() != "y" {
                return Ok(());
            }
        }
        let config = include_str!("../assets/hook_config.kdl");
        let tmp = tempfile::NamedTempFile::new()?;
        fs::write(tmp.path(), config)?;
        writer_write(
            tmp.path().to_str().context("Invalid tmp path")?,
            cfg.hooks_config_path.clone(),
        )?;
        println!(
            "Config created in {}",
            cfg.hooks_config_path
                .to_str()
                .context("Invalid config path")?
                .green()
        );
        Ok(())
    }
    pub fn load() -> anyhow::Result<Self> {
        let cfg = AutoGenNiuxConfig::get();
        let content = fs::read_to_string(&cfg.hooks_config_path).with_context(|| {
            format!(
                "Failed to read config: {})",
                cfg.hooks_config_path.display()
            )
        })?;
        Ok(
            match knuffel::parse::<HookConfig>("niux_hooks.kdl", &content) {
                Ok(parsed_config) => parsed_config,
                Err(e) => {
                    let mut s = String::new();
                    miette::GraphicalReportHandler::new()
                        .render_report(&mut s, &e)
                        .context("{e}")?;
                    eprintln!("{s}");
                    bail!("Failed to parse hook config");
                }
            },
        )
    }
    pub fn get() -> &'static Self {
        static CONFIG: OnceLock<HookConfig> = OnceLock::new();
        CONFIG.get_or_init(|| {
            Self::load().unwrap_or_else(|e| {
                error!("Failed to init hook config\n{e}");
                std::process::exit(1);
            })
        })
    }
    pub fn run(event: HookEvent) -> anyhow::Result<()> {
        let cfg = AutoGenNiuxConfig::get();
        if !cfg.hooks_config_path.exists() {
            return Ok(());
        }
        let config = HookConfig::get();
        let action = match event {
            HookEvent::PreInstall => "pre-install",
            HookEvent::PostInstall => "post-install",
            HookEvent::PreRemove => "pre-remove",
            HookEvent::PostRemove => "post-remove",
            HookEvent::PreEdit => "pre-edit",
            HookEvent::PostEdit => "post-edit",
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
                run_bash_interactive::<Just>(&["sh", "-c", &hook.run])?;
            }
        }
        Ok(())
    }
}

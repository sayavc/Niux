use crate::structures::models::Just;
use crate::structures::{AutoGenNiuxConfig, hook_config::HookConfig, models::HookEvent};
use crate::utils::{run_bash_interactive, user_input, writer_write};
use colored::Colorize;
use std::fs;
use std::sync::OnceLock;
impl HookConfig {
    pub fn create() -> crate::NiuxResult<()> {
        let cfg = AutoGenNiuxConfig::get();
        if cfg.hooks_config_path.exists() {
            println!("{}", "Hooks config already exists, rewrite? y/n".blue());
            if user_input()?.trim() != "y" {
                return Ok(());
            }
        } else {
            println!("{}", "Create hook config? y/n".blue());
            if user_input()?.trim() != "y" {
                return Ok(());
            }
        }

        let config = include_str!("../assets/hook_config.kdl");
        let tmp = tempfile::NamedTempFile::new()
            .map_err(|e| crate::TmpErr::Create { e })
            .map_err(crate::IoErr::from)?;

        fs::write(tmp.path(), config)
            .map_err(|e| crate::TmpErr::Write { e })
            .map_err(crate::IoErr::from)?;

        writer_write(
            tmp.path().to_str().ok_or(crate::Utf8Err::InvalidUtf8)?,
            cfg.hooks_config_path.clone(),
        )?;

        println!(
            "Config created in {}",
            cfg.hooks_config_path
                .to_str()
                .ok_or(crate::Utf8Err::InvalidUtf8)?
                .green()
        );
        Ok(())
    }
    pub fn load() -> crate::NiuxResult<Self> {
        let cfg = AutoGenNiuxConfig::get();
        let content = fs::read_to_string(&cfg.hooks_config_path)
            .map_err(|e| crate::ConfigIoErr::read(cfg.hooks_config_path.clone(), e))
            .map_err(crate::IoErr::from)?;

        match knuffel::parse::<Self>(
            &cfg.hooks_config_path
                .file_name()
                .unwrap_or_default()
                .to_string_lossy(),
            &content,
        ) {
            Ok(parsed) => Ok(parsed),
            Err(e) => {
                let mut s = String::new();

                miette::GraphicalReportHandler::new()
                    .render_report(&mut s, &e)
                    .unwrap_or_else(|e| panic!("Failed to render diagnostic\nKdl err: {e}"));

                eprintln!("{s}");

                Err(crate::IoErr::from(crate::ConfigIoErr::Parse).into())
            }
        }
    }

    pub fn get() -> &'static Self {
        static CONFIG: OnceLock<HookConfig> = OnceLock::new();
        CONFIG.get_or_init(|| {
            Self::load().unwrap_or_else(|e| {
                let mut s = String::new();

                miette::GraphicalReportHandler::new()
                    .render_report(&mut s, &e)
                    .unwrap_or_else(|e| panic!("Failed to render diagnostic\nKdl err: {e}"));

                eprintln!("{s}");
                std::process::exit(1);
            })
        })
    }
    pub fn run(event: HookEvent) -> crate::NiuxResult<()> {
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

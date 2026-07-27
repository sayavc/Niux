use crate::structures::AutoGenNiuxConfig;
use crate::structures::NiuxConfig;
use crate::utils::{Color, get_privilege_type, user_input, writer_write};
use anyhow::{Context, bail};
use colored::Colorize;
use std::fs;
use std::sync::OnceLock;

impl NiuxConfig {
    pub fn create() -> anyhow::Result<()> {
        let cfg = AutoGenNiuxConfig::get();
        if cfg.config_path.exists() {
            println!("{}", "Niux config already exists, rewrite? y/n".blue());
            if user_input().trim() != "y" {
                return Ok(());
            }
        }
        let commands = NiuxConfig::autodetect()?;
        let default_config = format!(
            include_str!("../assets/default_config.kdl"),
            get_privilege_type(),
            commands.editor,
            commands.rebuild_home,
            commands.rebuild_system,
            commands.update_flake,
            commands.update_inputs
        );

        let tmp = tempfile::NamedTempFile::new()
            .with_context(|| "Failed to create tmp file".to_string())?;
        fs::write(tmp.path(), default_config)?;
        println!(
            "Config created in {}. Please edit it",
            cfg.config_path.to_string_lossy().green()
        );

        writer_write(
            tmp.path().to_str().context("Invalid tmp path")?,
            cfg.config_path.clone(),
        )?;
        Ok(())
    }

    pub fn get() -> &'static Self {
        static CONFIG: OnceLock<NiuxConfig> = OnceLock::new();

        CONFIG.get_or_init(|| {
            Self::load().unwrap_or_else(|e| {
                eprintln!("{e}");
                std::process::exit(1);
            })
        })
    }

    pub fn load() -> anyhow::Result<NiuxConfig> {
        let cfg = AutoGenNiuxConfig::get();
        let content = fs::read_to_string(&cfg.config_path)
            .map_err(|e| anyhow::anyhow!("{e:?}"))
            .with_context(|| {
                format!(
                    "{} {}\n{} {}",
                    "Failed to read config".red(),
                    cfg.config_path.to_string_lossy().red(),
                    "Try".red(),
                    "`niux --gen-config`".cold_white()
                )
            })?;

        Ok(match knuffel::parse::<NiuxConfig>("config.kdl", &content) {
            Ok(parsed_config) => parsed_config,
            Err(e) => {
                let mut s = String::new();
                miette::GraphicalReportHandler::new()
                    .render_report(&mut s, &e)
                    .context("{e}")?;
                eprintln!("{s}");
                bail!(
                    "\n{} {}",
                    "Failed to deserialize config\nTry".red(),
                    "`niux --gen-config`".cold_white()
                );
            }
        })
    }
}

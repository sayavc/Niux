use crate::structures::AutoGenNiuxConfig;
use crate::structures::{NIUX_CONFIG_VERSION, NiuxConfig};
use crate::utils::{Color, NiuxKdlExt, get_privilege_type, user_input, writer_write};
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
                    "{} {}\n {} {}",
                    "Failed to read config:".red(),
                    cfg.config_path.to_string_lossy().red(),
                    "Try".red(),
                    "`niux --gen-config`".cold_white()
                )
            })?;

        let doc = match content.parse::<kdl::KdlDocument>() {
            Ok(d) => d,
            Err(e) => {
                let mut s = String::new();

                miette::GraphicalReportHandler::new()
                    .render_report(&mut s, &e)
                    .with_context(|| "Failed to render diagnostic")?;

                eprintln!("{s}");
                bail!("{}", "Invalid syntax".red());
            }
        };

        let version = doc.get_version()?;

        if version < NIUX_CONFIG_VERSION {
            bail!(
                "{} {}",
                "Your config is old\nTry".red(),
                "`niux --migrate`".cold_white()
            );
        }

        if version > NIUX_CONFIG_VERSION {
            bail!(
                "{}",
                "Your config was created by a newer version of niux".red()
            );
        }

        Ok(match knuffel::parse::<NiuxConfig>("config.kdl", &content) {
            Ok(parsed_config) => parsed_config,
            Err(e) => {
                let mut s = String::new();

                miette5::GraphicalReportHandler::new()
                    .render_report(&mut s, &e)
                    .with_context(|| format!("Failed to render diagnostic\n{e}"))?;

                eprintln!("{}", s.cold_white());
                bail!(
                    "\n{} {}",
                    "Failed to deserialize config\nTry".red(),
                    "`niux --gen-config`".cold_white()
                );
            }
        })
    }
}

use crate::structures::AutoGenNiuxConfig;
use crate::structures::NiuxConfig;
use crate::utils::{get_privilege_type, replace_env::ReplaceEnv, user_input, writer_write};
use colored::Colorize;
use std::fs;
use std::sync::OnceLock;

impl NiuxConfig {
    pub fn create() -> crate::NiuxResult<()> {
        let cfg = AutoGenNiuxConfig::get();
        if cfg.config_path.exists() {
            println!("{}", "Niux config already exists, rewrite? y/n".blue());
            if user_input()?.trim() != "y" {
                return Ok(());
            }
        }
        let commands = NiuxConfig::autodetect()?;
        let default_config = format!(
            include_str!("../assets/default_config.kdl"),
            get_privilege_type()?,
            commands.editor,
            commands.rebuild_home,
            commands.rebuild_system,
            commands.update_flake,
            commands.update_inputs
        );

        let tmp = tempfile::NamedTempFile::new()
            .map_err(|e| crate::TmpErr::Create { e })
            .map_err(crate::IoErr::from)?;

        fs::write(tmp.path(), default_config)
            .map_err(|e| crate::TmpErr::Write { e })
            .map_err(crate::IoErr::from)?;

        println!(
            "Config created in {}. Please edit it",
            cfg.config_path.to_string_lossy().green()
        );

        writer_write(
            tmp.path().to_str().ok_or(crate::Utf8Err::InvalidUtf8)?,
            cfg.config_path.clone(),
        )?;
        Ok(())
    }

    pub fn get() -> &'static Self {
        static CONFIG: OnceLock<NiuxConfig> = OnceLock::new();

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

    pub fn load() -> crate::NiuxResult<NiuxConfig> {
        let cfg = AutoGenNiuxConfig::get();
        let content = fs::read_to_string(&cfg.config_path)
            .map_err(|e| crate::ConfigIoErr::read(cfg.config_path.clone(), e))
            .map_err(crate::IoErr::from)?;

        match knuffel::parse::<Self>(
            &cfg.config_path
                .file_name()
                .unwrap_or_default()
                .to_string_lossy(),
            &content,
        ) {
            Ok(mut parsed) => {
                parsed.replace_env();
                Ok(parsed)
            }
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
}

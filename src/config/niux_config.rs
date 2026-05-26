use std::fs;
use anyhow::{ Context, bail};
use colored::Colorize;
use crate::structures::AutoGenNiuxConfig;
use crate::utils::{get_privilege_type, user_input, writer_write};
use crate::structures::NiuxConfig;

impl NiuxConfig {
    pub fn create() -> anyhow::Result<()>  {
        let cfg = AutoGenNiuxConfig::get()?; 
        if cfg.config_path.exists() { 
            println!("{}", "Niux config is exists, rewrite? y/n".blue());
            if user_input().trim() != "y" { return Ok(()); }
        }  
        let commands = NiuxConfig::autodetect()?;
        let default_config = format!(include_str!("../assets/default_config.kdl"), get_privilege_type()?, commands.editor, commands.rebuild_home, commands.rebuild_system, commands.update_flakes);
        let tmp = tempfile::NamedTempFile::new().with_context(|| "Failed to create tmp file".to_string())?;
        fs::write(tmp.path(), default_config)?;
        println!("Config created in {} please edit it", cfg.config_path.to_string_lossy().green());
        writer_write(tmp.path().to_str().context("Invalid tmp path")?, cfg.config_path.to_str().context("Invalid config path")?)?;  
        Ok(())
    }
    pub fn get() -> anyhow::Result<NiuxConfig> {
        let cfg = AutoGenNiuxConfig::get()?;
        let content = fs::read_to_string(&cfg.config_path).map_err(|e| anyhow::anyhow!("{e:?}")).with_context(|| format!("Failed to read config: {}", cfg.config_path.display()))?;
        Ok(match knuffel::parse::<NiuxConfig>("config.kdl", &content) {
            Ok(parsed_config) => parsed_config,
            Err(e) => {
                let mut s = String::new();
                miette::GraphicalReportHandler::new()
                    .render_report(&mut s, &e)
                    .context("{e}")?;
                eprintln!("{s}");
                bail!("Failed to parse config");
            }
        })
    }
}

use crate::structures::{
    Args, AutoGenNiuxConfig, NIUX_CONFIG_VERSION, NiuxConfig,
    hook_config::HookConfig,
    models::{ConfigPath, HooksPath, Package},
};
use crate::utils::{Color, NiuxKdlExt, write_changes_to_config};
use anyhow::{Context, bail};
use colored::Colorize;
use kdl::KdlDocument;
impl Package {
    pub fn show_path() {
        let cfg = AutoGenNiuxConfig::get();
        println!(
            "Config:{}\nHook config:{}",
            cfg.config_path.to_string_lossy().blue(),
            cfg.hooks_config_path.to_string_lossy().blue()
        );
    }
    pub fn gen_config() -> anyhow::Result<()> {
        AutoGenNiuxConfig::init()?;
        NiuxConfig::create()?;
        HookConfig::create()?;
        Ok(())
    }
    pub fn set_config_path(args: &Args) -> anyhow::Result<()> {
        if let Some(path) = &args.config {
            AutoGenNiuxConfig::create::<ConfigPath>(path.to_path_buf())?;
        }
        Ok(())
    }
    pub fn set_hook_config_path(args: &Args) -> anyhow::Result<()> {
        if let Some(path) = &args.hook_config {
            AutoGenNiuxConfig::create::<HooksPath>(path.to_path_buf())?;
        }
        Ok(())
    }
    pub fn migrate() -> anyhow::Result<()> {
        let cfg = AutoGenNiuxConfig::get();
        let content = std::fs::read_to_string(&cfg.config_path).with_context(|| {
            format!(
                "{} {}\n{} {}",
                "Failed to read".red(),
                cfg.config_path.to_string_lossy().red(),
                "Try".red(),
                "`niux --gen-config`".cold_white()
            )
        })?;

        let doc = content.parse::<KdlDocument>().with_context(|| {
            format!(
                "{} {} {}",
                "Failed to parse",
                cfg.config_path.to_string_lossy().red(),
                "to KdlDocument".red()
            )
        })?;

        let version = doc.get_version()?;

        let result = match niux_migrate::migration(doc, version, NIUX_CONFIG_VERSION) {
            Ok(d) => d,
            Err(report) => {
                eprintln!("{report}");
                bail!("Migration failed");
            }
        };
        log::info!("Migrated doc: {result}");
        let config = serde_kdl2::from_doc::<NiuxConfig>(&result)
            .inspect_err(|e| eprintln!("{}", e.to_string().red()))
            .with_context(|| "Failed to deserialize migrate config")?;
        let content = serde_kdl2::to_string_pretty(&config)
            .inspect_err(|e| eprintln!("{}", e.to_string().red()))
            .with_context(|| "Failed to serialize migrate config")?;

        write_changes_to_config(&content, cfg.config_path.to_path_buf())?;
        Ok(())
    }
}

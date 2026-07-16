use crate::{
    error,
    structures::{
        AutoGenNiuxConfig,
        models::{ConfigPath, HooksPath},
    },
    utils::writer_init,
};
use anyhow::Context;
use colored::Colorize;
use std::path::PathBuf;
use std::sync::OnceLock;
pub trait ConfigPathKind {
    fn transform(s: AutoGenNiuxConfig, path: PathBuf) -> AutoGenNiuxConfig;
}
impl ConfigPathKind for ConfigPath {
    fn transform(s: AutoGenNiuxConfig, path: PathBuf) -> AutoGenNiuxConfig {
        AutoGenNiuxConfig {
            config_path: path,
            hooks_config_path: s.hooks_config_path,
        }
    }
}
impl ConfigPathKind for HooksPath {
    fn transform(s: AutoGenNiuxConfig, path: PathBuf) -> AutoGenNiuxConfig {
        AutoGenNiuxConfig {
            config_path: s.config_path,
            hooks_config_path: path,
        }
    }
}
impl Default for AutoGenNiuxConfig {
    fn default() -> Self {
        Self {
            config_path: "/etc/niux.kdl".into(),
            hooks_config_path: "/etc/niux_hooks.kdl".into(),
        }
    }
}
impl AutoGenNiuxConfig {
    pub fn create<T>(path: PathBuf) -> anyhow::Result<()>
    where
        T: ConfigPathKind,
    {
        let s = T::transform(Self::load().unwrap_or_default(), path);
        writer_init(s)?;
        Ok(())
    }
    pub fn init() -> anyhow::Result<()> {
        writer_init(Self::default())?;
        Ok(())
    }
    pub fn load() -> anyhow::Result<AutoGenNiuxConfig> {
        let content = std::fs::read_to_string("/var/lib/niux/niux_autogen.kdl")
            .with_context(|| "Failed to read config: /var/lib/niux/niux_autogen.kdl".red())?;
        knuffel::parse::<Self>("niux_autogen.kdl", &content)
            .with_context(|| "Failed to parse config: /var/lib/niux/niux_autogen.kdl".red())
    }
    pub fn get() -> &'static Self {
        static CONFIG: OnceLock<AutoGenNiuxConfig> = OnceLock::new();
        CONFIG.get_or_init(|| {
            Self::load().unwrap_or_else(|e| {
                error!("Failed to init autogen config, try --gen-config\n{e}");
                std::process::exit(1);
            })
        })
    }
}

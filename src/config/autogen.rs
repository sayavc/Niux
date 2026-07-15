use crate::{
    utils::writer_init,
    structures::{
        AutoGenNiuxConfig,
        models::{
            ConfigPath,
            HooksPath,
        },
    },
};
use anyhow::{ Context };
use std::path::PathBuf;
use std::sync::OnceLock;
pub trait ConfigPathKind {
    fn transform(s: &AutoGenNiuxConfig, path: PathBuf) -> AutoGenNiuxConfig;
}
impl ConfigPathKind for ConfigPath {
    fn transform(s: &AutoGenNiuxConfig, path: PathBuf) -> AutoGenNiuxConfig {
        AutoGenNiuxConfig {
            config_path: path,
            hooks_config_path: s.hooks_config_path.to_path_buf(),
        }
    }
}
impl ConfigPathKind for HooksPath {
    fn transform(s: &AutoGenNiuxConfig, path: PathBuf) -> AutoGenNiuxConfig {
        AutoGenNiuxConfig {
            config_path: s.config_path.to_path_buf(),
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
    fn get_cached() -> &'static Self {
        static INSTANCE: OnceLock<AutoGenNiuxConfig> = OnceLock::new();
        INSTANCE.get_or_init(|| {
            Self::get().unwrap_or_else(|_| Self::default())
        })
    }
    pub fn create<T>(path: PathBuf) -> anyhow::Result<()>
        where 
            T: ConfigPathKind {
                let s = T::transform(Self::get_cached(), path);
                writer_init(s)?;
                Ok(())
    }
    pub fn init() -> anyhow::Result<()> {
        writer_init(Self::default())?;
        Ok(())
    }
    pub fn get() -> anyhow::Result<Self> {
        let content = std::fs::read_to_string("/var/lib/niux/niux_autogen.kdl").with_context(|| "Failed to read config: /var/lib/niux/niux_autogen.kdl".to_string())?;
        knuffel::parse::<Self>("niux_autogen.kdl", &content).with_context(|| "Failed to parse config: /var/lib/niux/niux_autogen.kdl".to_string())
    }
}

use crate::structures::AutoGenNiuxConfig;
use anyhow::{ Context };
use std::path::PathBuf;
use crate::utils::{ writer_init };
impl AutoGenNiuxConfig {
    pub fn create(path: Option<PathBuf>, hooks_path: Option<PathBuf>) -> anyhow::Result<()> {
        let current = AutoGenNiuxConfig::get().unwrap_or_else(|_| AutoGenNiuxConfig {
            config_path: PathBuf::from("/etc/niux.kdl"),
            hooks_config_path: PathBuf::from("/etc/niux_hooks.kdl"), 
        });
        let path = path.unwrap_or(current.config_path);
        let hooks_path = hooks_path.unwrap_or(current.hooks_config_path);
        writer_init(path.to_str().context("Invalid config path")?, hooks_path.to_str().context("Invalid hook path")?)?;
        Ok(())
    }
    pub fn get() -> anyhow::Result<AutoGenNiuxConfig> {
        let content = std::fs::read_to_string("/var/lib/niux/niux_autogen.kdl").with_context(|| "Failed to read config: /var/lib/niux/niux_autogen.kdl".to_string())?;
        knuffel::parse::<AutoGenNiuxConfig>("niux_autogen.kdl", &content).with_context(|| "Failed to parse config: /var/lib/niux_autogen.kdl".to_string())
    }

}

use crate::structures::AutoGenNiuxConfig;
use anyhow::{ Context };
use crate::utils::{ writer_init };
use std::path::PathBuf;
impl AutoGenNiuxConfig {
    pub fn create(path: Option<std::path::PathBuf>, hooks_path: Option<std::path::PathBuf>) -> anyhow::Result<()> {
        let current = AutoGenNiuxConfig::get().unwrap_or(AutoGenNiuxConfig {
            config_path: PathBuf::from("/etc/niux.kdl"),
            hooks_config_path: PathBuf::from("/etc/niux_hooks.kdl"), 
        });
        let path = path.unwrap_or(current.config_path);
        let hooks_path = hooks_path.unwrap_or(current.hooks_config_path);
        writer_init(path.to_str().unwrap(), hooks_path.to_str().unwrap())?;
        Ok(())
    }
    pub fn get() -> anyhow::Result<AutoGenNiuxConfig> {
        let content = std::fs::read_to_string("/var/lib/niux/niux_autogen.kdl").with_context(|| "Failed to read config: /var/lib/niux/niux_autogen.kdl".to_string())?;
        knuffel::parse::<AutoGenNiuxConfig>("niux_autogen.kdl", &content).with_context(|| "Failed to parse config: /var/lib/niux_autogen.kdl".to_string())
    }

}

use crate::structures::{
    Package,
    AutoGenNiuxConfig,
    NiuxConfig,
    hook_config::HookConfig,
    Args,
};
use colored::Colorize;
impl Package {
    pub fn show_path() {
        match AutoGenNiuxConfig::get().ok() {
            Some(cfg) => println!("Config:{}\nHook config:{}", cfg.config_path.to_string_lossy().blue(), cfg.hooks_config_path.to_string_lossy().blue()),
            None => println!("none"),
        }
    }
    pub fn gen_config(args: &Args) -> anyhow::Result<()> {
        AutoGenNiuxConfig::create(args.config.clone(), args.hook_config.clone())?;
        NiuxConfig::create()?;
        HookConfig::create()?;
        Ok(())
    }
    pub fn set_config_path(args: &Args) -> anyhow::Result<()> {
        if let Some(path) = args.config.clone() {
        AutoGenNiuxConfig::create(AutoGenNiuxConfig::get().ok().map(|c| c.config_path), Some(path))?;
        }
        Ok(())
    }
    pub fn set_hook_config_path(args: &Args) -> anyhow::Result<()> {
        if let Some(path) = args.hook_config.clone() {
        AutoGenNiuxConfig::create(AutoGenNiuxConfig::get().ok().map(|c| c.config_path), Some(path))?;
        }
        Ok(())
    }
}

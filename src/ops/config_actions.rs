use crate::structures::{
    Package,
    AutoGenNiuxConfig,
    NiuxConfig,
    hook_config::HookConfig,
    Args,
    models::{
        HooksPath,
        ConfigPath,
    },
};
use colored::Colorize;
impl Package {
    pub fn show_path() {
        let cfg = AutoGenNiuxConfig::get();
        println!("Config:{}\nHook config:{}", cfg.config_path.to_string_lossy().blue(), cfg.hooks_config_path.to_string_lossy().blue());
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
}

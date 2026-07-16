use crate::{
    structures::models::{HookEvent, Just, Package, Target},
    structures::{NiuxConfig, hook_config::HookConfig},
    utils::run_bash_interactive,
};
use anyhow::Context;
use shell_words;
impl Package {
    pub fn rebuild_home(&self) -> anyhow::Result<()> {
        HookConfig::run(HookEvent::PreRebuild)?;
        let args = shell_words::split(&NiuxConfig::get().commands.rebuild_home)?;
        run_bash_interactive::<Just>(&args.iter().map(String::as_str).collect::<Vec<_>>())?;
        Package::nvd(Target::Home)?;
        HookConfig::run(HookEvent::PostRebuild)?;
        Ok(())
    }
    pub fn rebuild_system(&self) -> anyhow::Result<()> {
        HookConfig::run(HookEvent::PreRebuild)?;
        let args = shell_words::split(&NiuxConfig::get().commands.rebuild_system)?;
        run_bash_interactive::<Just>(&args.iter().map(String::as_str).collect::<Vec<_>>())?;
        Package::nvd(Target::System)?;
        HookConfig::run(HookEvent::PostRebuild)?;
        Ok(())
    }
    pub fn update() -> anyhow::Result<()> {
        let args = shell_words::split(&NiuxConfig::get().commands.update_flakes)?;
        run_bash_interactive::<Just>(&args.iter().map(String::as_str).collect::<Vec<_>>())?;
        Ok(())
    }
    pub fn update_flake(&self) -> anyhow::Result<()> {
        run_bash_interactive::<Just>(&[
            "sudo",
            "nix",
            "flake",
            "update",
            &self.name[0],
            "--flake",
            NiuxConfig::get()
                .config_paths
                .path_nix_flake
                .to_str()
                .with_context(|| "Invalid nix flake path")?,
        ])?;
        Ok(())
    }
    pub fn clear() -> anyhow::Result<()> {
        run_bash_interactive::<Just>(&["nix-collect-garbage"])?;
        Ok(())
    }
}

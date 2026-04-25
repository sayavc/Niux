use shell_words;
use crate::structures::{ NiuxConfig, Package};
use crate::utils::{ run_bash_interactive };
impl NiuxConfig {
    pub fn rebuild_home(package: &Package) -> anyhow::Result<()>  {
        let args = shell_words::split(&Self::get()?.commands.rebuild_home)?;
        run_bash_interactive(&args.iter().map(String::as_str).collect::<Vec<_>>())?;
        Package::nvd(package)?;
        Ok(())
    }
    pub fn rebuild_system(package: &Package) -> anyhow::Result<()>  {
        let args = shell_words::split(&Self::get()?.commands.rebuild_system)?;
        run_bash_interactive(&args.iter().map(String::as_str).collect::<Vec<_>>())?;
        Package::nvd(package)?;
        Ok(())
    }
    pub fn update()-> anyhow::Result<()> {
        let args = shell_words::split(&Self::get()?.commands.update_flakes)?;
        run_bash_interactive(&args.iter().map(String::as_str).collect::<Vec<_>>())?;
        Ok(())
    }
    pub fn update_flake(package: &str) -> anyhow::Result<()>  {
        run_bash_interactive(&["sudo", "nix", "flake", "update", package, "--flake", &NiuxConfig::get()?.config_paths.path_nix_flake])?;
        Ok(())
    }
    pub fn clear() -> anyhow::Result<()>  {
        run_bash_interactive(&["nix-collect-garbage"])?;
        Ok(())
    }
}

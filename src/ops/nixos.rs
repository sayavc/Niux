use crate::{
    structures::models::{HookEvent, Just, Package, Target},
    structures::{NiuxConfig, hook_config::HookConfig},
    utils::run_bash_interactive,
};
impl Package {
    pub fn rebuild_home(&self, extra: &[String]) -> crate::NiuxResult<()> {
        HookConfig::run(HookEvent::PreRebuild)?;
        let mut args = shell_words::split(&NiuxConfig::get().commands.rebuild_home)?;
        args.extend(extra.iter().cloned());

        run_bash_interactive::<Just>(&args.iter().map(String::as_str).collect::<Vec<_>>())?;

        Package::nvd(Target::Home)?;
        HookConfig::run(HookEvent::PostRebuild)?;

        Ok(())
    }
    pub fn rebuild_system(&self, extra: &[String]) -> crate::NiuxResult<()> {
        HookConfig::run(HookEvent::PreRebuild)?;
        let mut args = shell_words::split(&NiuxConfig::get().commands.rebuild_system)?;
        args.extend(extra.iter().cloned());

        run_bash_interactive::<Just>(&args.iter().map(String::as_str).collect::<Vec<_>>())?;

        Package::nvd(Target::System)?;
        HookConfig::run(HookEvent::PostRebuild)?;

        Ok(())
    }
    pub fn update() -> crate::NiuxResult<()> {
        let args = shell_words::split(&NiuxConfig::get().commands.update_flake)?;
        run_bash_interactive::<Just>(&args.iter().map(String::as_str).collect::<Vec<_>>())?;
        Ok(())
    }
    pub fn update_flake(&self) -> crate::NiuxResult<()> {
        let args = shell_words::split(&NiuxConfig::get().commands.update_inputs)?;

        let result: Vec<&str> = args
            .iter()
            .flat_map(|w| {
                if w == "[packages]" {
                    self.name.iter().map(String::as_str).collect::<Vec<_>>()
                } else {
                    vec![w.as_str()]
                }
            })
            .collect();

        run_bash_interactive::<Just>(&result)?;
        Ok(())
    }
}

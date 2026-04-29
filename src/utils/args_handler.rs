use colored::Colorize;
use anyhow::bail;
use crate::structures::{AutoGenNiuxConfig, NiuxConfig, Args, Package, HookEvent, hook_config::HookConfig };
pub enum Target { System, Home, Both, None }
pub enum Action { Install, Remove, Search, None }
impl Args {
    pub fn target(&self) -> Target {
        match (self.system, self.home) {
            (true, false) => Target::System,
            (false, true) => Target::Home,
            (true, true) => Target::Both,
            _ => Target::None,
        }
    }
    pub fn action(&self) -> Action {
        match (self.install, self.remove, self.search) {
            (true, false, false) => Action::Install,
            (false, true, false) => Action::Remove,
            (false, false, true) => Action::Search,
            _ => Action::None,
        }
    }
}
pub fn dispatch(action: &Action, package: &Package) -> anyhow::Result<()>  {
    match action {
        Action::Install => package.install()?,
        Action::Remove => package.remove()?,
        Action::Search => package.search()?,
        Action::None => bail!("No action specified"),
    }
    Ok(())
}
pub fn handle(target: &Target, args: &Args, package: &Package) -> anyhow::Result<bool>  {
    if args.package.is_some() && !args.install && !args.remove && !args.update && !args.list && !args.search { bail!("Invalid arguments") }
    if args.gen_config { AutoGenNiuxConfig::create(args.config.clone(), args.hook_config.clone())?; NiuxConfig::create()?; HookConfig::create()?; return Ok(true); }
    if args.package.is_some() && args.update { 
        HookConfig::run(HookEvent::PreUpdate)?;
        if let Some(packages) = &args.package {
        NiuxConfig::update_flake(&packages[0])?;
        }
        rebuild(target, args, package)?;
        HookConfig::run(HookEvent::PostUpdate)?;
        return Ok(true);
    }
    if let Some(path) = args.config.clone() {
        AutoGenNiuxConfig::create(Some(path), AutoGenNiuxConfig::get().ok().map(|c| c.hooks_config_path))?;
        return Ok(true);
    }
    if let Some(path) = args.hook_config.clone() {
        AutoGenNiuxConfig::create(AutoGenNiuxConfig::get().ok().map(|c| c.config_path), Some(path))?;
        return Ok(true);
    }
    if args.show_path {
        match AutoGenNiuxConfig::get().ok() {
            Some(cfg) => println!("{}\n{}", cfg.config_path.to_string_lossy().blue(), cfg.hooks_config_path.to_string_lossy().blue()),
            None => println!("none"),
        }
        return Ok(true);
    }
    if args.list && !args.home && !args.system && args.package.is_none() {
        HookConfig::run(HookEvent::PreList)?;
            Package::list_all()?;
        HookConfig::run(HookEvent::PostList)?;
        return Ok(true);
    }
    if args.update {
        HookConfig::run(HookEvent::PreUpdate)?;
        NiuxConfig::update()?;
        HookConfig::run(HookEvent::PostUpdate)?;
        rebuild(target, args, package)?; 
        return Ok(true);
    }
    if args.clear {
        HookConfig::run(HookEvent::PreClear)?;
        NiuxConfig::clear()?;
        HookConfig::run(HookEvent::PostClear)?;
        return Ok(true);
    }
    if (args.install || args.remove) && args.package.is_none() {
        bail!("No package specified");
    }
    if (args.install || args.remove) && matches!(target, Target::Both) {
        bail!("Cannot install/remove to both targets simultaneously");
    } 
    Ok(false)
}
pub fn rebuild(target: &Target, args: &Args, package: &Package) -> anyhow::Result<bool> {
    if matches!(args.action(), Action::None) && args.apply {
        HookConfig::run(HookEvent::PreRebuild)?;
        match target {
            Target::System => NiuxConfig::rebuild_system(package)?,
            Target::Home => NiuxConfig::rebuild_home(package)?,
            Target::Both => { NiuxConfig::rebuild_system(&Package { is_system: true, ..(*package).clone() })?; NiuxConfig::rebuild_home(&Package { is_system: false, ..(*package).clone() })?; } 
            Target::None => bail!("No target specified"),
        }
        HookConfig::run(HookEvent::PostRebuild)?;
        return Ok(true);
    }  
    Ok(false)
}
pub fn list(args: &Args, package: &Package) -> anyhow::Result<bool> {
    HookConfig::run(HookEvent::PreList)?;
    if args.list && args.package.is_none() {
        Package::list_type(package)?;
        HookConfig::run(HookEvent::PostList)?;
        return Ok(true); 
    }
    if args.list && args.package.is_some() {
        Package::list_do_package(package)?;
        HookConfig::run(HookEvent::PostList)?;
        return Ok(true);
    }
    Ok(false)
}

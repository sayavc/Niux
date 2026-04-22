use colored::Colorize;
use crate::structures::{AutoGenNiuxConfig, NiuxConfig, Args, Package, HookEvent, hook_config::HookConfig };
use crate::utils::nvd_integration::nvd;
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
pub fn dispatch(action: &Action, package: &Package) -> Result<(), Box<dyn std::error::Error>>  {
    match action {
        Action::Install => package.install()?,
        Action::Remove => package.remove()?,
        Action::Search => package.search()?,
        Action::None => return Err("No action specified".into()),
    }
    Ok(())
}
pub fn handle(target: &Target, args: &Args) -> Result<bool, Box<dyn std::error::Error>> {
    if args.package.is_some() && !args.install && !args.remove && !args.update && !args.list && !args.search { return Err("Invalid arguments".into()) }
    if args.gen_config { AutoGenNiuxConfig::create(args.default_path_config.clone(), args.default_hook_path_config.clone())?; NiuxConfig::create()?; HookConfig::create()?; return Ok(true); }
    if args.package.is_some() && args.update { 
        HookConfig::run(HookEvent::PreUpdate)?;
        NiuxConfig::update_flake(&args.package.as_ref().unwrap()[0])?;
        rebuild(target, args)?;
        HookConfig::run(HookEvent::PostUpdate)?;
        return Ok(true);
    }
    if let Some(path) = args.default_path_config.clone() {
        AutoGenNiuxConfig::create(Some(path), AutoGenNiuxConfig::get().map(|c| c.hooks_config_path))?;
        return Ok(true);
    }
    if let Some(path) = args.default_hook_path_config.clone() {
        AutoGenNiuxConfig::create(AutoGenNiuxConfig::get().map(|c| c.config_path), Some(path))?;
        return Ok(true);
    }
    if args.get_current_path {
        match AutoGenNiuxConfig::get() {
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
        rebuild(target, args)?; 
        return Ok(true);
    }
    if args.clear {
        HookConfig::run(HookEvent::PreClear)?;
        NiuxConfig::clear()?;
        HookConfig::run(HookEvent::PostClear)?;
        return Ok(true);
    }
    if (args.install || args.remove) && args.package.is_none() {
        return Err("No package specified".into());
    }
    if (args.install || args.remove) && matches!(target, Target::Both) {
        return Err("Cannot install/remove to both targets simultaneously".into());
    } 
    Ok(false)
}
pub fn rebuild(target: &Target, args: &Args) -> Result<bool, Box<dyn std::error::Error>> {
    if matches!(args.action(), Action::None) && args.apply {
        HookConfig::run(HookEvent::PreRebuild)?;
        match target {
            Target::System => NiuxConfig::rebuild_system()?,
            Target::Home => NiuxConfig::rebuild_home()?,
            Target::Both => { NiuxConfig::rebuild_system()?; NiuxConfig::rebuild_home()?; } 
            Target::None => return Err("No target specified".into()),
        }
        HookConfig::run(HookEvent::PostRebuild)?;
        nvd()?;
        return Ok(true);
    }  
    Ok(false)
}
pub fn list(args: &Args, package: &Package) -> Result<bool, Box<dyn std::error::Error>> {
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

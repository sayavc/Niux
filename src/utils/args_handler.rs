use colored::Colorize;
use crate::structures::{ Args, Package }; 
use crate::structures::AutoGenNiuxConfig;
use crate::structures::NiuxConfig;
pub enum Target { System, Home, Both, None }
pub enum Action { Install, Remove, None }
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
        match (self.install, self.remove) {
            (true, false) => Action::Install,
            (false, true) => Action::Remove,
            _ => Action::None,
        }
    }
}
pub fn dispatch(action: &Action, package: &Package) -> Result<(), Box<dyn std::error::Error>>  {
    match action {
        Action::Install => package.install()?,
        Action::Remove => package.remove()?,
        Action::None => return Err("No action specified".into()),
    }
    Ok(())
}
pub fn handle(target: &Target, args: &Args) -> Result<bool, Box<dyn std::error::Error>> {
    if args.package.is_some() && !args.install && !args.remove && !args.update && !args.list { return Err("Invalid arguments".into()) }
    if args.gen_config { AutoGenNiuxConfig::create(args.default_path_config.clone())?; NiuxConfig::create()?; return Ok(true); }
    if args.package.is_some() && args.update { 
        NiuxConfig::update_flake(&args.package.as_ref().unwrap()[0])?;
        rebuild(target, args)?;
        return Ok(true);
    }
    if let Some(path) = args.default_path_config.clone() {
        AutoGenNiuxConfig::create(Some(path))?;
        return Ok(true);
    }
    if args.get_currect_path {
        match AutoGenNiuxConfig::get() {
            Some(cfg) => println!("{}", cfg.config_path.to_string_lossy().blue()),
            None => println!("none"),
        }
    }
    if args.list && !args.home && !args.system && args.package.is_none() {
            Package::list_all()?;
        return Ok(true);
    }
    if args.update {
        NiuxConfig::update()?;
        rebuild(target, args)?; 
        return Ok(true);
    }
    if args.clear {
        NiuxConfig::clear()?;
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
        match target {
            Target::System => NiuxConfig::rebuild_system()?,
            Target::Home => NiuxConfig::rebuild_home()?,
            Target::Both => { NiuxConfig::rebuild_system()?; NiuxConfig::rebuild_home()?; } 
            Target::None => return Err("No target specified".into()),
        }
        return Ok(true);
    }  
    Ok(false)
}
pub fn list(args: &Args, package: &Package) -> Result<bool, Box<dyn std::error::Error>> {
    if args.list && args.package.is_none() {
        Package::list_type(package)?;
        return Ok(true); 
    }
    if args.list && args.package.is_some() {
        Package::list_do_package(package)?;
        return Ok(true);
    }
    Ok(false)
}

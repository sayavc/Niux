use std::process;
use crate::ops::*;
use crate::utils::nixos::*;
use crate::structures::{ Args, Package }; 
use crate::utils::exit_eargs;
use crate::structures::AutoGenNiuxConfig;
use crate::structures::NiuxConfig;
pub enum Target { System, Home, Both, None }
pub enum Action { Install, Uninstall, None }
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
        match (self.install, self.uninstall) {
            (true, false) => Action::Install,
            (false, true) => Action::Uninstall,
            _ => Action::None,
        }
    }
}
pub fn dispatch(action: &Action, package: &Package ) {
    match action {
        Action::Install => package.install(),
        Action::Uninstall => package.uninstall(),
        Action::None => exit_eargs(),
    }
}
pub fn handle(target: &Target, args: &Args) -> Result<bool, Box<dyn std::error::Error>> {
    if args.package.is_some() && !args.install && !args.uninstall { return Err("Invalid arguments".into()) }
    if args.gen_config { AutoGenNiuxConfig::create(args.default_path_config.clone())?; NiuxConfig::create()?; return Ok(true); }
    if let Some(path) = args.default_path_config.clone() {
        AutoGenNiuxConfig::create(Some(path))?;
        return Ok(true);
    }
    if matches!(args.action(), Action::None) && args.apply {
        match target {
            Target::System => NiuxConfig::rebuild_system(),
            Target::Home => NiuxConfig::rebuild_home(),
            Target::Both => NiuxConfig::rebuild_system(), 
            Target::None => exit_eargs(),
        }
        return Ok(true);
    }  
    Ok(false)
}

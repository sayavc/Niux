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
pub fn handle(target: &Target, args: &Args) {
    if args.package.is_some() && !args.install && !args.uninstall { exit_eargs() }
    if args.gen_config { NiuxConfig::create() }
    if let Some(path) = args.default_path_config.clone() {
        AutoGenNiuxConfig { config_path: path }.create();
    }
    AutoGenNiuxConfig::exist();
    if matches!(args.action(), Action::None) && args.apply {
        match target {
            Target::System => { NiuxConfig::rebuild_system(); process::exit(0); }
            Target::Home => {NiuxConfig::rebuild_home(); process::exit(0); }
            Target::Both => { NiuxConfig::rebuild_system(); NiuxConfig::rebuild_home(); process::exit(0); }
            Target::None => exit_eargs(),
        }
    }  
}

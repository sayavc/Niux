mod config;
mod utils;
mod ops;
mod structures;
use clap::Parser;
use crate::config::gen_config;
use crate::utils::{rebuild_nixos_config_home, rebuild_nixos_config_system, exit_eargs,};
use crate::structures::{ Package };
use crate::structures::{ Args };


enum Target { System, Home, Both}
enum Action { Install, Uninstall }
fn main() {
    let args = Args::parse();
    if args.gen_config { gen_config(); }
    if !args.install && !args.uninstall { exit_eargs(); }
    if args.home && args.system && !args.rebuild_config { exit_eargs(); }
    if (args.install || args.uninstall) && args.package_name.is_none() { exit_eargs(); }
    let target = match (args.system, args.home) {
        (true, false) => Target::System,
        (false, true) => Target::Home,
        (true, true) => Target::Both,
        _ => unreachable!(),
    };
    let action = match (args.install, args.uninstall) {
        (true, false) => Action::Install,
        (false, true) => Action::Uninstall,
        _ => unreachable!(),
    };
    if args.rebuild_config {
        match target {
            Target::Home => rebuild_nixos_config_home(),
            Target::System => rebuild_nixos_config_system(),
            Target::Both => { rebuild_nixos_config_home(); rebuild_nixos_config_system(); }
        }
    }
    let package = Package {
        name: args.package_name.unwrap_or_else(|| unreachable!()),
        is_system: matches!(target, Target::System),
        rebuild: args.rebuild_config,
    };
    match action {
        Action::Install => package.install(),
        Action::Uninstall => package.uninstall(),
    }
}


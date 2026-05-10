mod config;
mod commands;
mod utils;
mod ops;
mod structures;
use clap::Parser;
use commands::validate;
use structures::{
    Args,
    Package,
    models::{
        Target,
    },
};
fn main() {
    pretty_env_logger::init();
    let args = Args::parse();
    let target = args.target();
    let action = args.action();
    let rebuild = args.rebuild_mode();
    let package = Package {
        name: args.package.clone().unwrap_or_default(),
        is_system: matches!(target, Target::System),
        rebuild: args.apply,
    };
    match validate(&args) {
        Ok(()) => {},
        Err(e) => { error!("{e}"); std::process::exit(1); }
    }
    match action.dispatch_config(&args) {
        Ok(true) => return,
        Ok(false) => {},
        Err(e) => { error!("{e}"); std::process::exit(1); }
    }
    match action.pre_hooks() {
        Ok(()) => {},
        Err(e) => { error!("{e}"); std::process::exit(1); }
    }
    match action.dispatch(&package) {
        Ok(()) => {},
        Err(e) => { error!("{e}"); std::process::exit(1); }
    }
    match rebuild.rebuild_wrapper(&package) {
        Ok(()) => {},
        Err(e) => { error!("{e}"); std::process::exit(1); }
    }
    match action.post_hooks() {
        Ok(()) => {},
        Err(e) => { error!("{e}"); std::process::exit(1); }
    }
}

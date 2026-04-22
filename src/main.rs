mod config;
mod utils;
mod ops;
mod structures;
use clap::Parser;
use structures::{ Args, Package};
use utils::args_handler::*;
fn main() {
    let args = Args::parse();
    let target = args.target();
    let action = args.action();
    let package = Package {
        name: args.package.clone().unwrap_or_default(),
        is_system: matches!(target, Target::System),
        rebuild: args.apply,
    };
    match handle(&target, &args, &package) { 
        Ok(true) => return,
        Ok(false) => {},
        Err(e) => { error!("{e}"); std::process::exit(1); }
    }
    match rebuild(&target, &args, &package) {
        Ok(true) => return,
        Ok(false) => {},
        Err(e) => { error!("{e}"); std::process::exit(1); }
    }
    match list(&args, &package) {
        Ok(true) => return,
        Ok(false) => {},
        Err(e) => { error!("{e}"); std::process::exit(1); }
    }
    match dispatch(&action, &package) {
        Ok(()) => (),
        Err(e) => { error!("{e}"); std::process::exit(1); }
    }
}

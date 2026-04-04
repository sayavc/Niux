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
    match handle(&target, &args) { 
        Ok(true) => return,
        Ok(false) => {},
        Err(e) => { eprintln!("error: {e}"); std::process::exit(1); }
    }
    let package = Package {
        name: args.package.unwrap_or_default(),
        is_system: matches!(target, Target::System),
        rebuild: args.apply, };
    dispatch(&action, &package);
}

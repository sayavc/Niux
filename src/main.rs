mod config;
mod utils;
mod ops;
mod structures;
use clap::{ Parser, CommandFactory };
use structures::{ Args, Package};
use utils::args_handler::*;
fn main() {
    let args = Args::parse();
    if let Some(shell) = args.completions {
        let mut cmd = Args::command();
        clap_complete::generate(shell, &mut cmd, "niux", &mut std::io::stdout());
        return;
    }
    let target = args.target();
    let action = args.action();
    match handle(&target, &args) { 
        Ok(true) => return,
        Ok(false) => {},
        Err(e) => { eprintln!("error: {e}"); std::process::exit(1); }
    }
    match rebuild(&target, &args) {
        Ok(true) => return,
        Ok(false) => {},
        Err(e) => { eprintln!("error: {e}"); std::process::exit(1); }
    }
    let package = Package {
        name: args.package.clone().unwrap_or_default(),
        is_system: matches!(target, Target::System),
        rebuild: args.apply,
    };
    match list(&args, &package) {
        Ok(true) => return,
        Ok(false) => {},
        Err(e) => { eprintln!("error: {e}"); std::process::exit(1); }
    }
    match dispatch(&action, &package) {
        Ok(()) => (),
        Err(e) => { eprintln!("error: {e}"); std::process::exit(1); }
    }
}

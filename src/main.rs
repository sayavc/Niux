mod commands;
mod config;
mod ops;
mod structures;
mod utils;
use clap::Parser;
use commands::validate;
use structures::Args;
use structures::models::Package;
fn main() {
    pretty_env_logger::init();
    if let Err(e) = run() {
        error!("{e}");
        std::process::exit(1);
    }
}
fn run() -> anyhow::Result<()> {
    let args = Args::parse();
    let target = args.target();
    let action = args.action();
    let package = Package {
        name: args.package.clone().unwrap_or_default(),
        ptype: target,
        rebuild: args.apply,
    };
    validate(&args)?;
    if action.dispatch_config(&args)? {
        return Ok(());
    }
    action.pre_hooks()?;
    action.dispatch(&package)?;
    args.rebuild_mode().rebuild_wrapper(&package)?;
    action.post_hooks()?;
    Ok(())
}

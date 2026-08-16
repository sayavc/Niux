mod commands;
mod config;
mod errors;
mod integrations;
mod ops;
mod structures;
mod utils;
use clap::Parser;
use commands::validate;
use structures::Args;
use structures::models::Package;

pub use errors::*;
pub type NiuxResult<T> = std::result::Result<T, NiuxError>;

fn main() {
    pretty_env_logger::init();

    if let Err(e) = run() {
        let mut report = String::new();

        miette::GraphicalReportHandler::new()
            .render_report(&mut report, &e)
            .expect("failed to render diagnostic");

        eprintln!("{report}");
        std::process::exit(1)
    }
}
fn run() -> NiuxResult<()> {
    let args = Args::parse();
    let target = args.target();
    let action = args.action();
    let package = Package {
        name: args.package.clone().unwrap_or_default(),
        ptype: target,
        rebuild: args.apply,
        raw_mode: args.raw,
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

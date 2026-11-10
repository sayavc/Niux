use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(about = "internal tool, used by niux")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}
#[derive(Subcommand)]
pub enum Commands {
    Init {
        config_path: String,
        hook_config_path: String,
    },
    Write {
        tmp_path: String,
        dest_path: String,
    },
}

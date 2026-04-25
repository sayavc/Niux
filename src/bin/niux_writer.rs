use clap::{ Parser, Subcommand };
use std::os::unix::fs::MetadataExt;
use anyhow::{ Context, bail };
use colored::Colorize;
use std::process;
use std::fs;
#[derive(Parser)]
#[command(about = "internal tool, used by niux")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}
#[derive(Subcommand)]
enum Commands {
    Init { config_path: String, hook_config_path: String },
    Write { tmp_path: String, dest_path: String },
}
fn main() {
let args = Cli::parse();
match args.command {
    Commands::Init { config_path, hook_config_path } => {
        if let Err(e) = create_autogen(&config_path, &hook_config_path) {
            eprintln!("{}: {}", "Failed".red(), e.to_string().red());
            process::exit(1);
        }
    }
    Commands::Write { tmp_path, dest_path } => {
        if let Err(e) = writer(&tmp_path, &dest_path) {
            eprintln!("{}: {}", "Failed".red(), e.to_string().red());
            process::exit(1);
        }
    }
}
}
fn create_autogen(config_path: &str, hook_config_path: &str) -> anyhow::Result<()> {
    let config_dir = std::path::Path::new("/var/lib/niux/");
    fs::create_dir_all(config_dir).with_context(|| format!("Failed to create dir: {}", config_dir.display()))?;
    let content = format!(include_str!("../assets/autogen_config.kdl"), config_path, hook_config_path);
    let path_to_write = config_dir.join("niux_autogen.kdl");
    fs::write(&path_to_write, content).with_context(|| format!("Failed to write config dir: {}", path_to_write.display()))?;
    Ok(())
}
fn writer(tmp_path: &str, dest_path: &str) -> anyhow::Result<()> {
    let tmp_metadata = std::fs::metadata(tmp_path).with_context(|| format!("failed to write dir metadata: {tmp_path}"))?;
    if tmp_metadata.uid() == 0 {
        bail!("tmp_path must not be owned by root");
    }
    if std::path::Path::new(dest_path).exists() {
        let metadata = std::fs::symlink_metadata(dest_path).with_context(|| format!("failed to read metadata: {dest_path}"))?;
        if metadata.file_type().is_symlink() {
            let real_path = std::fs::read_link(dest_path)?;
            let real_metadata = std::fs::metadata(&real_path)?;
            let file_uid = real_metadata.uid();
            let current_uid = unsafe { libc::getuid() };
            if file_uid != current_uid {
                bail!("Symlink points to file owned by another user");
            }
        }
    }
    let tmp_content = fs::read_to_string(tmp_path)?;
    fs::write(dest_path, tmp_content)?;
    Ok(())
}

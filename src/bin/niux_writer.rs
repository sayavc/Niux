use clap::{ Parser, Subcommand };
use std::os::unix::fs::MetadataExt;
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
    Init { config_path: String },
    Write { tmp_path: String, dest_path: String },
}
fn main() {
let args = Cli::parse();
match args.command {
    Commands::Init { config_path } => {
        if let Err(e) = create_autogen(&config_path) {
            eprintln!("Failed: {e}");
            process::exit(1);
        }
    }
    Commands::Write { tmp_path, dest_path } => {
        if let Err(e) = writer(&tmp_path, &dest_path) {
            eprintln!("Failed: {e}");
            process::exit(1);
        }
    }
}
}
fn create_autogen(config_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let config_dir = std::path::Path::new("/var/lib/niux/");
    fs::create_dir_all(config_dir)?;
    let content = format!(include_str!("../assets/autogen_config.kdl"), config_path);
    fs::write(config_dir.join("niux_autogen.kdl"), content)?;
    Ok(())
}
fn writer(tmp_path: &str, dest_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let tmp_metadata = std::fs::metadata(tmp_path)?;
    if tmp_metadata.uid() == 0 {
        return Err("tmp_path must not be owned by root".into());
    }
    if std::path::Path::new(dest_path).exists() {
        let metadata = std::fs::symlink_metadata(dest_path);
        if metadata?.file_type().is_symlink() {
            let real_path = std::fs::read_link(dest_path)?;
            let real_metadata = std::fs::metadata(&real_path)?;
            let file_uid = real_metadata.uid();
            let current_uid = unsafe { libc::getuid() };
            if file_uid != current_uid {
                return Err("Symlink points to file owned by another user".into());
            }
        }
    }
    let tmp_content = fs::read_to_string(tmp_path)?;
    fs::write(dest_path, tmp_content)?;
    Ok(())
}

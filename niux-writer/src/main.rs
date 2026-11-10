mod errors;
mod cli;

use clap::Parser;
use std::{fs, os::unix::fs::MetadataExt, path::PathBuf};

use crate::{cli::{Cli, Commands}, errors::{NiuxWriterError, NiuxWriterResult}};

fn main() {
    let args = Cli::parse();

    let result = match args.command {
        Commands::Init {
            config_path,
            hook_config_path,
        } => create_autogen(&config_path, &hook_config_path),
        Commands::Write {
            tmp_path,
            dest_path,
        } => writer(&tmp_path, &dest_path),
    };
    if let Err(e) = result {
        let mut report = String::new();

        miette::GraphicalReportHandler::new()
            .render_report(&mut report, &e)
            .expect("failed to render diagnostic");

        eprintln!("{report}");
        std::process::exit(1)
    }
}
fn create_autogen(config_path: &str, hook_config_path: &str) -> NiuxWriterResult<()> {
    let config_dir = std::path::Path::new("/var/lib/niux/");

    fs::create_dir_all(config_dir).map_err(|e| NiuxWriterError::Create {
        path: config_dir.to_path_buf(),
        e,
    })?;

    let content = format!(
        include_str!("assets/autogen_config.kdl"),
        config_path, hook_config_path
    );

    let path_to_write = config_dir.join("niux_autogen.kdl");

    fs::write(&path_to_write, content).map_err(|e| NiuxWriterError::Write {
        path: path_to_write,
        e,
    })?;

    Ok(())
}
fn writer(tmp_path: &str, dest_path: &str) -> NiuxWriterResult<()> {
    let tmp_metadata = std::fs::metadata(tmp_path).map_err(|e| NiuxWriterError::MetadataRead {
        path: PathBuf::from(tmp_path),
        e,
    })?;

    if tmp_metadata.uid() == 0 {
        return Err(NiuxWriterError::RootOwned {
            path: PathBuf::from(tmp_path),
        });
    }

    if std::path::Path::new(dest_path).exists() {
        let metadata = std::fs::symlink_metadata(dest_path).map_err(|e| {
            NiuxWriterError::MetadataSymlinkRead {
                path: PathBuf::from(dest_path),
                e,
            }
        })?;

        if metadata.file_type().is_symlink() {
            let real_metadata = std::fs::canonicalize(dest_path)
                .map_err(|e| NiuxWriterError::Canonicalize {
                    path: PathBuf::from(dest_path),
                    e,
                })?
                .metadata()
                .map_err(|e| NiuxWriterError::MetadataRead {
                    path: PathBuf::from(dest_path),
                    e,
                })?;

            let file_uid = real_metadata.uid();

            let current_uid = rustix::process::getuid().as_raw();

            if file_uid != current_uid {
                return Err(NiuxWriterError::NotOwned {
                    path: PathBuf::from(dest_path),
                });
            }
        }
    }
    let tmp_content = fs::read_to_string(tmp_path).map_err(|e| NiuxWriterError::Read {
        path: PathBuf::from(tmp_path),
        e,
    })?;

    let old_perms = fs::metadata(dest_path).ok().map(|m| m.permissions());

    fs::write(dest_path, tmp_content).map_err(|e| NiuxWriterError::Write {
        path: PathBuf::from(dest_path),
        e,
    })?;

    if let Some(perms) = old_perms {
        fs::set_permissions(dest_path, perms).map_err(|e| NiuxWriterError::SetPermissions {
            path: PathBuf::from(dest_path),
            e,
        })?;
    }
    Ok(())
}

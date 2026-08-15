use clap::{Parser, Subcommand};
use std::{fs, os::unix::fs::MetadataExt};
#[derive(Parser)]
#[command(about = "internal tool, used by niux")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}
#[derive(Subcommand)]
enum Commands {
    Init {
        config_path: String,
        hook_config_path: String,
    },
    Write {
        tmp_path: String,
        dest_path: String,
    },
}
use std::path::PathBuf;
#[derive(Debug, thiserror::Error, miette::Diagnostic)]
enum NiuxWriterError {
    #[error("Failed to create directory: {path}")]
    #[diagnostic(help("ensure that parent directory exists and you have permission to create it"))]
    Create {
        path: PathBuf,

        #[source]
        e: std::io::Error,
    },
    #[error("Failed to write file: {path}")]
    #[diagnostic(help("ensure that the file is writable and the parent directory exists"))]
    Write {
        path: PathBuf,

        #[source]
        e: std::io::Error,
    },
    #[error("Failed to read file: {path}")]
    #[diagnostic(help("ensure that the file exists and is readable"))]
    Read {
        path: PathBuf,

        #[source]
        e: std::io::Error,
    },
    #[error("Failed to resolve: {path}")]
    #[diagnostic(help("ensure that the path exists and can be resolved"))]
    Canonicalize {
        path: PathBuf,

        #[source]
        e: std::io::Error,
    },
    #[error("Failed to read metadata: {path}")]
    #[diagnostic(help("ensure that the path exists and you have permission to access it"))]
    MetadataRead {
        path: PathBuf,

        #[source]
        e: std::io::Error,
    },
    #[error("Failed to read symlink metadata, path: {path}")]
    #[diagnostic(help("ensure that the destination path is accessible"))]
    MetadataSymlinkRead {
        path: PathBuf,

        #[source]
        e: std::io::Error,
    },
    #[error("Temporary path must not be owned by root: {path}")]
    #[diagnostic(help("the temporary file must be owned by the current user"))]
    RootOwned { path: PathBuf },

    #[error("Symlink points to file owned by another user: {path}")]
    #[diagnostic(help("ensure that the target file is owned by the current user"))]
    NotOwned { path: PathBuf },

    #[error("Failed to set permissions: {path}")]
    #[diagnostic(help("ensure that you have permission to modify the file"))]
    SetPermissions {
        path: PathBuf,

        #[source]
        e: std::io::Error,
    },
}
type NiuxWriterResult<T> = std::result::Result<T, NiuxWriterError>;

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
        include_str!("../assets/autogen_config.kdl"),
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

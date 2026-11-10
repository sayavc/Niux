use std::path::PathBuf;

pub type NiuxWriterResult<T> = std::result::Result<T, NiuxWriterError>;

#[derive(Debug, thiserror::Error, miette::Diagnostic)]
pub enum NiuxWriterError {
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

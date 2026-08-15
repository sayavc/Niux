use std::path::PathBuf;

#[derive(Debug, thiserror::Error, miette::Diagnostic)]
pub enum NiuxError {
    #[error(transparent)]
    #[diagnostic(transparent)]
    ConfigError(#[from] ConfigErr),

    #[error(transparent)]
    #[diagnostic(transparent)]
    IoError(#[from] IoErr),

    #[error(transparent)]
    Serde(#[from] serde_json::Error),

    #[error(transparent)]
    #[diagnostic(transparent)]
    NixDrvError(#[from] NixDrvErr),

    #[error(transparent)]
    ShellWords(#[from] shell_words::ParseError),

    #[error(transparent)]
    #[diagnostic(transparent)]
    NixError(#[from] NixErr),

    #[error(transparent)]
    #[diagnostic(transparent)]
    NixConfigError(#[from] NixConfigErr),

    #[error(transparent)]
    #[diagnostic(transparent)]
    EnvError(#[from] EnvErr),

    #[error(transparent)]
    #[diagnostic(transparent)]
    ExecuteError(#[from] ExecuteErr),

    #[error(transparent)]
    #[diagnostic(transparent)]
    CliError(#[from] CliErr),

    #[error(transparent)]
    #[diagnostic(transparent)]
    Utf8Error(#[from] Utf8Err),

    #[error(transparent)]
    #[diagnostic(transparent)]
    InputError(#[from] InputErr),
}

#[derive(Debug, thiserror::Error, miette::Diagnostic)]
pub enum InputErr {
    #[error("Failed to read user input")]
    #[diagnostic(help("check that stdin is available and readable"))]
    Read {
        #[source]
        e: std::io::Error,
    },
}
#[derive(Debug, thiserror::Error, miette::Diagnostic)]
pub enum Utf8Err {
    #[error("Failed to convert path to UTF-8")]
    #[diagnostic(help("the path contains characters that cannot be represented as UTF-8"))]
    InvalidUtf8,

    #[error("Failed to convert string to UTF-8")]
    #[diagnostic(help("the path contains characters that cannot be represented as UTF-8"))]
    InvalidUtf8String {
        #[source]
        e: std::string::FromUtf8Error,
    },
}

#[derive(Debug, thiserror::Error, miette::Diagnostic)]
pub enum CliErr {
    #[error("Cannot install or remove packages for both targets simultaneously")]
    #[diagnostic(help(
        "Packages cannot be installed or removed for both home and system simultaneously"
    ))]
    TargetsRestriction,

    #[error("Extra args cannot be used with `both`")]
    #[diagnostic(help("rebuild home or system separately to pass additional argument"))]
    ExtraArgBadUsage,
}

#[derive(Debug, thiserror::Error, miette::Diagnostic)]
pub enum EnvErr {
    #[error("Environment variable `{name}` is not set")]
    #[diagnostic(help("set the `{name}` environment variable and try again"))]
    NotPresent { name: String },

    #[error("environment variable `{name}` contains invalid UTF-8")]
    #[diagnostic(help("ensure that `{name}` contains valid UTF-8"))]
    NotUnicode { name: String },
}

#[derive(Debug, thiserror::Error, miette::Diagnostic)]
pub enum NixErr {
    #[error("{command} is not installed")]
    #[diagnostic(help("install `{command}` and make sure it is available in PATH"))]
    CommandNotInstalled { command: String },

    #[error("Failed to read Nix directory {dir}")]
    #[diagnostic(
        help("ensure that `{}` is a valid directory", dir.display())
    )]
    ReadDir {
        dir: PathBuf,

        #[source]
        e: std::io::Error,
    },
}

#[derive(Debug, thiserror::Error, miette::Diagnostic)]
pub enum ExecuteErr {
    #[error("Failed to execute command")]
    #[diagnostic(help("check that the command exists and can be executed"))]
    Io {
        #[source]
        e: std::io::Error,
    },
    #[error("Command exited unsuccessfully\n(exit code: {code})\n(command: {command})")]
    #[diagnostic(help("check the command output for more information"))]
    ExitStatus { code: i32, command: String },
}

#[derive(Debug, thiserror::Error, miette::Diagnostic)]
pub enum IoErr {
    #[error(transparent)]
    #[diagnostic(transparent)]
    ConfigIoErr(#[from] ConfigIoErr),

    #[error(transparent)]
    #[diagnostic(transparent)]
    StateDirErr(#[from] StateDirErr),

    #[error(transparent)]
    #[diagnostic(transparent)]
    TmpErr(#[from] TmpErr),
}

#[derive(Debug, thiserror::Error, miette::Diagnostic)]
pub enum ConfigIoErr {
    #[error("Failed to copy config from: {from_path} to {to_path}")]
    #[diagnostic(help("ensure that the source and destination paths are configured correctly"))]
    Copy {
        from_path: PathBuf,
        to_path: PathBuf,

        #[source]
        e: std::io::Error,
    },

    #[error("Failed to read config: {path}")]
    #[diagnostic(help("try `niux --gen-config`"))]
    Read {
        path: PathBuf,

        #[source]
        e: std::io::Error,
    },

    #[error("Failed to deserialize config")]
    #[diagnostic(help("try `niux --gen-config`"))]
    Parse,
}

#[derive(Debug, thiserror::Error, miette::Diagnostic)]
pub enum ConfigErr {
    #[error("Invalid config path: {path}")]
    #[diagnostic(help("ensure that the config path in your Niux configuration is valid"))]
    Invalid { path: PathBuf },
}

#[derive(Debug, thiserror::Error, miette::Diagnostic)]
pub enum NixConfigErr {
    #[error("Marker not found: [{marker}]")]
    #[diagnostic(help("ensure that config paths in the Niux configuration is valid"))]
    MarkerNotFound { marker: String },
    #[error("Failed to determine indentation")]
    #[diagnostic(help("ensure that Nix config is valid"))]
    WrongIndent,
}

#[derive(Debug, thiserror::Error, miette::Diagnostic)]
pub enum StateDirErr {
    #[error("Failed to create Niux state directory, path: {path}")]
    #[diagnostic(help("ensure that your home directory is configured correctly"))]
    Create {
        path: PathBuf,

        #[source]
        e: std::io::Error,
    },

    #[error("state directory could not be determined")]
    #[diagnostic(help("ensure that your home directory is configured correctly"))]
    Unavailable,
}

#[derive(Debug, thiserror::Error, miette::Diagnostic)]
pub enum TmpErr {
    #[error("Failed to create temp config file")]
    #[diagnostic(help("ensure that your temporary directory exists and is writable"))]
    Create {
        #[source]
        e: std::io::Error,
    },

    #[error("Failed to write temp config in tmp dir")]
    #[diagnostic(help("ensure that the temporary file still exists and is readable"))]
    Write {
        #[source]
        e: std::io::Error,
    },

    #[error("Failed to read temporary config file")]
    #[diagnostic(help("ensure that the temporary directory is accessible and writable"))]
    Read {
        #[source]
        e: std::io::Error,
    },
}

#[derive(Debug, thiserror::Error, miette::Diagnostic)]
pub enum NixDrvErr {
    #[error("Failed to parse nix derivation JSON")]
    #[diagnostic(help("the Nix command returned JSON without a `derivations` object"))]
    InvalidDerivationsJson,

    #[error("Failed to parse name in nix derivation JSON")]
    #[diagnostic(help("the Nix command returned JSON without a `name` in `derivations` object"))]
    InvalidNameInDerivationsJson,
}

impl ConfigIoErr {
    pub fn read(path: impl Into<PathBuf>, source: std::io::Error) -> Self {
        Self::Read {
            path: path.into(),
            e: source,
        }
    }
    pub fn copy(
        from_path: impl Into<PathBuf>,
        to_path: impl Into<PathBuf>,
        source: std::io::Error,
    ) -> Self {
        Self::Copy {
            from_path: from_path.into(),
            to_path: to_path.into(),
            e: source,
        }
    }
}

use std::env::VarError;
impl EnvErr {
    pub fn from_var(name: impl Into<String>, error: VarError) -> Self {
        let name = name.into();

        match error {
            VarError::NotPresent => Self::NotPresent { name },
            VarError::NotUnicode(_) => Self::NotUnicode { name },
        }
    }
}

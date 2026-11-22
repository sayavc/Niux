use knus::Decode;
use niux_macros::replace_env;
use std::path::PathBuf;

// main cfg
#[derive(Decode, replace_env)]
pub struct NiuxConfig {
    #[knus(child)]
    pub config_paths: ConfigPaths,
    #[knus(child)]
    pub config_markers: ConfigMarkers,
    #[knus(child)]
    pub features: Option<Features>,
    #[knus(child)]
    pub environment: Environment,
    #[knus(child)]
    pub commands: Commands,
}

#[derive(Decode, replace_env)]
pub struct ConfigPaths {
    #[knus(child, unwrap(argument))]
    pub home: PathBuf,
    #[knus(child, unwrap(argument))]
    pub system: PathBuf,
}

#[derive(Decode, Clone, replace_env)]
pub struct ConfigMarkers {
    #[knus(child, unwrap(argument))]
    pub home: String,
    #[knus(child, unwrap(argument))]
    pub system: String,
    #[knus(child, unwrap(argument))]
    pub home_end: String,
    #[knus(child, unwrap(argument))]
    pub system_end: String,
}

#[derive(Decode, replace_env)]
pub struct Features {
    #[knus(child)]
    pub nvd_integration: Option<Nvd>,
}

#[derive(Decode, Clone, replace_env)]
pub struct Nvd {
    #[knus(child)]
    #[replace_env(skip)]
    pub on: bool,
    #[knus(child)]
    #[replace_env(skip)]
    #[allow(dead_code)]
    off: bool,
    #[knus(child)]
    pub dirs: NvdDirs,
    #[knus(child)]
    pub signature: NvdSig,
}

#[derive(Decode, Clone, replace_env)]
pub struct NvdDirs {
    #[knus(child, unwrap(argument))]
    pub system: PathBuf,
    #[knus(child, unwrap(argument))]
    pub home: PathBuf,
}

#[derive(Decode, Clone, replace_env)]
pub struct NvdSig {
    #[knus(child, unwrap(argument))]
    pub system: String,
    #[knus(child, unwrap(argument))]
    pub home: String,
}

#[derive(Decode, replace_env)]
pub struct Environment {
    #[knus(child, unwrap(argument))]
    pub su_type: String,
    #[knus(child, unwrap(argument))]
    pub editor: String,
}

#[derive(Decode, replace_env)]
pub struct Commands {
    #[knus(child, unwrap(argument))]
    pub rebuild_home: String,
    #[knus(child, unwrap(argument))]
    pub rebuild_system: String,
    #[knus(child, unwrap(argument))]
    pub update_flake: String,
    #[knus(child, unwrap(argument))]
    pub update_inputs: String,
}

// auto generated config
#[derive(Decode, Clone, replace_env)]
pub struct AutoGenNiuxConfig {
    #[knus(child, unwrap(argument))]
    pub config_path: PathBuf,
    #[knus(child, unwrap(argument))]
    pub hooks_config_path: PathBuf,
}

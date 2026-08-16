use knuffel;
use niux_macros::replace_env;
use std::path::PathBuf;

// main cfg
#[derive(knuffel::Decode, replace_env)]
pub struct NiuxConfig {
    #[knuffel(child)]
    pub config_paths: ConfigPaths,
    #[knuffel(child)]
    pub config_markers: ConfigMarkers,
    #[knuffel(child)]
    pub features: Option<Features>,
    #[knuffel(child)]
    pub environment: Environment,
    #[knuffel(child)]
    pub commands: Commands,
}

#[derive(knuffel::Decode, replace_env)]
pub struct ConfigPaths {
    #[knuffel(child, unwrap(argument))]
    pub home: PathBuf,
    #[knuffel(child, unwrap(argument))]
    pub system: PathBuf,
}

#[derive(knuffel::Decode, Clone, replace_env)]
pub struct ConfigMarkers {
    #[knuffel(child, unwrap(argument))]
    pub home: String,
    #[knuffel(child, unwrap(argument))]
    pub system: String,
    #[knuffel(child, unwrap(argument))]
    pub home_end: String,
    #[knuffel(child, unwrap(argument))]
    pub system_end: String,
}

#[derive(knuffel::Decode, replace_env)]
pub struct Features {
    #[knuffel(child)]
    pub nvd_integration: Option<Nvd>,
}

#[derive(knuffel::Decode, Clone, replace_env)]
pub struct Nvd {
    #[knuffel(child)]
    #[replace_env(skip)]
    pub on: bool,
    #[knuffel(child)]
    #[replace_env(skip)]
    #[allow(dead_code)]
    off: bool,
    #[knuffel(child)]
    pub dirs: NvdDirs,
    #[knuffel(child)]
    pub signature: NvdSig,
}

#[derive(knuffel::Decode, Clone, replace_env)]
pub struct NvdDirs {
    #[knuffel(child, unwrap(argument))]
    pub system: PathBuf,
    #[knuffel(child, unwrap(argument))]
    pub home: PathBuf,
}

#[derive(knuffel::Decode, Clone, replace_env)]
pub struct NvdSig {
    #[knuffel(child, unwrap(argument))]
    pub system: String,
    #[knuffel(child, unwrap(argument))]
    pub home: String,
}

#[derive(knuffel::Decode, replace_env)]
pub struct Environment {
    #[knuffel(child, unwrap(argument))]
    pub su_type: String,
    #[knuffel(child, unwrap(argument))]
    pub editor: String,
}

#[derive(knuffel::Decode, replace_env)]
pub struct Commands {
    #[knuffel(child, unwrap(argument))]
    pub rebuild_home: String,
    #[knuffel(child, unwrap(argument))]
    pub rebuild_system: String,
    #[knuffel(child, unwrap(argument))]
    pub update_flake: String,
    #[knuffel(child, unwrap(argument))]
    pub update_inputs: String,
}

// auto generated config
#[derive(knuffel::Decode, Clone, replace_env)]
pub struct AutoGenNiuxConfig {
    #[knuffel(child, unwrap(argument))]
    pub config_path: PathBuf,
    #[knuffel(child, unwrap(argument))]
    pub hooks_config_path: PathBuf,
}

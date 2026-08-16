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
    pub config_path_home: PathBuf,
    #[knuffel(child, unwrap(argument))]
    pub config_path_system: PathBuf,
}

#[derive(knuffel::Decode, Clone, replace_env)]
pub struct ConfigMarkers {
    #[knuffel(child, unwrap(argument))]
    pub marker_home: String,
    #[knuffel(child, unwrap(argument))]
    pub marker_system: String,
    #[knuffel(child, unwrap(argument))]
    pub marker_home_end: String,
    #[knuffel(child, unwrap(argument))]
    pub marker_system_end: String,
}
#[derive(knuffel::Decode, Default, replace_env)]
pub struct Features {
    #[knuffel(child, unwrap(argument))]
    #[replace_env(skip)]
    pub nvd_integration: bool,
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

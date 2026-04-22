use knuffel;
use std::path::PathBuf;
#[derive(knuffel::Decode)]
// Niux.rs
pub struct NiuxConfig {
    #[knuffel(child)]
    pub config_paths: ConfigPaths,
    #[knuffel(child)]
    pub config_markers: ConfigMarkers,
    #[knuffel(child)]
    pub features: Features,
    #[knuffel(child)]
    pub security: Security,
    #[knuffel(child)]
    pub commands: Commands,
}

#[derive(knuffel::Decode)]
pub struct ConfigPaths {
    #[knuffel(child, unwrap(argument))]
    pub config_path_home: String,
    #[knuffel(child, unwrap(argument))]
    pub config_path_system: String,
    #[knuffel(child, unwrap(argument))]
    pub path_nix_flake: String,
}

#[derive(knuffel::Decode)]
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
#[derive(knuffel::Decode)]
pub struct Features {
    #[knuffel(child, unwrap(argument))]
    pub nvd_integration: bool,
}
#[derive(knuffel::Decode)]
pub struct Security {
    #[knuffel(child, unwrap(argument))]
    pub su_type: String,
}
#[derive(knuffel::Decode)]
pub struct Commands {
    #[knuffel(child, unwrap(argument))]
    pub rebuild_home: String,
    #[knuffel(child, unwrap(argument))]
    pub rebuild_system: String,
    #[knuffel(child, unwrap(argument))]
    pub update_flakes: String,
}
// auto generated config
#[derive(knuffel::Decode)]
pub struct AutoGenNiuxConfig {
    #[knuffel(child, unwrap(argument))]
    pub config_path: PathBuf,
    #[knuffel(child, unwrap(argument))]
    pub hooks_config_path: PathBuf,
}

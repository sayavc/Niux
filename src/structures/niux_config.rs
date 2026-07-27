use knuffel;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
pub const NIUX_CONFIG_VERSION: u32 = 1;

// Niux.rs
#[derive(knuffel::Decode, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct NiuxConfig {
    #[knuffel(child, unwrap(argument))]
    pub version: u32,
    #[knuffel(child)]
    pub config_paths: ConfigPaths,
    #[knuffel(child)]
    pub config_markers: ConfigMarkers,
    #[knuffel(child)]
    #[serde(default)]
    pub features: Option<Features>,
    #[knuffel(child)]
    pub environment: Environment,
    #[knuffel(child)]
    pub commands: Commands,
}

#[derive(knuffel::Decode, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct ConfigPaths {
    #[knuffel(child, unwrap(argument))]
    pub config_path_home: PathBuf,
    #[knuffel(child, unwrap(argument))]
    pub config_path_system: PathBuf,
}

#[derive(knuffel::Decode, Deserialize, Serialize, Clone)]
#[serde(rename_all = "kebab-case")]
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
#[derive(knuffel::Decode, Deserialize, Serialize, Default)]
#[serde(rename_all = "kebab-case")]
pub struct Features {
    #[knuffel(child, unwrap(argument))]
    #[serde(default)]
    pub nvd_integration: Option<bool>,
}
#[derive(knuffel::Decode, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Environment {
    #[knuffel(child, unwrap(argument))]
    pub su_type: String,
    #[knuffel(child, unwrap(argument))]
    pub editor: String,
}
#[derive(knuffel::Decode, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
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
#[derive(knuffel::Decode, Deserialize, Serialize, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct AutoGenNiuxConfig {
    #[knuffel(child, unwrap(argument))]
    pub config_path: PathBuf,
    #[knuffel(child, unwrap(argument))]
    pub hooks_config_path: PathBuf,
}

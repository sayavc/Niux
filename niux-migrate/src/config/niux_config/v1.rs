use crate::config::niux_config::v0::{ConfigMarkers, Environment};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct NiuxConfigV1 {
    pub version: u32,
    pub config_paths: ConfigPaths,
    pub config_markers: ConfigMarkers,
    #[serde(default)]
    pub features: Option<Features>,
    pub environment: Environment,
    pub commands: Commands,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct ConfigPaths {
    pub config_path_home: PathBuf,
    pub config_path_system: PathBuf,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Features {
    #[serde(default)]
    pub nvd_integration: Option<bool>,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Commands {
    pub rebuild_home: String,
    pub rebuild_system: String,
    pub update_flake: String,
    pub update_inputs: String,
}

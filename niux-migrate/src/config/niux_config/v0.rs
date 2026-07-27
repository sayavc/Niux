use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct NiuxConfigV0 {
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
    pub path_nix_flake: PathBuf,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct ConfigMarkers {
    pub marker_home: String,
    pub marker_system: String,
    pub marker_home_end: String,
    pub marker_system_end: String,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Features {
    pub nvd_integration: bool,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Environment {
    pub su_type: String,
    pub editor: String,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Commands {
    pub rebuild_home: String,
    pub rebuild_system: String,
    pub update_flakes: String,
}

use knuffel; 
use std::fs;
use crate::utils::{run_bash, user_input};
#[derive(knuffel::Decode)]
pub struct NiuxConfig {
    #[knuffel(child)]
    pub config_paths: ConfigPaths,
    #[knuffel(child)]
    pub config_markers: ConfigMarkers,
}

#[derive(knuffel::Decode)]
pub struct ConfigPaths {
    #[knuffel(child, unwrap(argument))]
    pub config_path_home: String,
    #[knuffel(child, unwrap(argument))]
    pub config_path_system: String,
    #[knuffel(child, unwrap(argument))]
    pub path_home_manager: String,
}

#[derive(knuffel::Decode)]
pub struct ConfigMarkers {
    #[knuffel(child, unwrap(argument))]
    pub marker_home: String,
    #[knuffel(child, unwrap(argument))]
    pub marker_system: String,
}

fn get_config_dir() -> std::path::PathBuf {
let path = dirs::home_dir()
    .expect("no home dir")
    .join(".config/niux/config.kdl");
    path
}
pub fn gen_config() {
    let config_dir = get_config_dir();
    if config_dir.exists() {
        println!("file is exists, rewrite? y/n");
        if user_input().trim() == "n" { 
        return;
        }
    }
    fs::create_dir_all(config_dir.parent().unwrap()).unwrap();
    let home_manager_path = run_bash(&["which", "home-manager"]);
    let default_config = format!(include_str!("assets/default_config.kdl"), home_manager_path);
    fs::write(&config_dir, default_config).unwrap();
}

    pub fn get_config_value() -> NiuxConfig {
        let content = fs::read_to_string(&get_config_dir()).expect("Config not found, run --gen-config");
        knuffel::parse::<NiuxConfig>("config.kdl", &content).expect("failed to parse config")
    }

use crate::structures::AutoGenNiuxConfig;
use std::fs;
use std::process;
use crate::utils::get_home_dir;
impl AutoGenNiuxConfig {
    pub fn create(&self) -> Result<(), Box<dyn std::error::Error>> {
        let config_dir = get_home_dir().join(".local/share/niux");
        fs::create_dir_all(&config_dir)?;
        let content = format!(include_str!("../assets/autogen_config.kdl"), self.config_path.display());
        fs::write(config_dir.join("niux_autogen.kdl"), content)?;
        Ok(())
    }
    fn exist_child() {
        let cfg = AutoGenNiuxConfig { config_path: "/etc/niux.kdl".into() };
        cfg.create();
    }
    pub fn exist() {
        let path = Self::get().unwrap_or_else(|| {
            println!("autogen not found, creating");
            Self::exist_child();
            process::exit(0);
        });
        if !path.config_path.exists() {
            Self::exist_child();
            process::exit(0);
        }
    }
    pub fn get() -> Option<AutoGenNiuxConfig> {
        let content = fs::read_to_string(get_home_dir()
            .join(".local/share/niux/niux_autogen.kdl")).ok()?;
        knuffel::parse::<AutoGenNiuxConfig>("niux_autogen.kdl", &content).ok()
    }
}

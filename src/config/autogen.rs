use crate::structures::AutoGenNiuxConfig;
use crate::utils::{ writer_init };
impl AutoGenNiuxConfig {
    pub fn create(path: Option<std::path::PathBuf>) -> Result<(), Box<dyn std::error::Error>> {
        let path = path.unwrap_or_else(|| std::path::PathBuf::from("/etc/niux.kdl"));
        writer_init(path.to_str().unwrap());
        Ok(()) 
    }
    pub fn get() -> Option<AutoGenNiuxConfig> {
        let content = std::fs::read_to_string("/var/lib/niux/niux_autogen.kdl").ok()?;
        knuffel::parse::<AutoGenNiuxConfig>("niux_autogen.kdl", &content).ok()
    }

}

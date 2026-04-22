use std::fs;
use crate::error;
use crate::structures::{ NiuxConfig, models::Package };
use crate::utils::{run_bash_interactive, command_exists};
impl Package {
    pub fn nvd(&self) -> Result<(), Box<dyn std::error::Error>>  {
        if !NiuxConfig::get().features.unwrap_or_default().nvd_integration { return Ok(()); }
        if !command_exists("nvd") { return Err("Nvd is not installed".into()); }
        let state_dir = match dirs::state_dir() {
            Some(num) => num,
            None => return Err(format!("{}.local/state is not exists", dirs::home_dir().ok_or("home dir is not exists")?.display()).into()),
        };
        if !state_dir.join("niux").exists() { fs::create_dir_all(state_dir.join("niux"))?; }
        if !state_dir.join("niux/nvd_integration.txt").exists() { fs::File::create(state_dir.join("niux/nvd_integration.txt"))?; }
        let (profiles_path, prefix) = if self.is_system {
            (std::path::PathBuf::from("/nix/var/nix/profiles"), "system-")
        } else {
            let local = state_dir.join("nix/profiles");
            let per_user = std::path::PathBuf::from(format!("/nix/var/nix/profiles/per-user/{}", std::env::var("USER")?));
            if local.exists() {
                (local, "home-manager-")
            } else if per_user.exists() {
                (per_user, "home-manager-") 
            } else {
                error!("home-manager is not installed");
                std::process::exit(1);
            }
        };

        let mut entries: Vec<_> = fs::read_dir(&profiles_path)?
            .filter_map(|e| {
                let entry = e.ok()?;
                let name = entry.file_name();
                let s = name.to_string_lossy();
                s.strip_prefix(prefix)?
                .strip_suffix("-link")?
                .parse::<u64>().ok()
        })
        .collect();
        if entries.len() < 2 { return Ok(()) }
        entries.sort();
        let new = entries[entries.len() - 1];
        let old = entries[entries.len() - 2];
        let content = fs::read_to_string(state_dir.join("niux/nvd_integration.txt"))?; 
        let lines: Vec<&str> = content.lines().collect(); 
        let line = if self.is_system { 0 } else { 1 };
        if lines.get(line).is_some_and(|l| l.contains(&new.to_string())) {
            return Ok(());
        }
        run_bash_interactive(&["nvd", "diff", &format!("{}/{prefix}{old}-link", profiles_path.display()), &format!("{}/{prefix}{new}-link", profiles_path.display())])?;
        let content = format!("{}\n{}",
            if self.is_system { new.to_string() } else { lines.get(0).unwrap_or(&"").to_string() },
            if self.is_system { lines.get(1).unwrap_or(&"").to_string() } else { new.to_string() });
        fs::write(state_dir.join("niux/nvd_integration.txt"), content)?;
        Ok(())
    }
}

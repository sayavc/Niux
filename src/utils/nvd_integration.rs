use std::fs;
use crate::error;
use anyhow::bail;
use crate::structures::{ NiuxConfig, models::Package };
use crate::utils::{run_bash_interactive, command_exists};
impl Package {
    pub fn nvd(&self) -> anyhow::Result<()>  {
        if !NiuxConfig::get()?.features.unwrap_or_default().nvd_integration { return Ok(()); }
        if !command_exists("nvd") { bail!("Nvd is not installed"); }
        let state_dir = match dirs::state_dir() {
            Some(num) => num,
            None => {
                let home = dirs::home_dir().unwrap_or_default();
                error!("{}.local/state does not exist", home.display());
                home
            }
        };
        let nvd_dir = state_dir.join("niux");
        if !&nvd_dir.exists() { fs::create_dir_all(&nvd_dir).inspect_err(|e| error!("Failed to create integration file: {e}")).ok(); }
        if !&nvd_dir.join("nvd_integration.txt").exists() { fs::File::create(nvd_dir.join("nvd_integration.txt")).inspect_err(|e| error!("Failed to create integration file: {e}")).ok(); }

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
            if self.is_system { new.to_string() } else { lines.first().unwrap_or(&"").to_string() },
            if self.is_system { lines.get(1).unwrap_or(&"").to_string() } else { new.to_string() });
        fs::write(nvd_dir.join("nvd_integration.txt"), content).inspect_err(|e| error!("Failed to write nvd_interation txt file for no re-output of diff: {e}")).ok();
        Ok(())
    }
}

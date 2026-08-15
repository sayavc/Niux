use crate::structures::NiuxConfig;
use crate::structures::models::{Just, Package, Target};
use crate::utils::{command_exists, run_bash_interactive};
use colored::Colorize;
use std::fs;
impl Package {
    pub fn nvd(ptype: Target) -> crate::NiuxResult<()> {
        let command = "nvd";
        if let Some(features) = &NiuxConfig::get().features {
            if !features.nvd_integration {
                return Ok(());
            }
        } else {
            return Ok(());
        };
        if !command_exists(command) {
            return Err(crate::NixErr::CommandNotInstalled {
                command: command.into(),
            }
            .into());
        }
        let state_dir = match dirs::state_dir() {
            Some(num) => num,
            None => {
                let home = dirs::home_dir().unwrap_or_default();
                eprintln!(
                    "{}{}",
                    home.display().to_string().yellow(),
                    ".local/state does not exist".yellow()
                );
                home
            }
        };
        let (profiles_path, prefix) = match ptype {
            Target::System => (std::path::PathBuf::from("/nix/var/nix/profiles"), "system-"),
            Target::Home => {
                let local = state_dir.join("nix/profiles");
                let per_user = std::path::PathBuf::from(format!(
                    "/nix/var/nix/profiles/per-user/{}",
                    std::env::var("USER").map_err(|e| crate::EnvErr::from_var("USER", e))?
                ));
                if local.exists() {
                    (local, "home-manager-")
                } else if per_user.exists() {
                    (per_user, "home-manager-")
                } else {
                    eprintln!("{}", "home-manager is not installed".red());
                    std::process::exit(1);
                }
            }
            _ => unreachable!(),
        };

        let mut entries: Vec<_> = fs::read_dir(&profiles_path)
            .map_err(|e| crate::NixErr::ReadDir {
                dir: profiles_path.clone(),
                e,
            })?
            .filter_map(|e| {
                let entry = e.ok()?;
                let name = entry.file_name();
                let s = name.to_string_lossy();
                s.strip_prefix(prefix)?
                    .strip_suffix("-link")?
                    .parse::<u64>()
                    .ok()
            })
            .collect();
        if entries.len() < 2 {
            return Ok(());
        }
        entries.sort();
        let new = entries[entries.len() - 1];
        let old = entries[entries.len() - 2];
        run_bash_interactive::<Just>(&[
            command,
            "diff",
            &format!("{}/{prefix}{old}-link", profiles_path.display()),
            &format!("{}/{prefix}{new}-link", profiles_path.display()),
        ])?;
        Ok(())
    }
}

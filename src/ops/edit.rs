use crate::structures::NiuxConfig;
use crate::structures::models::{Home, Just, Package, System, Target};
use crate::utils::{run_bash_interactive, write_changes_to_config};
use anyhow::{Context, bail};
use colored::Colorize;
use tempfile::NamedTempFile;
impl Package {
    pub fn edit(&self) -> anyhow::Result<()> {
        let config = NiuxConfig::get();

        let config_path = match self.ptype {
            Target::Home => &config.config_paths.config_path_home,
            Target::System => &config.config_paths.config_path_system,
            _ => unreachable!(),
        };

        let state_dir = match dirs::state_dir() {
            Some(num) => num,
            None => {
                let home = dirs::home_dir().unwrap_or_default();
                bail!("{}.local/state does not exist", home.display());
            }
        };

        let config_dir = state_dir.join("niux");
        if !config_dir.exists() {
            std::fs::create_dir_all(config_dir).with_context(|| "Failed to create state dir")?;
        }

        let backup_path = state_dir.join("niux/config_backup.nix");

        let content = std::fs::read_to_string(config_path)?;

        let range = match self.ptype {
            Target::Home => config.get_range::<Home>(&content),
            Target::System => config.get_range::<System>(&content),
            _ => unreachable!(),
        }?;

        let old_packages = range.packages.join("\n");

        let tmp = NamedTempFile::new().context("Failed to create tmp file")?;

        std::fs::write(tmp.path(), &old_packages)?;

        match run_bash_interactive::<Just>(&[
            &config.environment.editor,
            tmp.path()
                .to_str()
                .context("path to tmp contains invalid UTF-8")?,
        ]) {
            Ok(_) => {}
            Err(e) => bail!("{e}"),
        }

        std::fs::copy(config_path, state_dir.join(&backup_path))?;

        println!(
            "backup created: {}",
            backup_path.display().to_string().blue()
        );

        let new = std::fs::read_to_string(tmp.path())?.trim_end().to_string();
        let new_packages: Vec<String> = new.lines().map(String::from).collect();

        if old_packages == new {
            println!("{}", "Nothing has changed...".yellow());
            return Ok(());
        }

        let mut lines: Vec<String> = content.lines().map(String::from).collect();

        lines.splice(range.start..range.end, new_packages);

        let new_content = lines.join("\n");

        write_changes_to_config(&new_content, config_path.to_path_buf())?;

        println!("{}", "Packages edited".green());

        Ok(())
    }
}

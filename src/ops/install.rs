use crate::structures::NiuxConfig;
use crate::structures::models::{Home, Package, System, Target};
use crate::utils::{Color, PathExt, user_input, write_changes_to_config};
use colored::Colorize;

impl Package {
    pub fn install(&self) -> crate::NiuxResult<()> {
        log::info!(
            "Install is started, rebuild: {}, ptype: {:?}, package: {:?}",
            self.rebuild,
            self.ptype,
            self.name
        );

        let mut name = self.name.clone();

        let config = NiuxConfig::get();

        let config_path = match self.ptype {
            Target::Home => &config.config_paths.config_path_home,
            Target::System => &config.config_paths.config_path_system,
            _ => unreachable!(),
        };

        let content = config_path.read_to_string()?;

        let range = match self.ptype {
            Target::System => config.get_range::<System>(&content),
            Target::Home => config.get_range::<Home>(&content),
            _ => unreachable!(),
        }?;

        for p in &range.packages {
            let package = p.trim();
            if self.name.contains(&package.to_string()) {
                println!(
                    "{} {} {}",
                    "Package".yellow(),
                    package.cold_white(),
                    "is already installed, add duplicate? y/n".yellow()
                );
                if user_input()?.trim() != "y" {
                    name.retain(|pkg| pkg != package);
                }
            }
        }

        if name.is_empty() {
            return Ok(());
        }

        let mut new_range = range.packages.clone();

        name.iter()
            .for_each(|n| new_range.push(format!("{}{}", " ".repeat(range.indent), n)));

        let old = range.packages.join("\n");
        let new = new_range.join("\n");

        if new == old {
            println!("{}", "Nothing has changed...".yellow());
            return Ok(());
        }

        let mut lines: Vec<String> = content.lines().map(String::from).collect();

        lines.splice(range.start..range.end, new_range);

        let new_content = lines.join("\n");

        write_changes_to_config(&new_content, config_path.to_path_buf())?;

        println!("{}", "Packages installed".green());

        Ok(())
    }
}

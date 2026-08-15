use crate::structures::NiuxConfig;
use crate::structures::models::{Home, Package, System, Target};
use crate::utils::{PathExt, write_changes_to_config};
use colored::Colorize;
use std::collections::HashSet;

impl Package {
    pub fn remove(&self) -> crate::NiuxResult<()> {
        log::info!(
            "Remove is started, rebuild: {}, ptype: {:?}, package: {:?}",
            self.rebuild,
            self.ptype,
            self.name
        );

        let config = NiuxConfig::get();

        let config_path = match self.ptype {
            Target::Home => &config.config_paths.config_path_home,
            Target::System => &config.config_paths.config_path_system,
            _ => unreachable!(),
        };

        let content = config_path.read_to_string()?;

        let range = match self.ptype {
            Target::Home => config.get_range::<Home>(&content)?,
            Target::System => config.get_range::<System>(&content)?,
            _ => unreachable!(),
        };

        let targets: HashSet<String> = self.name.clone().into_iter().collect();

        let result: Vec<String> = range
            .packages
            .iter()
            .filter(|p| !targets.contains(p.trim()))
            .cloned()
            .collect();

        let old = range.packages.join("\n");
        let new = result.join("\n");

        if old == new {
            println!("{}", "Packages not found".yellow());
            return Ok(());
        }

        let mut lines: Vec<String> = content.lines().map(String::from).collect();

        lines.splice(range.start..range.end, result);

        let new_content = lines.join("\n");

        write_changes_to_config(&new_content, config_path.to_path_buf())?;

        println!("{}", "Packages removed".green());

        Ok(())
    }
}

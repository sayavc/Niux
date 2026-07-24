use crate::structures::NiuxConfig;
use crate::structures::models::{Home, Package, System, Target};
use crate::utils::{SortExt, print_packages, print_raw_packages};
use colored::Colorize;
use std::fs;

impl Package {
    pub fn list_all(&self) -> anyhow::Result<()> {
        let config = NiuxConfig::get();
        let content_system = fs::read_to_string(&config.config_paths.config_path_system)?;
        let content_home = fs::read_to_string(&config.config_paths.config_path_home)?;
        let packages_system = config.get_range::<System>(&content_system)?.sorted();
        let packages_home = config.get_range::<Home>(&content_home)?.sorted();

        if self.raw_mode {
            let packages: Vec<String> = packages_system.into_iter().chain(packages_home).collect();
            print_raw_packages(&packages.sorted());
            return Ok(());
        }

        if !print_packages("system", packages_system.iter().peekable(), true)
            & !print_packages("home", packages_home.iter().peekable(), false)
        {
            println!("{}", "Packages list is none".yellow());
        }
        Ok(())
    }
    pub fn list_type(&self) -> anyhow::Result<()> {
        let config = NiuxConfig::get();
        let config_path = match self.ptype {
            Target::Home => &config.config_paths.config_path_home,
            Target::System => &config.config_paths.config_path_system,
            _ => unreachable!(),
        };
        let content = fs::read_to_string(config_path)?;
        let (range, ptype) = match self.ptype {
            Target::Home => (config.get_range::<Home>(&content)?.sorted(), "home"),
            Target::System => (config.get_range::<System>(&content)?.sorted(), "system"),
            _ => unreachable!(),
        };

        if self.raw_mode {
            print_raw_packages(&range);
            return Ok(());
        }

        if !print_packages(ptype, range.iter().peekable(), false) {
            println!("{}", "Packages list is none".yellow())
        }
        Ok(())
    }

    pub fn list_do_package(&self) -> anyhow::Result<()> {
        let config = NiuxConfig::get();
        let result = match self.ptype {
            Target::Home => {
                let content = fs::read_to_string(&config.config_paths.config_path_home)?;
                let packages = config.get_range::<Home>(&content)?.sorted();

                let found = packages
                    .iter()
                    .filter(|p| self.name.iter().any(|n| p.contains(&**n)));
                print_packages("home", found.peekable(), false)
            }
            Target::System => {
                let content = fs::read_to_string(&config.config_paths.config_path_system)?;
                let packages = config.get_range::<System>(&content)?.sorted();

                let found = packages
                    .iter()
                    .filter(|p| self.name.iter().any(|n| p.contains(&**n)));
                print_packages("system", found.peekable(), false)
            }
            Target::None => {
                let content_system = fs::read_to_string(&config.config_paths.config_path_system)?;
                let content_home = fs::read_to_string(&config.config_paths.config_path_home)?;
                let packages_system = config.get_range::<System>(&content_system)?.sorted();
                let packages_home = config.get_range::<Home>(&content_home)?.sorted();

                let found_system = packages_system
                    .iter()
                    .filter(|p| self.name.iter().any(|n| p.contains(&**n)));
                let found_home = packages_home
                    .iter()
                    .filter(|p| self.name.iter().any(|n| p.contains(&**n)));

                match (
                    print_packages("system", found_system.peekable(), true),
                    print_packages("home", found_home.peekable(), false),
                ) {
                    (false, false) => false,
                    (true, _) | (_, true) => true,
                }
            }
            _ => unreachable!(),
        };
        if !result {
            println!("not found:");
            for n in &self.name {
                println!("- {}", n.blue());
            }
        }
        Ok(())
    }
}

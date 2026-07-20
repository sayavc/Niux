use crate::structures::NiuxConfig;
use crate::structures::models::{Home, Package, System, Target};
use colored::{Colorize, CustomColor};
use std::fs;
use std::iter::Peekable;

impl Package {
    pub fn list_all(&self) -> anyhow::Result<()> {
        let config = NiuxConfig::get();
        let content_system = fs::read_to_string(&config.config_paths.config_path_system)?;
        let content_home = fs::read_to_string(&config.config_paths.config_path_home)?;
        let mut packages_system = config.get_range::<System>(&content_system)?;
        let mut packages_home = config.get_range::<Home>(&content_home)?;

        if self.raw_mode {
            let mut packages: Vec<String> =
                packages_system.into_iter().chain(packages_home).collect();
            packages.sort();
            Self::print_raw_packages(&packages);
            return Ok(());
        }

        packages_system.sort();
        packages_home.sort();

        if !Self::print_packages("system", packages_system.iter().peekable(), true)
            & !Self::print_packages("home", packages_home.iter().peekable(), false)
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
        let (mut range, ptype) = match self.ptype {
            Target::Home => (config.get_range::<Home>(&content)?, "home"),
            Target::System => (config.get_range::<System>(&content)?, "system"),
            _ => unreachable!(),
        };

        range.sort();

        if self.raw_mode {
            Self::print_raw_packages(&range);
            return Ok(());
        }

        if !Self::print_packages(ptype, range.iter().peekable(), false) {
            println!("{}", "Packages list is none".yellow())
        }
        Ok(())
    }

    fn print_packages<'a>(
        ptype: &str,
        mut packages: Peekable<impl Iterator<Item = &'a String>>,
        whitespace: bool,
    ) -> bool {
        if packages.peek().is_some() {
            println!(" {}:", ptype.cyan().bold());
            for p in packages {
                println!(
                    " {} {}",
                    "-".blue(),
                    p.trim().custom_color(CustomColor {
                        r: 240,
                        g: 246,
                        b: 252,
                    })
                );
            }
            if whitespace {
                println!();
            }
            true
        } else {
            false
        }
    }
    fn print_raw_packages(packages: &Vec<String>) {
        for p in packages {
            println!("{}", p.trim())
        }
    }
    pub fn list_do_package(&self) -> anyhow::Result<()> {
        let config = NiuxConfig::get();
        let result = match self.ptype {
            Target::Home => {
                let content = fs::read_to_string(&config.config_paths.config_path_home)?;
                let mut packages = config.get_range::<Home>(&content)?;

                packages.sort();

                let found = packages
                    .iter()
                    .filter(|p| self.name.iter().any(|n| p.contains(&**n)));
                Self::print_packages("home", found.peekable(), false)
            }
            Target::System => {
                let content = fs::read_to_string(&config.config_paths.config_path_system)?;
                let mut packages = config.get_range::<System>(&content)?;

                packages.sort();

                let found = packages
                    .iter()
                    .filter(|p| self.name.iter().any(|n| p.contains(&**n)));
                Self::print_packages("system", found.peekable(), false)
            }
            Target::None => {
                let content_system = fs::read_to_string(&config.config_paths.config_path_system)?;
                let content_home = fs::read_to_string(&config.config_paths.config_path_home)?;
                let mut packages_system = config.get_range::<System>(&content_system)?;
                let mut packages_home = config.get_range::<Home>(&content_home)?;

                packages_system.sort();
                packages_home.sort();

                let found_system = packages_system
                    .iter()
                    .filter(|p| self.name.iter().any(|n| p.contains(&**n)));
                let found_home = packages_home
                    .iter()
                    .filter(|p| self.name.iter().any(|n| p.contains(&**n)));

                match (
                    Self::print_packages("system", found_system.peekable(), true),
                    Self::print_packages("home", found_home.peekable(), false),
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

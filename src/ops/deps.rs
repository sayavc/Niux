use crate::structures::NiuxConfig;
use crate::structures::models::{Home, Just, Package, System, Target};
use crate::utils::{Color, PathExt, SanitizePackages, SortExt, bash, print_packages};
use serde_json::Value;

impl Package {
    pub fn deps_list_all() -> crate::NiuxResult<()> {
        let config = NiuxConfig::get();

        let content_system = config.config_paths.config_path_system.read_to_string()?;
        let content_home = config.config_paths.config_path_home.read_to_string()?;

        let packages_system = config.get_range::<System>(&content_system)?;
        let packages_home = config.get_range::<Home>(&content_home)?;

        let deps_system = Self::transform(packages_system.packages)?.sorted();
        let deps_home = Self::transform(packages_home.packages)?.sorted();

        println!("{}", "system".cold_white());
        for (pkg, deps) in deps_system {
            if !print_packages(&pkg, deps.sorted().iter().peekable(), true) {
                log::info!("system packages dependencies not found");
            }
        }

        println!("{}", "home".cold_white());
        for (pkg, deps) in deps_home {
            if !print_packages(&pkg, deps.sorted().iter().peekable(), false) {
                log::info!("home packages dependencies not found");
            }
        }
        Ok(())
    }

    pub fn deps_list_type(&self) -> crate::NiuxResult<()> {
        log::info!("Deps list_type is started, ptype: {:?}", self.ptype);

        let config = NiuxConfig::get();

        let path = match self.ptype {
            Target::Home => &config.config_paths.config_path_home,
            Target::System => &config.config_paths.config_path_system,
            _ => unreachable!(),
        };

        let content = path.read_to_string()?;

        let (packages, ptype) = match self.ptype {
            Target::Home => (config.get_range::<Home>(&content)?.packages, "home"),
            Target::System => (config.get_range::<System>(&content)?.packages, "system"),
            _ => unreachable!(),
        };

        let ndeps = Self::transform(packages)?.sorted();

        println!("{}", ptype.cold_white());
        for (pkg, deps) in ndeps {
            if !print_packages(&pkg, deps.sorted().iter().peekable(), false) {
                log::info!("{} packages dependencies not found", ptype)
            }
        }
        Ok(())
    }

    pub fn deps_list_do_package(&self) -> crate::NiuxResult<()> {
        log::info!("Deps list_do_package is started, ptype: {:?}", self.ptype);
        let deps = Self::transform(self.name.clone())?.sorted();

        for (pkg, deps) in deps {
            if !print_packages(&pkg, deps.sorted().iter().peekable(), false) {
                log::info!("system packages dependencies not found")
            }
        }
        Ok(())
    }

    fn normalize_installable(packages: Vec<String>) -> Vec<String> {
        packages
            .into_iter()
            .map(|p| {
                if p.contains('#') {
                    p.trim().to_string()
                } else {
                    format!("nixpkgs#{}", p.trim())
                }
            })
            .collect::<Vec<String>>()
    }

    fn transform(packages: Vec<String>) -> crate::NiuxResult<Vec<(String, Vec<String>)>> {
        log::info!("Deps transform is started, packages: {:?}", packages);

        let packages = Self::normalize_installable(packages).sanitize_packages();
        let mut command = vec!["nix", "derivation", "show"];

        command.extend(packages.iter().map(String::as_str));
        command.push("--impure");

        log::info!("Command: {:?}", command);

        let output = bash::<Just>(&command)?;
        let json = serde_json::from_str::<Value>(&output)?;

        let drv_values = json["derivations"]
            .as_object()
            .ok_or(crate::NixDrvErr::InvalidDerivationsJson)?
            .values();

        let mut drv_map: Vec<(String, &Value)> = Vec::new();
        for d in drv_values {
            drv_map.push((
                d["name"]
                    .as_str()
                    .ok_or(crate::NixDrvErr::InvalidNameInDerivationsJson)?
                    .to_string(),
                d,
            ));
        }

        let result: Vec<(String, Vec<String>)> = drv_map
            .into_iter()
            .filter_map(|(name, drv)| {
                let names = drv["inputs"]["drvs"].as_object()?.keys();
                let deps: Vec<String> = names
                    .filter_map(|n| Some(n.split_once("-")?.1.rsplit_once(".drv")?.0.to_string()))
                    .collect();
                Some((name, deps))
            })
            .collect();
        Ok(result)
    }
}

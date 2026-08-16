use crate::structures::models::Just;
use crate::structures::{NiuxConfig, models::Commands};
use crate::utils::common::{bash, user_input};
use colored::Colorize;
impl NiuxConfig {
    pub fn autodetect() -> crate::NiuxResult<Commands> {
        println!("{}", "Do you have flakes? y/n".blue());
        let flakes = loop {
            match user_input()?.trim() {
                "y" => break true,
                "n" => break false,
                _ => {
                    println!("Incorrect answer");
                    continue;
                }
            };
        };

        println!("{}", "Do you have standalone home-manager? y/n".blue());

        let home_manager = loop {
            match user_input()?.trim() {
                "y" => break true,
                "n" => break false,
                _ => {
                    println!("Incorrect answer");
                    continue;
                }
            };
        };

        Ok(Commands {
            rebuild_system: Self::rebuild_system_command(flakes)?,
            rebuild_home: Self::rebuild_home_command(flakes, home_manager)?,
            update_flake: Self::update_flake_command(flakes),
            update_inputs: Self::update_inputs_command(flakes),
        })
    }
    fn rebuild_system_command(flakes: bool) -> crate::NiuxResult<String> {
        let hostname = bash::<Just>(&["hostname"])?;
        let mut args = vec!["sudo", "nixos-rebuild", "switch"];
        let flake_arg = format!("/etc/nixos#{}", hostname);

        if flakes {
            args.push("--flake");
            args.push(&flake_arg);
        }

        Ok(args.join(" "))
    }
    fn rebuild_home_command(flakes: bool, home_manager: bool) -> crate::NiuxResult<String> {
        let user = std::env::var("USER").map_err(|e| crate::EnvErr::from_var("USER", e))?;

        if !home_manager {
            return Self::rebuild_system_command(flakes);
        }

        let flake_arg = &format!("/etc/nixos#{}", user);
        let mut args = vec!["home-manager", "switch"];

        if flakes {
            args.push("--flake");
            args.push(flake_arg);
        }

        Ok(args.join(" "))
    }
    pub fn update_flake_command(flakes: bool) -> String {
        if flakes {
            "sudo nix flake update --flake /etc/nixos".to_string()
        } else {
            "nix-channel update".to_string()
        }
    }
    pub fn update_inputs_command(flakes: bool) -> String {
        if flakes {
            "sudo nix flake update [packages] --flake /etc/nixos".to_string()
        } else {
            "nix-channel update [packages]".to_string()
        }
    }
}

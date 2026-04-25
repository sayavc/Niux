use anyhow::Context;
use colored::Colorize;
use crate::structures::{ NiuxConfig, Commands };
use crate::utils::common::{ run_bash, user_input };
impl NiuxConfig {
    pub fn autodetect() -> anyhow::Result<Commands> {
        println!("{}", "Do you have flakes? y/n".blue());
        let flakes = loop {
            match user_input().trim() {
                "y" => break true,
                "n" => break false,
                _ => { println!("Inccorect answer"); continue; }
            };
        };
        println!("{}", "Do you have standalone home-manager? y/n".blue());
        let home_manager = loop {
            match user_input().trim() {
                "y" => break true,
                "n" => break false,
                _ => { println!("Inccorect answer"); continue; }
            };
        };
        Ok(Commands {
            rebuild_system: Self::rebuild_system_command(flakes)?,
            rebuild_home: Self::rebuild_home_command(flakes, home_manager)?,
            update_flakes: Self::update_flakes_command(flakes),
        })
    }
    fn rebuild_system_command(flakes: bool) -> anyhow::Result<String> {
        let hostname = run_bash(&["hostname"])?;
        let mut args = vec!["sudo", "nixos-rebuild", "switch"];
        let flake_arg = format!("/etc/nixos#{}", hostname);
        if flakes {
            args.push("--flake");
            args.push(&flake_arg);
        }
        Ok(args.join(" "))
    }
    fn rebuild_home_command(flakes: bool, home_manager: bool) -> anyhow::Result<String> {
        let user = std::env::var("USER").with_context(|| "Failed to get var $USER".to_string())?;
        if !home_manager {
        return Self::rebuild_system_command(flakes);
        }
        let flake_arg = &format!("/etc/nixos#{}", user);
        let mut args = vec![
            "home-manager",
            "switch"
        ];
        if flakes {
            args.push("--flake");
            args.push(flake_arg);
        }
        Ok(args.join(" "))
    }
    pub fn update_flakes_command(flakes: bool)-> String {
        if flakes {
        "sudo nix flake update --flake /etc/nixos".to_string()
        } else {
            "nix-channel update".to_string()
        }
    }

}

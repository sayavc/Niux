use std::process;
use colored::Colorize;
use crate::error;
use crate::structures::{ NiuxConfig, Commands };
use crate::utils::common::{ run_bash, user_input };
impl NiuxConfig {
    pub fn autodetect() -> Commands {
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
        Commands {
            rebuild_system: Self::rebuild_system_command(flakes),
            rebuild_home: Self::rebuild_home_command(flakes, home_manager),
            update_flakes: Self::update_flakes_command(flakes),
        }
    }
    fn rebuild_system_command(flakes: bool) -> String {
        let hostname = run_bash(&["hostname"]);
        let mut args = vec!["sudo", "nixos-rebuild", "switch"];
        let flake_arg = format!("/etc/nixos#{}", hostname);
        if flakes {
            args.push("--flake");
            args.push(&flake_arg);
        }
        args.join(" ")
    }
    fn rebuild_home_command(flakes: bool, home_manager: bool) -> String {
        let user = std::env::var("USER").unwrap_or_else(|e| { error!("{e}"); process::exit(1); });
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
        args.join(" ")
    }
    pub fn update_flakes_command(flakes: bool)-> String {
        if flakes {
        "sudo nix flake update --flake /etc/nixos".to_string()
        } else {
            "nix-channel update".to_string()
        }
    }

}

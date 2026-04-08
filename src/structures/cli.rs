use clap::{ Parser, ArgGroup };
#[derive(Parser)]
#[command(
    name = "niux",
    about = "A simple CLI tool for managing NixOS packages",
    long_about = "Declarative NixOS/home-manager CLI package manager written in Rust.\n\nManage system and home packages, update flake inputs, and rebuild configurations.",
    before_help = "NIUX - NixOS Package Manager",
    after_help = "EXAMPLES:\n  niux -Hi firefox                Install firefox for home\n  niux -Hr firefox                Remove firefox from home\n  niux -Si vim                    Install vim for system\n  niux -Sr vim                    Remove vim from system\n  niux -HSi firefox vim           Install firefox for home and vim for system\n\nBUILD & APPLY:\n  niux -Ha                        Rebuild home config\n  niux -Sa                        Rebuild system config\n  niux -HSa                       Rebuild home and system configs\n  niux -Hia firefox               Install firefox for home and rebuild\n  niux -Sia vim                   Install vim for system and rebuild\n  niux -HSia firefox vim          Install packages and rebuild both configs\n\nLISTING & SEARCH:\n  niux -Hl                        List packages in home config\n  niux -Sl                        List packages in system config\n  niux -Hl firefox                Search \"firefox\" in home config\n  niux -l firefox                 Search \"firefox\" in home and system configs\n\nUPDATES:\n  niux -U                         Update all flakes\n  niux -Ua                        Update flakes and rebuild\n  niux -Ua nixpkgs                Update specific flake input (e.g., nixpkgs)\n  niux -USa                       Update flakes and rebuild system and home\n  niux -USHa                      Update flakes and rebuild system and home configs\n\nFor more information, visit: https://github.com/sayavc/niux")]
#[clap(group(
        ArgGroup::new("target_group")
        .args(["home", "system"])
        .multiple(true)
))]
pub struct Args {
    #[arg(long, conflicts_with_all = ["home", "system", "install", "remove", "update", "apply", "package", "default_path_config", "list", "clear"],
        help = "Generate a default configuration file")]
    pub gen_config: bool,
    #[arg(long, conflicts_with_all = ["home", "system", "install", "remove", "update", "apply", "package", "gen_config", "list"],
        help = "Set default configuration path")]
    pub default_path_config: Option<std::path::PathBuf>,
    #[arg(long, conflicts_with_all = ["home", "system", "install", "remove", "update", "apply", "package", "gen_config", "list"],
        help = "Displays current path")]
    pub get_current_path: bool,
    #[arg(long, conflicts_with_all = ["home", "system", "install", "remove", "update", "apply", "package", "gen_config", "list"],
        help = "This is nix-collect-garbage")]
    pub clear: bool,
    #[arg(short = 'H', required_unless_present_any = ["system", "gen_config", "default_path_config", "update", "list", "clear", "get_current_path"],
        help = "Manage home packages")]
    pub home: bool,
    #[arg(short = 'S', required_unless_present_any = ["home", "gen_config", "default_path_config", "update", "list", "clear", "get_current_path"],
        help = "Manage system packages")]
    pub system: bool,
    #[arg(short = 'i', conflicts_with_all = ["remove", "update"], 
        help = "Install packages")]
    pub install: bool,
    #[arg(short = 'r', conflicts_with_all = ["install", "update"],
        help = "Remove packages")]
    pub remove: bool,
    #[arg(short = 'U', conflicts_with_all = ["install", "remove"], 
        help = "Update flakes")]
    pub update: bool,
    #[arg(short = 'a', requires = "target_group", 
        help = "rebuild nixos configuration")] 
    pub apply: bool,
    #[arg(short = 'l', conflicts_with_all = ["install", "remove", "update", "apply"], 
        help = "List packages")]
    pub list: bool,
    pub package: Option<Vec<String>>,
}

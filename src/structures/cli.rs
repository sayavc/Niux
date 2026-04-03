use clap::Parser;
#[derive(Parser)]
pub struct Args {
    #[arg(long, conflicts_with_all = ["home", "system", "install", "uninstall", "apply", "package", "default_path_config"])]
    pub gen_config: bool,
    #[arg(long, conflicts_with_all = ["home", "system", "install", "uninstall", "apply", "package", "gen_config"])]
    pub default_path_config: Option<std::path::PathBuf>,
    #[arg(short = 'H', required_unless_present_any = ["system", "gen_config", "default_path_config"])]
    pub home: bool,
    #[arg(short = 'S', required_unless_present_any = ["home", "gen_config", "default_path_config"])]
    pub system: bool,
    #[arg(short = 'i', conflicts_with = "uninstall")]
    pub install: bool,
    #[arg(short = 'u', conflicts_with = "install")]
    pub uninstall: bool,
    #[arg(short = 'a')]
    pub apply: bool,
    pub package: Option<Vec<String>>,
}

use clap::Parser;
#[derive(Parser)]
pub struct Args {
    #[arg(long, conflicts_with_all = ["home", "system", "install", "uninstall", "rebuild_config", "package_name"])]
    pub gen_config: bool,
    #[arg(short = 'H')]
    pub home: bool,
    #[arg(short = 'S')]
    pub system: bool,
    #[arg(short = 'i', conflicts_with = "uninstall")]
    pub install: bool,
    #[arg(short = 'u', conflicts_with = "install")]
    pub uninstall: bool,
    #[arg(short = 'a')]
    pub rebuild_config: bool,
    #[arg(num_args = 1.., required_if_eq("install", "true"))]
    pub package_name: Option<Vec<String>>,
    #[arg(conflicts_with_all = ["gen_config", "home", "system", "install", "uninstall", "rebuild_config", "package_name"])]
    pub edit_func: Option<String>,
}

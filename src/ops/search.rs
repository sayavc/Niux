use crate::structures::hook_config::HookConfig;
use crate::structures::models::{HookEvent, Just, Package};
use crate::utils::{bash, command_exists};
use anyhow::bail;
impl Package {
    pub fn search(&self) -> anyhow::Result<()> {
        log::info!("search is started, package: {:?}", self.name);
        HookConfig::run(HookEvent::PreSearch)?;
        if !command_exists("nix-search") {
            bail!("nix-search is not installed");
        }
        if self.name.is_empty() {
            return Ok(());
        }
        println!(
            "{}",
            bash::<Just>(&["nix-search", &self.name[0]])?
                .lines()
                .filter_map(|line| line.split_whitespace().next())
                .collect::<Vec<_>>()
                .join("\n")
        );
        HookConfig::run(HookEvent::PostSearch)?;
        Ok(())
    }
}

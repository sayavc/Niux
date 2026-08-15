use crate::structures::hook_config::HookConfig;
use crate::structures::models::{HookEvent, Just, Package};
use crate::utils::{bash, command_exists};
impl Package {
    pub fn search(&self) -> crate::NiuxResult<()> {
        log::info!("search is started, package: {:?}", self.name);
        let command = "nix-search";

        HookConfig::run(HookEvent::PreSearch)?;
        if !command_exists(command) {
            return Err(crate::NixErr::CommandNotInstalled {
                command: command.into(),
            }
            .into());
        }
        if self.name.is_empty() {
            return Ok(());
        }
        println!(
            "{}",
            bash::<Just>(&[command, &self.name[0]])?
                .lines()
                .filter_map(|line| line.split_whitespace().next())
                .collect::<Vec<_>>()
                .join("\n")
        );
        HookConfig::run(HookEvent::PostSearch)?;
        Ok(())
    }
}

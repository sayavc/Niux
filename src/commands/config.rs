use crate::structures::models::Package;
use crate::structures::{Args, models::Action};
impl Action {
    pub fn dispatch_config(&self, args: &Args) -> anyhow::Result<bool> {
        match self {
            Action::ShowPath => {
                Package::show_path();
                Ok(true)
            }
            Action::GenConfig => {
                Package::gen_config()?;
                Ok(true)
            }
            Action::SetConfigPath => {
                Package::set_config_path(args)?;
                Ok(true)
            }
            Action::SetHookConfigPath => {
                Package::set_hook_config_path(args)?;
                Ok(true)
            }
            Action::Migrate => {
                Package::migrate()?;
                Ok(true)
            }
            _ => Ok(false),
        }
    }
}

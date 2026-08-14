use crate::structures::models::HookEvent;
use crate::structures::{hook_config::HookConfig, models::Action};
impl Action {
    pub fn pre_hooks(&self) -> anyhow::Result<()> {
        match self {
            Action::Install => HookConfig::run(HookEvent::PreInstall),
            Action::Remove => HookConfig::run(HookEvent::PreRemove),
            Action::Edit => HookConfig::run(HookEvent::PreEdit),
            Action::List(_) => HookConfig::run(HookEvent::PreList),
            Action::Search => HookConfig::run(HookEvent::PreSearch),
            Action::Update(_) => HookConfig::run(HookEvent::PreUpdate),
            _ => Ok(()),
        }
    }
    pub fn post_hooks(&self) -> anyhow::Result<()> {
        match self {
            Action::Install => HookConfig::run(HookEvent::PostInstall),
            Action::Remove => HookConfig::run(HookEvent::PostRemove),
            Action::Edit => HookConfig::run(HookEvent::PostEdit),
            Action::List(_) => HookConfig::run(HookEvent::PostList),
            Action::Search => HookConfig::run(HookEvent::PostSearch),
            Action::Update(_) => HookConfig::run(HookEvent::PostUpdate),
            _ => Ok(()),
        }
    }
}

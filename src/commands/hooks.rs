use crate::structures::{
    HookEvent,
    hook_config::HookConfig,
    models::{
        Action,
    },
};
impl Action {
    pub fn pre_hooks(&self) -> anyhow::Result<()> {
        match self {
            Action::Install => HookConfig::run(HookEvent::PreInstall),
            Action::Remove => HookConfig::run(HookEvent::PreRemove),
            Action::List(_) => HookConfig::run(HookEvent::PreList),
            Action::Search => HookConfig::run(HookEvent::PreSearch),
            Action::Clear => HookConfig::run(HookEvent::PreClear),
            Action::Update(_) => HookConfig::run(HookEvent::PreUpdate),
            _ => unreachable!(""),
        }
    }
    pub fn post_hooks(&self) -> anyhow::Result<()> {
        match self {
            Action::Install => HookConfig::run(HookEvent::PostInstall),
            Action::Remove => HookConfig::run(HookEvent::PostRemove),
            Action::List(_) => HookConfig::run(HookEvent::PostList),
            Action::Search => HookConfig::run(HookEvent::PostSearch),
            Action::Clear => HookConfig::run(HookEvent::PostClear),
            Action::Update(_) => HookConfig::run(HookEvent::PostUpdate),
            _ => unreachable!(),
        }
    }
}

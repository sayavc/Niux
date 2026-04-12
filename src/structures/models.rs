pub struct Package {
    pub name: Vec<String>,
    pub is_system: bool,
    pub rebuild: bool,
}
pub enum HookEvent {
    PreInstall,
    PostInstall,
    PreRemove,
    PostRemove,
    PreRebuild,
    PostRebuild,
    PreUpdate,
    PostUpdate,
    PreList,
    PostList,
    PreClear,
    PostClear,
}

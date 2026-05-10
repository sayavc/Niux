#[derive(Clone)]
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
    PreSearch,
    PostSearch,
}
#[allow(dead_code)]
pub struct Commands {
    rebuild_system: String,
    rebuild_home: String,
    update_flakes: String,
}
pub enum Target { System, Home, Both, None }
pub enum Action { Install, Remove, Search, List(List), Clear, Update(Update), ShowPath, GenConfig, SetConfigPath, SetHookConfigPath }
pub enum List { Package, Type, All }
pub enum Update { Just, Flakes }
pub enum Rebuild { Home, System, Both, None }

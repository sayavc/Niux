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
    PreEdit,
    PostEdit,
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
pub struct Commands {
    pub rebuild_system: String,
    pub rebuild_home: String,
    pub update_flakes: String,
    pub editor: String,
}
pub enum Target { System, Home, Both, None }
pub enum Action { Install, Remove, Edit, Search, List(List), Clear, Update(Update), ShowPath, GenConfig, SetConfigPath, SetHookConfigPath }
pub enum List { Package, Type, All }
pub enum Update { Just, Flakes }
pub enum Rebuild { Home, System, Both, None }

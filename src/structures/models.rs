pub struct Package {
    pub name: Vec<String>,
    pub ptype: Target,
    pub rebuild: bool,
    pub raw_mode: bool,
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
    PreSearch,
    PostSearch,
}
pub struct Commands {
    pub rebuild_system: String,
    pub rebuild_home: String,
    pub update_flake: String,
    pub update_inputs: String,
}
#[derive(Debug)]
pub enum Target {
    System,
    Home,
    Both,
    None,
}
pub enum Action {
    Install,
    Remove,
    Edit,
    Search,
    List(List),
    Deps(List),
    Update(Update),
    ShowPath,
    GenConfig,
    SetConfigPath,
    SetHookConfigPath,
    None,
}
pub enum List {
    Package,
    Type,
    All,
}
pub enum Update {
    Just,
    Flakes,
}
pub enum Rebuild<'a> {
    Home(&'a [String]),
    System(&'a [String]),
    Both,
    None,
}

pub struct PackagesRange {
    pub packages: Vec<String>,
    pub start: usize,
    pub end: usize,
    pub indent: usize,
}

pub struct ConfigPath;
pub struct HooksPath;

pub struct Home;
pub struct System;

pub struct Just;
pub struct Early;

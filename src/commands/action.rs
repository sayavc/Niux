use crate::structures::{
    Args,
    models::{
        Action,
        List,
        Update,
        Rebuild,
    },
};

impl Args {
    pub fn action(&self) -> Action {
        match () {
            _ if self.install => Action::Install,
            _ if self.remove => Action::Remove,
            _ if self.search => Action::Search,
            _ if self.list => Action::List(self.list_mode()),
            _ if self.clear => Action::Clear,
            _ if self.update => Action::Update(self.update_mode()),
            _ if self.show_path => Action::ShowPath,
            _ if self.gen_config => Action::GenConfig,
            _ if self.config.is_some() => Action::SetConfigPath,
            _ if self.hook_config.is_some() => Action::SetHookConfigPath,
            _ => unreachable!("Clap ensures one flag is always present"),
        }
    }
    pub fn list_mode(&self) -> List {
        match (self.home, self.system, self.package.is_some()) {
            (false, false, false) => List::All,
            (true, _, false) => List::Type,
            (_, true, false) => List::Type,
            (_, _, true) => List::Package,
        }
    }
    pub fn update_mode(&self) -> Update {
        match self.package.is_some() {
            true => Update::Flakes,
            false => Update::Just,
        }
    }
    pub fn rebuild_mode(&self) -> Rebuild {
        match (self.apply, self.system, self.home) {
            (true, true, false) => Rebuild::System,
            (true, false, true) => Rebuild::Home,
            (true, true, true) => Rebuild::Both,
            _ => Rebuild::None,
        }
    }
}

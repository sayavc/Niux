use crate::structures::models::{Action, List, Package, Update};
impl Action {
    pub fn dispatch(&self, package: &Package) -> anyhow::Result<()> {
        match self {
            Action::Install => package.install(),
            Action::Remove => package.remove(),
            Action::Edit => package.edit(),
            Action::Search => package.search(),
            Action::Update(Update::Just) => Package::update(),
            Action::Update(Update::Flakes) => package.update_flake(),
            Action::List(List::All) => package.list_all(),
            Action::List(List::Type) => package.list_type(),
            Action::List(List::Package) => package.list_do_package(),
            Action::Deps(List::All) => Package::deps_list_all(),
            Action::Deps(List::Type) => package.deps_list_type(),
            Action::Deps(List::Package) => package.deps_list_do_package(),
            _ => Ok(()),
        }
    }
}

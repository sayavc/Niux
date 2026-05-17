use crate::{
    structures::{
        Package,
        models::{
            Action,
            List,
            Update,
        },
    },
};
impl Action {
    pub fn dispatch(&self, package: &Package) -> anyhow::Result<()> {
        match self {
            Action::Install => package.install(),
            Action::Remove => package.remove(),
            Action::Edit => package.edit(),
            Action::Clear => Package::clear(),
            Action::Search => package.search(),
            Action::Update(Update::Just) => Package::update(),
            Action::Update(Update::Flakes) => package.update_flake(),
            Action::List(List::All) => Package::list_all(),
            Action::List(List::Type) => package.list_type(),
            Action::List(List::Package) => package.list_do_package(),
            _ => Ok(()),
        }
    }
}

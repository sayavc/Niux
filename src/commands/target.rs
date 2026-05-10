use crate::structures::{
    Args,
    models::Target
};
impl Args {
    pub fn target(&self) -> Target {
        match (self.system, self.home) {
            (true, false) => Target::System,
            (false, true) => Target::Home,
            (true, true) => Target::Both,
            (false, false) => Target::None,
        }
    }
}

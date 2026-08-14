use crate::structures::models::{Package, Rebuild};
impl Rebuild<'_> {
    pub fn rebuild_wrapper(&self, package: &Package) -> anyhow::Result<()> {
        match self {
            Rebuild::Both => {
                package.rebuild_system(&[])?;
                package.rebuild_home(&[])?;
            }
            Rebuild::Home(extra) => {
                package.rebuild_home(extra)?;
            }
            Rebuild::System(extra) => {
                package.rebuild_system(extra)?;
            }
            Rebuild::None => {}
        }
        Ok(())
    }
}

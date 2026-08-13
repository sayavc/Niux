use crate::structures::models::{Package, Rebuild};
impl Rebuild {
    pub fn rebuild_wrapper(&self, package: &Package) -> anyhow::Result<()> {
        match self {
            Rebuild::Both => {
                package.rebuild_system()?;
                package.rebuild_home()?;
                Ok(())
            }
            Rebuild::Home => {
                package.rebuild_home()?;
                Ok(())
            }
            Rebuild::System => {
                package.rebuild_system()?;
                Ok(())
            }
            Rebuild::None => Ok(()),
        }
    }
}

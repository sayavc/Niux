use crate::structures::Args;
use anyhow::bail;
pub fn validate(args: &Args) -> anyhow::Result<()> {
    if (args.install || args.remove || args.edit) && args.home && args.system {
        bail!("Cannot install/remove to both targets simultaneously");
    }
    Ok(())
}

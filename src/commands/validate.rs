use crate::structures::Args;
use anyhow::bail;

pub fn validate(args: &Args) -> anyhow::Result<()> {
    if (args.install || args.remove || args.edit) && args.home && args.system {
        bail!("Cannot install/remove to both targets simultaneously");
    }

    if args.home && args.system && !args.extra.is_empty() {
        bail!(
            "extra args cannot be used with `both`\nhelp: rebuild home or system separately to pass additional arguments"
        );
    }
    Ok(())
}

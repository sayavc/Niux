use crate::structures::Args;

pub fn validate(args: &Args) -> crate::NiuxResult<()> {
    if (args.install || args.remove || args.edit) && args.home && args.system {
        return Err(crate::CliErr::TargetsRestriction.into());
    }

    if args.home && args.system && !args.extra.is_empty() {
        return Err(crate::CliErr::ExtraArgBadUsage.into());
    }
    Ok(())
}

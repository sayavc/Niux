use crate::structures::models::{Home, Just, System};
use crate::structures::{NiuxConfig, Nvd};
use crate::utils::{SortExt, command_exists, run_bash_interactive};

pub trait TargetNvd {
    fn ntype(nvd: Nvd) -> (std::path::PathBuf, String);
}

impl TargetNvd for Home {
    fn ntype(nvd: Nvd) -> (std::path::PathBuf, String) {
        (nvd.dirs.home, nvd.signature.home)
    }
}

impl TargetNvd for System {
    fn ntype(nvd: Nvd) -> (std::path::PathBuf, String) {
        (nvd.dirs.system, nvd.signature.system)
    }
}

pub fn nvd<T: TargetNvd>() -> crate::NiuxResult<()> {
    let command = "nvd";

    let config = NiuxConfig::get();

    let Some(nvd_integration) = config
        .features
        .as_ref()
        .and_then(|f| f.nvd_integration.clone())
    else {
        return Ok(());
    };

    if !nvd_integration.on {
        return Ok(());
    }

    if !command_exists(command) {
        return Err(crate::NixErr::CommandNotInstalled {
            command: command.into(),
        }
        .into());
    }

    let (path, signature) = T::ntype(nvd_integration);

    let dir_list = std::fs::read_dir(&path).map_err(|e| crate::NixErr::ReadDir {
        dir: path.clone(),
        e,
    })?;

    let Some((prefix, suffix)) = signature.split_once("[number]") else {
        return Err(crate::ConfigErr::InvalidSig { sig: signature }.into());
    };

    let entries = dir_list
        .filter_map(|e| {
            let entry = e.ok()?;

            let name = entry.file_name();
            let s = name.to_string_lossy();

            s.strip_prefix(prefix)?
                .strip_suffix(suffix)?
                .parse::<u64>()
                .ok()
        })
        .collect::<Vec<_>>()
        .sorted();

    if entries.len() < 2 {
        return Ok(());
    }

    let new = entries[entries.len() - 1];
    let old = entries[entries.len() - 2];

    run_bash_interactive::<Just>(&[
        command,
        "diff",
        &format!("{}/{prefix}{old}{suffix}", path.display()),
        &format!("{}/{prefix}{new}{suffix}", path.display()),
    ])?;
    Ok(())
}

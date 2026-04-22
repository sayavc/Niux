use std::fs;
use crate::structures::NiuxConfig;
use crate::utils::{run_bash, command_exists};
pub fn nvd() -> Result<(), Box<dyn std::error::Error>>  {
    if !NiuxConfig::get().features.unwrap_or_default().nvd_integration { return Ok(()); }
    if !command_exists("nvd") { return Err("Nvd is not installed".into()); }
    let state_dir = match dirs::state_dir() {
        Some(num) => num,
        None => return Err(format!("{}.local/state is not exists", dirs::home_dir().ok_or("home dir is not exists")?.display()).into()),
    };
    if !state_dir.join("niux").exists() { fs::create_dir_all(state_dir.join("niux"))?; }
    if !state_dir.join("niux/nvd_integration.txt").exists() { fs::File::create(state_dir.join("niux/nvd_integration.txt"))?; }

    let mut entries: Vec<_> = fs::read_dir("/nix/var/nix/profiles")?
        .filter_map(|e| {
            let entry = e.ok()?;
            let name = entry.file_name();
            let s = name.to_string_lossy();
            s.strip_prefix("system-")?
                .strip_suffix("-link")?
                .parse::<u64>().ok()
        })
    .collect();
    if entries.len() < 2 { return Ok(()) }
    entries.sort();
    let new = entries[entries.len() - 1];
    let old = entries[entries.len() - 2];
    let content = fs::read_to_string(state_dir.join("niux/nvd_integration.txt"))?; 
    let lines: Vec<&str> = content.lines().collect(); 
    if lines.first().is_some_and(|l| l.contains(&new.to_string())) {
        return Ok(());
    }
    run_bash(&["nvd", "diff", &format!("/nix/var/nix/profiles/system-{old}-link"), &format!("/nix/var/nix/profiles/system-{new}-link")]);
    fs::write(state_dir.join("niux/nvd_integration.txt"), new.to_string())?;
    Ok(())
}

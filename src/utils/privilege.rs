use crate::utils::user_input;
use anyhow::Context;
use std::process;
fn privilege_type(nya: &str) -> anyhow::Result<String> {
    let output = process::Command::new("which")
        .arg(nya)
        .env("PATH", std::env::var("PATH").unwrap_or_default())
        .output()
        .context(format!("Failed to run bash command: which {nya}"))?;
    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    }
    else {
        Ok(String::new())
    }
}
pub fn get_privilege_type() -> anyhow::Result<String> {
    for su in &["doas", "sudo", "run0", "pkexec"] {
        if !privilege_type(su)?.is_empty() {
            return Ok(su.to_string());
        }
    }
    println!("Privilege escalation tool not found. Enter yours (e.g. sudo, doas)");
    Ok(user_input().trim().to_string())
}

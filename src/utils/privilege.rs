use std::process;
fn privilege_type(nya: &str) -> String {
    let output = process::Command::new("which")
        .arg(nya)
        .env("PATH", std::env::var("PATH").unwrap_or_default())
        .output()
        .unwrap();
    if output.status.success() {
        String::from_utf8_lossy(&output.stdout).trim().to_string()
    }
    else {
        String::new()
    }
}
pub fn get_privilege_type() -> String {
    for su in &["doas", "sudo", "run0", "pkexec"] {
        if !privilege_type(su).is_empty() {
            return su.to_string();
        }
    }
    " ".to_string()
}

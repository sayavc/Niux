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
    if privilege_type("sudo").is_empty() {
        if privilege_type("doas").is_empty() {
            println!("unknown privilege type");
            std::process::exit(1);
        } else {
            "doas".to_string()
        }
    } else {
        "sudo".to_string()
    }
}

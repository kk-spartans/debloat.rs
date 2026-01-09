use std::process::Command;

pub fn check_admin() -> Result<(), String> {
    if is_admin() {
        println!("[+] Running with administrator privileges");
        Ok(())
    } else {
        Err("This application must be run as Administrator".to_string())
    }
}

fn is_admin() -> bool {
    Command::new("net")
        .args(["session"])
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

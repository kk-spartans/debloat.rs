use crate::apps::onedrive::remove_onedrive;
use crate::apps::outlook::remove_outlook;
use std::process::{Command, Stdio};

pub fn uninstall_outlook_onedrive() -> Result<(), String> {
    remove_outlook()?;

    // OneDrive removal
    println!("Removing OneDrive...");
    remove_onedrive();

    // Restart Explorer
    println!("Restarting Explorer...");
    restart_explorer();

    Ok(())
}

pub fn restart_explorer() {
    let _ = Command::new("taskkill")
        .args(["/F", "/IM", "explorer.exe"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .output();

    std::thread::sleep(std::time::Duration::from_secs(2));

    let _ = Command::new("explorer.exe")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn();
}

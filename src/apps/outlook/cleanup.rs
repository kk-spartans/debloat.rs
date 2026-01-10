use std::env;
use std::path::Path;
use std::process::{Command, Stdio};

pub fn remove_outlook_windowsapps_folders() {
    let windows_apps = "C:\\Program Files\\WindowsApps";
    if !Path::new(windows_apps).exists() {
        return;
    }

    if let Ok(entries) = std::fs::read_dir(windows_apps) {
        for entry in entries.flatten() {
            let path = entry.path();
            if let Some(name) = path.file_name() {
                let name_str = name.to_string_lossy().to_string();
                if name_str.contains("Microsoft.OutlookForWindows") {
                    println!("    Taking ownership of: {name_str}");
                    let _ = Command::new("takeown")
                        .args(["/f", path.to_str().unwrap_or(""), "/r", "/d", "Y"])
                        .stdout(Stdio::null())
                        .stderr(Stdio::null())
                        .output();

                    println!("    Granting permissions...");
                    let _ = Command::new("icacls")
                        .args([path.to_str().unwrap_or(""), "/grant", "administrators:F", "/t"])
                        .stdout(Stdio::null())
                        .stderr(Stdio::null())
                        .output();

                    println!("    Removing folder: {name_str}");
                    let _ = std::fs::remove_dir_all(&path);
                }
            }
        }
    }
}

pub fn remove_outlook_shortcuts() {
    let programdata = env::var("ProgramData").unwrap_or_default();
    let appdata = env::var("APPDATA").unwrap_or_default();
    let public = env::var("PUBLIC").unwrap_or_default();
    let userprofile = env::var("USERPROFILE").unwrap_or_default();

    let shortcuts = [
        format!("{programdata}\\Microsoft\\Windows\\Start Menu\\Programs\\Outlook.lnk"),
        format!("{appdata}\\Microsoft\\Windows\\Start Menu\\Programs\\Outlook.lnk"),
        format!("{programdata}\\Microsoft\\Windows\\Start Menu\\Programs\\Microsoft Office\\Outlook.lnk"),
        format!("{appdata}\\Microsoft\\Windows\\Start Menu\\Programs\\Microsoft Office\\Outlook.lnk"),
        format!("{public}\\Desktop\\Outlook.lnk"),
        format!("{userprofile}\\Desktop\\Outlook.lnk"),
        format!("{public}\\Desktop\\Microsoft Outlook.lnk"),
        format!("{userprofile}\\Desktop\\Microsoft Outlook.lnk"),
        format!("{public}\\Desktop\\Outlook (New).lnk"),
        format!("{userprofile}\\Desktop\\Outlook (New).lnk"),
        format!("{programdata}\\Microsoft\\Windows\\Start Menu\\Programs\\Outlook (New).lnk"),
        format!("{appdata}\\Microsoft\\Windows\\Start Menu\\Programs\\Outlook (New).lnk"),
    ];

    let mut removed_count = 0;
    for shortcut in shortcuts {
        if Path::new(&shortcut).exists() {
            let _ = std::fs::remove_file(&shortcut);
            removed_count += 1;
        }
    }
    if removed_count > 0 {
        println!("    Removed {removed_count} Outlook shortcuts");
    }
}

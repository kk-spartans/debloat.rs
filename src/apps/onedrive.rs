use std::env;
use std::ffi::OsStr;
use std::iter::once;
use std::os::windows::ffi::OsStrExt;
use std::path::Path;
use std::process::{Command, Stdio};
use windows::core::PCWSTR;

use windows::Win32::System::Registry::{
    RegDeleteKeyExW, RegDeleteTreeW, HKEY_CLASSES_ROOT, HKEY_CURRENT_USER, KEY_WOW64_32KEY,
    KEY_WOW64_64KEY,
};

pub fn remove_onedrive() {
    // Execute OneDriveSetup.exe /uninstall
    let _ = Command::new(r"C:\Windows\SysWOW64\OneDriveSetup.exe")
        .args(["/uninstall"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .output();

    // Remove OneDrive folders and shortcuts
    let onedrive_paths = vec![
        format!(
            "{}\\Microsoft\\Windows\\Start Menu\\Programs\\OneDrive.lnk",
            env::var("ProgramData").unwrap_or_default()
        ),
        format!(
            "{}\\Microsoft\\Windows\\Start Menu\\Programs\\OneDrive.lnk",
            env::var("APPDATA").unwrap_or_default()
        ),
        format!(
            "{}\\Desktop\\OneDrive.lnk",
            env::var("PUBLIC").unwrap_or_default()
        ),
        format!(
            "{}\\Desktop\\OneDrive.lnk",
            env::var("USERPROFILE").unwrap_or_default()
        ),
        format!("{}\\OneDrive", env::var("USERPROFILE").unwrap_or_default()),
        format!(
            "{}\\Microsoft\\OneDrive",
            env::var("LOCALAPPDATA").unwrap_or_default()
        ),
        format!(
            "{}\\Microsoft\\OneDrive",
            env::var("ProgramData").unwrap_or_default()
        ),
        format!("C:\\OneDriveTemp"),
    ];

    for path_str in onedrive_paths {
        let path = Path::new(&path_str);
        if path.exists() {
            if path.is_dir() {
                let _ = std::fs::remove_dir_all(path);
            } else {
                let _ = std::fs::remove_file(path);
            }
        }
    }

    // Remove OneDrive registry keys
    let registry_keys = vec![
        ("HKCR", "CLSID\\{018D5C66-4533-4307-9B53-224DE2ED1FE6}"),
        ("HKCR", "Wow6432Node\\CLSID\\{018D5C66-4533-4307-9B53-224DE2ED1FE6}"),
        ("HKCU", "Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Desktop\\NameSpace\\{018D5C66-4533-4307-9B53-224DE2ED1FE6}"),
    ];

    for (hive_str, key_path) in registry_keys {
        let hive = if hive_str == "HKCR" {
            HKEY_CLASSES_ROOT
        } else {
            HKEY_CURRENT_USER
        };

        unsafe {
            let key_wide = to_wide(key_path);
            let key_pcwstr = PCWSTR(key_wide.as_ptr());

            let _ = RegDeleteTreeW(hive, key_pcwstr);
            let _ = RegDeleteKeyExW(hive, key_pcwstr, KEY_WOW64_32KEY.0, None);
            let _ = RegDeleteKeyExW(hive, key_pcwstr, KEY_WOW64_64KEY.0, None);
        }
    }
}

fn to_wide(s: &str) -> Vec<u16> {
    OsStr::new(s).encode_wide().chain(once(0)).collect()
}

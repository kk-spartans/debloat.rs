use std::env;
use std::ffi::OsStr;
use std::iter::once;
use std::os::windows::ffi::OsStrExt;
use std::path::Path;
use std::process::{Command, Stdio};
use windows::core::PCWSTR;

use windows::Win32::System::Diagnostics::ToolHelp::{
    CreateToolhelp32Snapshot, PROCESSENTRY32W, Process32FirstW, Process32NextW, TH32CS_SNAPPROCESS,
};
use windows::Win32::System::Registry::{
    HKEY_CLASSES_ROOT, HKEY_CURRENT_USER, KEY_WOW64_32KEY, KEY_WOW64_64KEY, RegDeleteKeyExW,
    RegDeleteTreeW,
};
use windows::Win32::System::Threading::{OpenProcess, PROCESS_ALL_ACCESS, TerminateProcess};

#[allow(clippy::unnecessary_wraps)]
pub fn remove_onedrive() -> Result<(), String> {
    println!("    Terminating OneDrive processes...");
    let _ = terminate_processes("onedrive");

    println!("    Running OneDrive setup uninstall...");
    execute_onedrive_setup();

    println!("    Removing OneDrive folders and shortcuts...");
    remove_onedrive_paths();

    println!("    Removing OneDrive registry keys...");
    remove_onedrive_registry();

    Ok(())
}

fn terminate_processes(name_pattern: &str) -> Result<(), String> {
    unsafe {
        let snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0)
            .map_err(|e| format!("Failed to create process snapshot: {e:?}"))?;

        let mut entry = PROCESSENTRY32W {
            #[allow(clippy::cast_possible_truncation)]
            dwSize: std::mem::size_of::<PROCESSENTRY32W>() as u32,
            ..Default::default()
        };

        if Process32FirstW(snapshot, &raw mut entry).is_ok() {
            loop {
                let name = String::from_utf16_lossy(
                    &entry.szExeFile[..entry
                        .szExeFile
                        .iter()
                        .position(|&x| x == 0)
                        .unwrap_or(entry.szExeFile.len())],
                )
                .to_lowercase();

                if name.contains(name_pattern) {
                    let process_handle =
                        OpenProcess(PROCESS_ALL_ACCESS, false, entry.th32ProcessID).map_err(
                            |e| format!("Failed to open process {}: {e:?}", entry.th32ProcessID),
                        )?;

                    let _ = TerminateProcess(process_handle, 1);
                    let _ = windows::Win32::Foundation::CloseHandle(process_handle);
                }

                if Process32NextW(snapshot, &raw mut entry).is_err() {
                    break;
                }
            }
        }

        Ok(())
    }
}

fn execute_onedrive_setup() {
    let setup_paths = vec![
        r"C:\Windows\SysWOW64\OneDriveSetup.exe",
        r"C:\Windows\System32\OneDriveSetup.exe",
    ];

    for path in setup_paths {
        if Path::new(path).exists() {
            let _ = Command::new(path)
                .args(["/uninstall"])
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .output();
            return;
        }
    }
}

fn remove_onedrive_paths() {
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
        r"C:\OneDriveTemp".to_string(),
    ];

    let mut removed_count = 0;
    for path_str in onedrive_paths {
        let path = Path::new(&path_str);
        if path.exists() {
            if path.is_dir() {
                let _ = std::fs::remove_dir_all(path);
            } else {
                let _ = std::fs::remove_file(path);
            }
            removed_count += 1;
        }
    }
    if removed_count > 0 {
        println!("    Removed {removed_count} OneDrive paths");
    }
}

fn remove_onedrive_registry() {
    unsafe {
        let registry_keys = vec![
            (
                HKEY_CLASSES_ROOT,
                "CLSID\\{018D5C66-4533-4307-9B53-224DE2ED1FE6}",
            ),
            (
                HKEY_CLASSES_ROOT,
                "Wow6432Node\\CLSID\\{018D5C66-4533-4307-9B53-224DE2ED1FE6}",
            ),
            (
                HKEY_CURRENT_USER,
                "Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Desktop\\NameSpace\\{018D5C66-4533-4307-9B53-224DE2ED1FE6}",
            ),
        ];

        for (hive, key_path) in registry_keys {
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

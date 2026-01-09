use std::env;
use std::path::Path;
use std::process::{Command, Stdio};
use windows::core::w;

use windows::Win32::System::Diagnostics::ToolHelp::{
    CreateToolhelp32Snapshot, Process32FirstW, Process32NextW, PROCESSENTRY32W, TH32CS_SNAPPROCESS,
};
use windows::Win32::System::Registry::{RegDeleteTreeW, HKEY_CURRENT_USER};
use windows::Win32::System::Threading::{OpenProcess, TerminateProcess, PROCESS_ALL_ACCESS};

pub fn remove_outlook() -> Result<(), String> {
    // Close Outlook processes
    println!("Closing Outlook processes...");
    terminate_processes("outlook")?;
    std::thread::sleep(std::time::Duration::from_secs(2));

    // Remove Outlook apps
    println!("Removing Outlook apps...");
    remove_outlook_appx();

    // Remove Outlook folders
    println!("Removing Outlook folders...");
    remove_outlook_folders();

    // Remove shortcuts
    println!("Removing shortcuts...");
    remove_outlook_shortcuts();

    // Taskbar cleanup
    println!("Cleaning taskbar registry entries...");
    clean_taskbar_registry();

    Ok(())
}

fn terminate_processes(name_pattern: &str) -> Result<(), String> {
    unsafe {
        let snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0)
            .map_err(|e| format!("Failed to create process snapshot: {e:?}"))?;

        #[allow(clippy::cast_possible_truncation)]
        let mut entry = PROCESSENTRY32W {
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

fn remove_outlook_appx() {
    println!("[*] Removing Outlook Appx packages...");

    let apps = vec!["Microsoft.OutlookForWindows", "Microsoft.Outlook.Desktop"];

    for app in apps {
        let _ = Command::new("powershell")
            .args([
                "-NoProfile",
                "-ExecutionPolicy",
                "Bypass",
                "-Command",
                &format!(
                    "Get-AppxPackage {app} | Remove-AppxPackage -ErrorAction SilentlyContinue"
                ),
            ])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .output();
    }

    println!("[+] Outlook Appx packages removed");
}

fn remove_outlook_folders() {
    println!("[*] Removing Outlook folders...");

    let local_appdata = env::var("LOCALAPPDATA").unwrap_or_default();
    let appdata = env::var("APPDATA").unwrap_or_default();

    let folders = vec![
        format!("{}\\Microsoft\\Outlook", local_appdata),
        format!("{}\\Microsoft\\Outlook", appdata),
    ];

    for folder in folders {
        if Path::new(&folder).exists() {
            let _ = std::fs::remove_dir_all(&folder);
        }
    }

    println!("[+] Outlook folders removed");
}

fn remove_outlook_shortcuts() {
    println!("[*] Removing Outlook shortcuts...");

    let programdata = env::var("ProgramData").unwrap_or_default();
    let appdata = env::var("APPDATA").unwrap_or_default();
    let public = env::var("PUBLIC").unwrap_or_default();
    let userprofile = env::var("USERPROFILE").unwrap_or_default();

    let shortcuts = vec![
        format!(
            "{}\\Microsoft\\Windows\\Start Menu\\Programs\\Outlook.lnk",
            programdata
        ),
        format!(
            "{}\\Microsoft\\Windows\\Start Menu\\Programs\\Outlook.lnk",
            appdata
        ),
        format!("{}\\Desktop\\Outlook.lnk", public),
        format!("{}\\Desktop\\Outlook.lnk", userprofile),
    ];

    for shortcut in shortcuts {
        if Path::new(&shortcut).exists() {
            let _ = std::fs::remove_file(&shortcut);
        }
    }

    println!("[+] Outlook shortcuts removed");
}

fn clean_taskbar_registry() {
    println!("[*] Cleaning taskbar registry...");

    unsafe {
        let keys = vec![(
            HKEY_CURRENT_USER,
            w!("SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Taskband"),
        )];

        for (hive, path) in keys {
            let _ = RegDeleteTreeW(hive, path);
        }
    }

    println!("[+] Taskbar registry cleaned");
}

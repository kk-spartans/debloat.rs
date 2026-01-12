use std::env;
use std::path::Path;
use std::process::{Command, Stdio};
use tracing::{debug, error};
use windows::Win32::System::Diagnostics::ToolHelp::{
    CreateToolhelp32Snapshot, PROCESSENTRY32W, Process32FirstW, Process32NextW, TH32CS_SNAPPROCESS,
};
use windows::Win32::System::Registry::{HKEY_CURRENT_USER, HKEY_LOCAL_MACHINE, RegDeleteTreeW};
use windows::Win32::System::Threading::{OpenProcess, PROCESS_ALL_ACCESS, TerminateProcess};
use windows::core::w;

use crate::apps::edge_services::{create_protective_folders, remove_edge_services};
use crate::debloat::uninstall_oo::restart_explorer;

pub fn remove_edge() -> Result<(), String> {
    debug!("Terminating Edge processes...");
    terminate_edge_processes()?;

    debug!("Running Edge setup uninstall...");
    uninstall_edge_setup();

    debug!("Removing Start Menu shortcuts...");
    remove_start_menu_shortcuts();

    debug!("Cleaning Edge folders...");
    clean_edge_folders();

    debug!("Cleaning Edge registry entries...");
    clean_edge_registry();

    debug!("Uninstalling EdgeUpdate...");
    uninstall_edge_update();

    debug!("Removing Edge services...");
    remove_edge_services()?;

    debug!("Running final Edge uninstall...");
    uninstall_edge_setup();

    debug!("Restarting Explorer...");
    restart_explorer();

    debug!("Creating protective folders...");
    create_protective_folders()?;

    Ok(())
}

fn uninstall_edge_update() {
    let update_path = r"C:\Program Files (x86)\Microsoft\EdgeUpdate\MicrosoftEdgeUpdate.exe";
    let _ = Command::new(update_path)
        .args(["/uninstall"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .output();
}

fn uninstall_edge_setup() {
    let _ = Command::new("powershell")
        .args([
            "-NoProfile",
            "-ExecutionPolicy",
            "Bypass",
            "-Command",
            r"Get-ChildItem 'C:\Program Files (x86)\Microsoft\Edge\Application\*\Installer\setup.exe' | ForEach-Object { & $_.FullName --uninstall --system-level --verbose-logging --force-uninstall }",
        ])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .output();
}

fn remove_start_menu_shortcuts() {
    let shortcuts = vec![
        r"C:\ProgramData\Microsoft\Windows\Start Menu\Programs\Microsoft Edge.lnk",
        r"C:\ProgramData\Microsoft\Windows\Start Menu\Programs\Microsoft Edge Dev.lnk",
        r"C:\ProgramData\Microsoft\Windows\Start Menu\Programs\Microsoft Edge Beta.lnk",
        r"C:\ProgramData\Microsoft\Windows\Start Menu\Programs\Microsoft Edge Canary.lnk",
    ];

    let mut removed_count = 0;
    for shortcut in shortcuts {
        if Path::new(shortcut).exists() {
            let _ = std::fs::remove_file(shortcut);
            removed_count += 1;
        }
    }
    if removed_count > 0 {
        debug!("Removed {removed_count} Edge shortcuts");
    }
}

fn clean_edge_folders() {
    let local_appdata = env::var("LOCALAPPDATA").unwrap_or_default();
    let appdata = env::var("APPDATA").unwrap_or_default();

    let folders = vec![
        r"C:\Program Files (x86)\Microsoft\Edge".to_string(),
        r"C:\Program Files\Microsoft\Edge".to_string(),
        r"C:\Program Files (x86)\Microsoft\EdgeCore".to_string(),
        format!("{}\\Microsoft\\Edge", local_appdata),
        format!("{}\\Microsoft\\Edge", appdata),
    ];

    let mut removed_count = 0;
    for folder in folders {
        if Path::new(&folder).exists() {
            let _ = std::fs::remove_dir_all(&folder);
            removed_count += 1;
        }
    }
    if removed_count > 0 {
        debug!("Removed {removed_count} Edge folders");
    }
}

fn clean_edge_registry() {
    unsafe {
        let keys = vec![
            (HKEY_CURRENT_USER, w!("SOFTWARE\\Microsoft\\Edge")),
            (HKEY_CURRENT_USER, w!("SOFTWARE\\Microsoft\\EdgeUpdate")),
            (HKEY_LOCAL_MACHINE, w!("SOFTWARE\\Microsoft\\Edge")),
            (
                HKEY_LOCAL_MACHINE,
                w!("SOFTWARE\\WOW6432Node\\Microsoft\\Edge"),
            ),
            (
                HKEY_LOCAL_MACHINE,
                w!("SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\App Paths\\msedge.exe"),
            ),
        ];

        for (root, path) in keys {
            let _ = RegDeleteTreeW(root, path);
        }
    }
}

fn terminate_edge_processes() -> Result<(), String> {
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

                if name.contains("edge") {
                    let process_handle =
                        OpenProcess(PROCESS_ALL_ACCESS, false, entry.th32ProcessID).map_err(
                            |e| format!("Failed to open process {}: {e:?}", entry.th32ProcessID),
                        )?;

                    if let Err(e) = TerminateProcess(process_handle, 1) {
                        error!(
                            "Failed to terminate Edge process {}: {e:?}",
                            entry.th32ProcessID
                        );
                    }

                    if let Err(e) = windows::Win32::Foundation::CloseHandle(process_handle) {
                        error!(
                            "Failed to close handle for process {}: {e:?}",
                            entry.th32ProcessID
                        );
                    }
                }

                if Process32NextW(snapshot, &raw mut entry).is_err() {
                    break;
                }
            }
        }

        Ok(())
    }
}

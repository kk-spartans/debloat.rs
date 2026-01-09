use std::env;
use std::path::Path;
use std::process::{Command, Stdio};
use windows::core::w;
use windows::Win32::System::Diagnostics::ToolHelp::{
    CreateToolhelp32Snapshot, Process32FirstW, Process32NextW, PROCESSENTRY32W, TH32CS_SNAPPROCESS,
};
use windows::Win32::System::Registry::{RegDeleteTreeW, HKEY_CURRENT_USER, HKEY_LOCAL_MACHINE};
use windows::Win32::System::Threading::{OpenProcess, TerminateProcess, PROCESS_ALL_ACCESS};

use crate::apps::edge_services::{create_protective_folders, remove_edge_services};
use crate::debloat::uninstall_oo::restart_explorer;

pub fn remove_edge() -> Result<(), String> {
    println!("Edge Vanisher started");
    println!("Starting Microsoft Edge uninstallation process...");

    // Terminate Edge processes
    println!("Terminating Edge processes...");
    terminate_edge_processes()?;

    // Uninstall Edge with setup.exe
    println!("Uninstalling Edge with setup...");
    uninstall_edge_setup();

    // Remove Start Menu shortcuts
    println!("Removing Start Menu shortcuts...");
    remove_start_menu_shortcuts();

    // Clean Edge folders
    println!("Cleaning Edge folders...");
    clean_edge_folders();

    // Clean Edge registry entries
    println!("Cleaning Edge registry entries...");
    clean_edge_registry();

    // Force uninstall EdgeUpdate
    uninstall_edge_update();

    // Remove EdgeUpdate services
    remove_edge_services()?;

    // Finally force uninstall Edge again
    uninstall_edge_setup();

    // Restart Explorer
    restart_explorer();

    println!("\nMicrosoft Edge uninstallation process completed!");

    // Create protective Edge folders
    println!("Creating protective Edge folders...");
    create_protective_folders()?;

    println!("Protective folders created and security settings configured for Edge and EdgeCore.");

    Ok(())
}

fn uninstall_edge_update() {
    println!("[*] Uninstalling Edge Update...");

    let update_path = r"C:\Program Files (x86)\Microsoft\EdgeUpdate\MicrosoftEdgeUpdate.exe";
    let _ = Command::new(update_path)
        .args(["/uninstall"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .output();

    println!("[+] Edge Update uninstalled");
}

fn uninstall_edge_setup() {
    println!("[*] Uninstalling Edge setup...");

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

    println!("[+] Edge setup uninstalled");
}

fn remove_start_menu_shortcuts() {
    println!("[*] Removing start menu shortcuts...");

    let shortcuts = vec![
        r"C:\ProgramData\Microsoft\Windows\Start Menu\Programs\Microsoft Edge.lnk",
        r"C:\ProgramData\Microsoft\Windows\Start Menu\Programs\Microsoft Edge Dev.lnk",
        r"C:\ProgramData\Microsoft\Windows\Start Menu\Programs\Microsoft Edge Beta.lnk",
        r"C:\ProgramData\Microsoft\Windows\Start Menu\Programs\Microsoft Edge Canary.lnk",
    ];

    for shortcut in shortcuts {
        if Path::new(shortcut).exists() {
            let _ = std::fs::remove_file(shortcut);
        }
    }

    println!("[+] Start menu shortcuts removed");
}

fn clean_edge_folders() {
    println!("[*] Cleaning Edge folders...");

    let local_appdata = env::var("LOCALAPPDATA").unwrap_or_default();
    let appdata = env::var("APPDATA").unwrap_or_default();

    let folders = vec![
        r"C:\Program Files (x86)\Microsoft\Edge".to_string(),
        r"C:\Program Files\Microsoft\Edge".to_string(),
        format!("{}\\Microsoft\\Edge", local_appdata),
        format!("{}\\Microsoft\\Edge", appdata),
    ];

    for folder in folders {
        if Path::new(&folder).exists() {
            let _ = std::fs::remove_dir_all(&folder);
        }
    }

    println!("[+] Edge folders cleaned");
}

fn clean_edge_registry() {
    println!("[*] Cleaning Edge registry...");

    unsafe {
        let keys = vec![
            (HKEY_CURRENT_USER, w!("SOFTWARE\\Microsoft\\Edge")),
            (HKEY_CURRENT_USER, w!("SOFTWARE\\Microsoft\\EdgeUpdate")),
            (HKEY_LOCAL_MACHINE, w!("SOFTWARE\\Microsoft\\Edge")),
            (HKEY_LOCAL_MACHINE, w!("SOFTWARE\\Microsoft\\EdgeUpdate")),
        ];

        for (root, path) in keys {
            let _ = RegDeleteTreeW(root, path);
        }
    }

    println!("[+] Edge registry cleaned");
}

fn terminate_edge_processes() -> Result<(), String> {
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

                if name.contains("edge") {
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

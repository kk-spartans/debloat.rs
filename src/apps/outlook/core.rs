use crate::apps::outlook::cleanup::remove_outlook_shortcuts;
use crate::apps::outlook::cleanup::remove_outlook_windowsapps_folders;
use crate::apps::outlook::userdata::clean_outlook_user_data;
use crate::apps::outlook::userdata::clean_taskbar_registry;
use std::process::{Command, Stdio};
use tracing::debug;
use windows::Win32::System::Diagnostics::ToolHelp::{
    CreateToolhelp32Snapshot, PROCESSENTRY32W, Process32FirstW, Process32NextW, TH32CS_SNAPPROCESS,
};
use windows::Win32::System::Threading::{OpenProcess, PROCESS_ALL_ACCESS, TerminateProcess};

pub fn remove_outlook() -> Result<(), String> {
    debug!("Terminating Outlook processes...");
    terminate_processes("outlook")?;
    debug!("Waiting for processes to terminate...");
    std::thread::sleep(std::time::Duration::from_secs(2));
    debug!("Removing Outlook Appx packages...");
    remove_outlook_appx();
    debug!("Removing WindowsApps folders...");
    remove_outlook_windowsapps_folders();
    debug!("Removing shortcuts...");
    remove_outlook_shortcuts();
    debug!("Cleaning taskbar registry...");
    clean_taskbar_registry();
    debug!("Cleaning user data...");
    clean_outlook_user_data();
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

fn remove_outlook_appx() {
    let packages = ["Microsoft.Office.Outlook", "Microsoft.OutlookForWindows"];

    for package in packages {
        let _ = Command::new("powershell")
            .args([
                "-NoProfile",
                "-ExecutionPolicy",
                "Bypass",
                "-Command",
                &format!(
                    "Get-AppxPackage *{package}* | Remove-AppxPackage -ErrorAction SilentlyContinue"
                ),
            ])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .output();

        let _ = Command::new("powershell")
            .args(["-NoProfile", "-ExecutionPolicy", "Bypass", "-Command", &format!("Get-AppxProvisionedPackage -Online | Where-Object {{ $_.PackageName -like \"*{package}*\" }} | Remove-AppxProvisionedPackage -Online -ErrorAction SilentlyContinue")])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .output();
    }
}

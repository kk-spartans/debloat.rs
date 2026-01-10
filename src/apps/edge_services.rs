use std::env;
use std::ffi::OsStr;
use std::iter::once;
use std::os::windows::ffi::OsStrExt;
use std::path::Path;
use std::process::{Command, Stdio};
use windows::Win32::System::Services::{
    ControlService, DeleteService, OpenSCManagerW, OpenServiceW, SC_MANAGER_ALL_ACCESS,
    SERVICE_ALL_ACCESS, SERVICE_CONTROL_STOP, SERVICE_STATUS,
};
use windows::core::PCWSTR;

pub fn remove_edge_services() -> Result<(), String> {
    let services = vec!["edgeupdate", "edgeupdatem", "MicrosoftEdgeElevationService"];

    unsafe {
        let sc_manager = OpenSCManagerW(None, None, SC_MANAGER_ALL_ACCESS)
            .map_err(|e| format!("Failed to open service control manager: {e:?}"))?;

        for service_name in services {
            let service_wide = to_wide(service_name);
            let service_pcwstr = PCWSTR(service_wide.as_ptr());

            if let Ok(service_handle) = OpenServiceW(sc_manager, service_pcwstr, SERVICE_ALL_ACCESS)
            {
                let mut status = SERVICE_STATUS::default();
                let _ = ControlService(service_handle, SERVICE_CONTROL_STOP, &raw mut status);

                std::thread::sleep(std::time::Duration::from_millis(500));

                let _ = DeleteService(service_handle);
            }
        }
    }

    Ok(())
}

pub fn create_protective_folders() -> Result<(), String> {
    let program_files_x86 = env::var("ProgramFiles(x86)")
        .or_else(|_| env::var("ProgramFiles"))
        .map_err(|_| "Failed to get ProgramFiles path")?;

    let protective_folders = vec![
        format!("{}\\Microsoft\\Edge", program_files_x86),
        format!("{}\\Microsoft\\Edge\\Application", program_files_x86),
        format!("{}\\Microsoft\\EdgeCore", program_files_x86),
    ];

    for folder in protective_folders {
        let path = Path::new(&folder);

        if std::fs::create_dir_all(path).is_err() {
            continue;
        }

        let _ = Command::new("icacls")
            .args([
                &folder,
                "/inheritance:d",
                "/grant",
                "administrators:F",
                "/deny",
                "SYSTEM:(OI)(CI)(DE,DC)",
                "/deny",
                "Administrators:(OI)(CI)(DE,DC)",
                "/deny",
                "*S-1-5-11:(OI)(CI)(DE,DC)",
            ])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .output();
    }

    Ok(())
}

fn to_wide(s: &str) -> Vec<u16> {
    OsStr::new(s).encode_wide().chain(once(0)).collect()
}

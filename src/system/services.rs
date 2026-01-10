use crate::system::service_helpers::to_wide;
use windows::Win32::System::Services::{
    ChangeServiceConfigW, ControlService, OpenSCManagerW, OpenServiceW, SC_MANAGER_ALL_ACCESS,
    SERVICE_ALL_ACCESS, SERVICE_CONTROL_STOP, SERVICE_DISABLED, SERVICE_ERROR, SERVICE_START_TYPE,
    SERVICE_STATUS,
};
use windows::core::PCWSTR;

pub fn disable_services(services: &[&str]) {
    unsafe {
        let sc_manager = OpenSCManagerW(None, None, SC_MANAGER_ALL_ACCESS).unwrap_or_default();

        for service_name in services {
            let service_wide = to_wide(service_name);
            let service_pcwstr = PCWSTR(service_wide.as_ptr());

            if let Ok(service_handle) = OpenServiceW(sc_manager, service_pcwstr, SERVICE_ALL_ACCESS)
            {
                let mut status = SERVICE_STATUS::default();
                let _ = ControlService(service_handle, SERVICE_CONTROL_STOP, &raw mut status);
                std::thread::sleep(std::time::Duration::from_millis(500));

                let _ = ChangeServiceConfigW(
                    service_handle,
                    windows::Win32::System::Services::ENUM_SERVICE_TYPE::default(),
                    SERVICE_START_TYPE(SERVICE_DISABLED.0),
                    SERVICE_ERROR::default(),
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                );
            }
        }
    }
}

pub fn wpf_tweaks_services() {
    let services = [
        "DiagTrack",
        "dmwappushservice",
        "WSearch",
        "TrkWks",
        "WbioSrvc",
        "RemoteRegistry",
        "RemoteAccess",
        "SharedAccess",
        "TabletInputService",
        "WMPNetworkSvc",
    ];
    disable_services(&services);
}

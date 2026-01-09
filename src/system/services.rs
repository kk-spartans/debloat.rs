use windows::core::w;
use windows::Win32::Foundation::ERROR_SUCCESS;
use windows::Win32::System::Registry::{
    RegCloseKey, RegCreateKeyExW, RegDeleteTreeW, RegOpenKeyExW, RegSetValueExW, HKEY,
    HKEY_CLASSES_ROOT, HKEY_CURRENT_USER, HKEY_LOCAL_MACHINE, KEY_SET_VALUE, REG_DWORD,
    REG_OPTION_NON_VOLATILE,
};
use windows::Win32::System::Services::{
    ChangeServiceConfigW, ControlService, OpenSCManagerW, OpenServiceW, ENUM_SERVICE_TYPE,
    SC_MANAGER_ALL_ACCESS, SERVICE_ALL_ACCESS, SERVICE_CONTROL_STOP, SERVICE_DISABLED,
    SERVICE_ERROR, SERVICE_START_TYPE, SERVICE_STATUS,
};

pub fn wpf_tweaks_services() -> Result<(), String> {
    println!("[*] Disabling unnecessary Windows services...");
    let services = vec![
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

    for service_name in services {
        disable_service(service_name)?;
    }

    Ok(())
}

fn disable_service(service_name: &str) -> Result<(), String> {
    unsafe {
        let sc_manager = OpenSCManagerW(None, None, SC_MANAGER_ALL_ACCESS)
            .map_err(|e| format!("Failed to open service control manager: {e:?}"))?;

        let service_wide = to_wide(service_name);
        let service_pcwstr = windows::core::PCWSTR(service_wide.as_ptr());

        if let Ok(service_handle) = OpenServiceW(sc_manager, service_pcwstr, SERVICE_ALL_ACCESS) {
            // Stop the service
            let mut status = SERVICE_STATUS::default();
            let _ = ControlService(service_handle, SERVICE_CONTROL_STOP, &raw mut status);
            std::thread::sleep(std::time::Duration::from_millis(500));

            // Disable the service
            let _ = ChangeServiceConfigW(
                service_handle,
                ENUM_SERVICE_TYPE::default(),
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
    Ok(())
}

fn to_wide(s: &str) -> Vec<u16> {
    use std::ffi::OsStr;
    use std::iter::once;
    use std::os::windows::ffi::OsStrExt;

    OsStr::new(s).encode_wide().chain(once(0)).collect()
}

pub fn wpf_tweaks_disable_lms1() {
    println!("[*] Disabling location services...");

    unsafe {
        // HKLM\SOFTWARE\Microsoft\Windows NT\CurrentVersion\Sensor\Overrides\{BFA794E4-F964-4FDB-90F6-51056BFE4B44}
        let key_path1 = w!("SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\Sensor\\Overrides\\{BFA794E4-F964-4FDB-90F6-51056BFE4B44}");
        let mut key1 = HKEY::default();
        if RegOpenKeyExW(
            HKEY_LOCAL_MACHINE,
            key_path1,
            Some(0),
            KEY_SET_VALUE,
            &raw mut key1,
        ) == ERROR_SUCCESS
        {
            let value: u32 = 0;
            let _ = RegSetValueExW(
                key1,
                w!("SensorPermissionState"),
                Some(0),
                REG_DWORD,
                Some(&value.to_le_bytes()),
            );
            let _ = RegCloseKey(key1);
        }

        // HKLM\SYSTEM\CurrentControlSet\Services\lfsvc\Service\Start
        let key_path2 = w!("SYSTEM\\CurrentControlSet\\Services\\lfsvc\\Service");
        let mut key2 = HKEY::default();
        if RegOpenKeyExW(
            HKEY_LOCAL_MACHINE,
            key_path2,
            Some(0),
            KEY_SET_VALUE,
            &raw mut key2,
        ) == ERROR_SUCCESS
        {
            let value: u32 = 4;
            let _ = RegSetValueExW(
                key2,
                w!("Start"),
                Some(0),
                REG_DWORD,
                Some(&value.to_le_bytes()),
            );
            let _ = RegCloseKey(key2);
        }
    }

    println!("[+] Location services disabled");
}

pub fn wpf_tweaks_tele() {
    println!("[*] Disabling telemetry...");

    unsafe {
        let key_path = w!("SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Policies\\DataCollection");
        let mut key = HKEY::default();
        if RegOpenKeyExW(
            HKEY_LOCAL_MACHINE,
            key_path,
            Some(0),
            KEY_SET_VALUE,
            &raw mut key,
        ) == ERROR_SUCCESS
        {
            let value0: u32 = 0;
            let value1: u32 = 1;
            let _ = RegSetValueExW(
                key,
                w!("AllowTelemetry"),
                Some(0),
                REG_DWORD,
                Some(&value0.to_le_bytes()),
            );
            let _ = RegSetValueExW(
                key,
                w!("DoNotShowFeedbackNotifications"),
                Some(0),
                REG_DWORD,
                Some(&value1.to_le_bytes()),
            );
            let _ = RegCloseKey(key);
        }
    }

    println!("[+] Telemetry disabled");
}

pub fn wpf_tweaks_disable_bgapps() {
    println!("[*] Disabling background apps...");

    unsafe {
        let key_path =
            w!("SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\BackgroundAccessApplications");
        let mut key = HKEY::default();
        if RegOpenKeyExW(
            HKEY_CURRENT_USER,
            key_path,
            Some(0),
            KEY_SET_VALUE,
            &raw mut key,
        ) == ERROR_SUCCESS
        {
            let value1: u32 = 1;
            let value0: u32 = 0;
            let _ = RegSetValueExW(
                key,
                w!("GlobalUserDisabled"),
                Some(0),
                REG_DWORD,
                Some(&value1.to_le_bytes()),
            );
            let _ = RegSetValueExW(
                key,
                w!("BackgroundAppGlobalToggle"),
                Some(0),
                REG_DWORD,
                Some(&value0.to_le_bytes()),
            );
            let _ = RegCloseKey(key);
        }
    }

    println!("[+] Background apps disabled");
}

pub fn wpf_tweaks_remove_onedrive() {
    println!("[*] Removing OneDrive...");

    // Execute OneDriveSetup.exe /uninstall
    let _ = std::process::Command::new(r"C:\Windows\SysWOW64\OneDriveSetup.exe")
        .args(["/uninstall"])
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .output();

    // Delete CLSID keys
    unsafe {
        let key_path1 = w!("CLSID\\{018D5C66-4533-4307-9B53-224DE2ED1FE6}");
        let _ = RegDeleteTreeW(HKEY_CLASSES_ROOT, key_path1);

        let key_path2 = w!("Wow6432Node\\CLSID\\{018D5C66-4533-4307-9B53-224DE2ED1FE6}");
        let _ = RegDeleteTreeW(HKEY_CLASSES_ROOT, key_path2);

        let key_path3 = w!("Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Desktop\\NameSpace\\{018D5C66-4533-4307-9B53-224DE2ED1FE6}");
        let _ = RegDeleteTreeW(HKEY_CURRENT_USER, key_path3);
    }

    // Remove folders
    let onedrive_folders = vec![
        format!(
            "{}\\OneDrive",
            std::env::var("USERPROFILE").unwrap_or_default()
        ),
        format!(
            "{}\\Microsoft\\OneDrive",
            std::env::var("LOCALAPPDATA").unwrap_or_default()
        ),
        format!(
            "{}\\Microsoft\\OneDrive",
            std::env::var("ProgramData").unwrap_or_default()
        ),
    ];

    for folder in onedrive_folders {
        if std::path::Path::new(&folder).exists() {
            let _ = std::fs::remove_dir_all(&folder);
        }
    }

    println!("[+] OneDrive removed");
}

pub fn wpf_tweaks_recall_off() {
    println!("[*] Disabling recall...");

    unsafe {
        let key_path = w!("SOFTWARE\\Policies\\Microsoft\\Windows\\WindowsAI");
        let mut key = HKEY::default();
        if RegCreateKeyExW(
            HKEY_LOCAL_MACHINE,
            key_path,
            Some(0),
            None,
            REG_OPTION_NON_VOLATILE,
            KEY_SET_VALUE,
            None,
            &raw mut key,
            None,
        ) == ERROR_SUCCESS
        {
            let value: u32 = 1;
            let _ = RegSetValueExW(
                key,
                w!("TurnOffWindowsRecall"),
                Some(0),
                REG_DWORD,
                Some(&value.to_le_bytes()),
            );
            let _ = RegCloseKey(key);
        }

        // HKLM\SOFTWARE\Policies\Microsoft\Windows\WindowsAI
        let key_path2 = w!("SOFTWARE\\Policies\\Microsoft\\Windows\\WindowsAI");
        let mut key2 = HKEY::default();
        if RegCreateKeyExW(
            HKEY_LOCAL_MACHINE,
            key_path2,
            Some(0),
            None,
            REG_OPTION_NON_VOLATILE,
            KEY_SET_VALUE,
            None,
            &raw mut key2,
            None,
        ) == ERROR_SUCCESS
        {
            let value: u32 = 1;
            let _ = RegSetValueExW(
                key2,
                w!("TurnOffWindowsRecall"),
                Some(0),
                REG_DWORD,
                Some(&value.to_le_bytes()),
            );
            let _ = RegCloseKey(key2);
        }

        // HKLM\SYSTEM\CurrentControlSet\Control\FeatureManagement\Overrides\0\2093230218
        let key_path3 =
            w!("SYSTEM\\CurrentControlSet\\Control\\FeatureManagement\\Overrides\\0\\2093230218");
        let mut key3 = HKEY::default();
        if RegCreateKeyExW(
            HKEY_LOCAL_MACHINE,
            key_path3,
            Some(0),
            None,
            REG_OPTION_NON_VOLATILE,
            KEY_SET_VALUE,
            None,
            &raw mut key3,
            None,
        ) == ERROR_SUCCESS
        {
            let value2: u32 = 2;
            let _ = RegSetValueExW(
                key3,
                w!("EnabledState"),
                Some(0),
                REG_DWORD,
                Some(&value2.to_le_bytes()),
            );
            let _ = RegCloseKey(key3);
        }
    }

    println!("[+] Recall disabled");
}

pub fn wpf_tweaks_disable_wpbt_execution() {
    println!("[*] Disabling WPBT execution...");

    unsafe {
        let key_path = w!("SYSTEM\\CurrentControlSet\\Control\\DeviceGuard\\Scenarios\\WindowsPlatformBinaryTable");
        let mut key = HKEY::default();
        if RegOpenKeyExW(
            HKEY_LOCAL_MACHINE,
            key_path,
            Some(0),
            KEY_SET_VALUE,
            &raw mut key,
        ) == ERROR_SUCCESS
        {
            let value: u32 = 0;
            let _ = RegSetValueExW(
                key,
                w!("Enabled"),
                Some(0),
                REG_DWORD,
                Some(&value.to_le_bytes()),
            );
            let _ = RegCloseKey(key);
        }
    }

    println!("[+] WPBT execution disabled");
}

pub fn wpf_tweaks_dvr() {
    println!("[*] Disabling DVR...");

    unsafe {
        let key_path = w!("System\\GameConfigStore");
        let mut key = HKEY::default();
        if RegOpenKeyExW(
            HKEY_CURRENT_USER,
            key_path,
            Some(0),
            KEY_SET_VALUE,
            &raw mut key,
        ) == ERROR_SUCCESS
        {
            let value: u32 = 0;
            let _ = RegSetValueExW(
                key,
                w!("GameDVR_Enabled"),
                Some(0),
                REG_DWORD,
                Some(&value.to_le_bytes()),
            );
            let _ = RegCloseKey(key);
        }

        // HKLM\SOFTWARE\Microsoft\PolicyManager\default\ApplicationManagement\AllowGameDVR
        let key_path2 =
            w!("SOFTWARE\\Microsoft\\PolicyManager\\default\\ApplicationManagement\\AllowGameDVR");
        let mut key2 = HKEY::default();
        if RegOpenKeyExW(
            HKEY_LOCAL_MACHINE,
            key_path2,
            Some(0),
            KEY_SET_VALUE,
            &raw mut key2,
        ) == ERROR_SUCCESS
        {
            let value: u32 = 0;
            let _ = RegSetValueExW(
                key2,
                w!("value"),
                Some(0),
                REG_DWORD,
                Some(&value.to_le_bytes()),
            );
            let _ = RegCloseKey(key2);
        }
    }

    println!("[+] DVR disabled");
}

pub fn wpf_tweaks_disable_explorer_auto_discovery() {
    println!("[*] Disabling explorer auto discovery...");

    unsafe {
        let key_path = w!("SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Advanced");
        let mut key = HKEY::default();
        if RegOpenKeyExW(
            HKEY_CURRENT_USER,
            key_path,
            Some(0),
            KEY_SET_VALUE,
            &raw mut key,
        ) == ERROR_SUCCESS
        {
            let value: u32 = 1;
            let _ = RegSetValueExW(
                key,
                w!("DisablePreviewHandlers"),
                Some(0),
                REG_DWORD,
                Some(&value.to_le_bytes()),
            );
            let _ = RegSetValueExW(
                key,
                w!("DisableThumbnailCache"),
                Some(0),
                REG_DWORD,
                Some(&value.to_le_bytes()),
            );
            let _ = RegCloseKey(key);
        }
    }

    println!("[+] Explorer auto discovery disabled");
}

use windows::core::w;
use windows::Win32::Foundation::ERROR_SUCCESS;
use windows::Win32::System::Registry::{
    RegCloseKey, RegOpenKeyExW, RegSetValueExW, HKEY, HKEY_LOCAL_MACHINE, KEY_SET_VALUE, REG_DWORD,
};
use windows::Win32::System::Services::{
    CloseServiceHandle, ControlService, OpenSCManagerW, OpenServiceW, SC_MANAGER_ALL_ACCESS,
    SERVICE_ALL_ACCESS, SERVICE_CONTROL_STOP,
};

pub fn disable_telemetry() {
    println!("[*] Disabling telemetry...");

    // Stop DiagTrack service
    unsafe {
        let Ok(sc_manager) = OpenSCManagerW(None, None, SC_MANAGER_ALL_ACCESS) else {
            eprintln!("Failed to open service control manager");
            return;
        };

        let Ok(service) = OpenServiceW(sc_manager, w!("DiagTrack"), SERVICE_ALL_ACCESS) else {
            eprintln!("Failed to open DiagTrack service");
            let _ = CloseServiceHandle(sc_manager);
            return;
        };

        let _ = ControlService(service, SERVICE_CONTROL_STOP, std::ptr::null_mut());

        let _ = CloseServiceHandle(service);
        let _ = CloseServiceHandle(sc_manager);
    }

    // Disable telemetry via registry
    unsafe {
        // HKLM\SOFTWARE\Policies\Microsoft\Windows\DataCollection
        let key_path1 = w!("SOFTWARE\\Policies\\Microsoft\\Windows\\DataCollection");
        let mut key1 = HKEY::default();
        if RegOpenKeyExW(
            HKEY_LOCAL_MACHINE,
            key_path1,
            Some(0),
            KEY_SET_VALUE,
            &raw mut key1,
        ) == ERROR_SUCCESS
        {
            let value0: u32 = 0;
            let _ = RegSetValueExW(
                key1,
                w!("AllowTelemetry"),
                Some(0),
                REG_DWORD,
                Some(&value0.to_le_bytes()),
            );
            let value1: u32 = 1;
            let _ = RegSetValueExW(
                key1,
                w!("DoNotShowFeedbackNotifications"),
                Some(0),
                REG_DWORD,
                Some(&value1.to_le_bytes()),
            );
            let value2: u32 = 0;
            let _ = RegSetValueExW(
                key1,
                w!("MaxTelemetryAllowed"),
                Some(0),
                REG_DWORD,
                Some(&value2.to_le_bytes()),
            );
            let _ = RegCloseKey(key1);
        }

        // HKLM\SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\DataCollection
        let key_path2 =
            w!("SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Policies\\DataCollection");
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
                w!("AllowTelemetry"),
                Some(0),
                REG_DWORD,
                Some(&value.to_le_bytes()),
            );
            let _ = RegCloseKey(key2);
        }
    }

    println!("[+] Telemetry disabled");
}

pub fn disable_bing() {
    println!("[*] Disabling Bing search...");

    unsafe {
        // HKCU\Software\Microsoft\Windows\CurrentVersion\Search
        let key_path1 = w!("SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Search");
        let mut key1 = HKEY::default();
        if RegOpenKeyExW(
            windows::Win32::System::Registry::HKEY_CURRENT_USER,
            key_path1,
            Some(0),
            KEY_SET_VALUE,
            &raw mut key1,
        ) == ERROR_SUCCESS
        {
            let value: u32 = 0;
            let _ = RegSetValueExW(
                key1,
                w!("BingSearchEnabled"),
                Some(0),
                REG_DWORD,
                Some(&value.to_le_bytes()),
            );
            let _ = RegSetValueExW(
                key1,
                w!("CortanaConsent"),
                Some(0),
                REG_DWORD,
                Some(&value.to_le_bytes()),
            );
            let _ = RegCloseKey(key1);
        }

        // HKLM\SOFTWARE\Policies\Microsoft\Windows\Windows Search
        let key_path2 = w!("SOFTWARE\\Policies\\Microsoft\\Windows\\Windows Search");
        let mut key2 = HKEY::default();
        if RegOpenKeyExW(
            HKEY_LOCAL_MACHINE,
            key_path2,
            Some(0),
            KEY_SET_VALUE,
            &raw mut key2,
        ) == ERROR_SUCCESS
        {
            let value0: u32 = 0;
            let value1: u32 = 1;
            let _ = RegSetValueExW(
                key2,
                w!("DisableWebSearch"),
                Some(0),
                REG_DWORD,
                Some(&value1.to_le_bytes()),
            );
            let _ = RegSetValueExW(
                key2,
                w!("ConnectedSearchUseWeb"),
                Some(0),
                REG_DWORD,
                Some(&value0.to_le_bytes()),
            );
            let _ = RegCloseKey(key2);
        }
    }

    println!("[+] Bing search disabled");
}

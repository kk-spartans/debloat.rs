use windows::Win32::Foundation::ERROR_SUCCESS;
use windows::Win32::System::Registry::{
    HKEY, HKEY_CLASSES_ROOT, HKEY_LOCAL_MACHINE, HKEY_CURRENT_USER, KEY_SET_VALUE, REG_DWORD,
    REG_OPTION_NON_VOLATILE, RegCloseKey, RegCreateKeyExW, RegDeleteTreeW, RegOpenKeyExW,
    RegSetValueExW,
};
use windows::core::w;

pub fn disable_location_services() {
    unsafe {
        let key_path1 = w!("SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\Sensor\\Overrides\\{BFA794E4-F964-4FDB-90F6-51056BFE4B44}");
        let mut key1 = HKEY::default();
        if RegOpenKeyExW(HKEY_LOCAL_MACHINE, key_path1, Some(0), KEY_SET_VALUE, &raw mut key1) == ERROR_SUCCESS {
            let value: u32 = 0;
            let _ = RegSetValueExW(key1, w!("SensorPermissionState"), Some(0), REG_DWORD, Some(&value.to_le_bytes()));
            let _ = RegCloseKey(key1);
        }

        let key_path2 = w!("SYSTEM\\CurrentControlSet\\Services\\lfsvc\\Service");
        let mut key2 = HKEY::default();
        if RegOpenKeyExW(HKEY_LOCAL_MACHINE, key_path2, Some(0), KEY_SET_VALUE, &raw mut key2) == ERROR_SUCCESS {
            let value: u32 = 4;
            let _ = RegSetValueExW(key2, w!("Start"), Some(0), REG_DWORD, Some(&value.to_le_bytes()));
            let _ = RegCloseKey(key2);
        }
    }
}

pub fn disable_telemetry_registry() {
    unsafe {
        let key_path = w!("SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Policies\\DataCollection");
        let mut key = HKEY::default();
        if RegOpenKeyExW(HKEY_LOCAL_MACHINE, key_path, Some(0), KEY_SET_VALUE, &raw mut key) == ERROR_SUCCESS {
            let value0: u32 = 0;
            let _ = RegSetValueExW(key, w!("AllowTelemetry"), Some(0), REG_DWORD, Some(&value0.to_le_bytes()));
            let _ = RegSetValueExW(key, w!("DoNotShowFeedbackNotifications"), Some(0), REG_DWORD, Some(&1u32.to_le_bytes()));
            let _ = RegCloseKey(key);
        }
    }
}

pub fn disable_background_apps() {
    unsafe {
        let key_path = w!("SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\BackgroundAccessApplications");
        let mut key = HKEY::default();
        if RegOpenKeyExW(HKEY_CURRENT_USER, key_path, Some(0), KEY_SET_VALUE, &raw mut key) == ERROR_SUCCESS {
            let value1: u32 = 1;
            let value0: u32 = 0;
            let _ = RegSetValueExW(key, w!("GlobalUserDisabled"), Some(0), REG_DWORD, Some(&value1.to_le_bytes()));
            let _ = RegSetValueExW(key, w!("BackgroundAppGlobalToggle"), Some(0), REG_DWORD, Some(&value0.to_le_bytes()));
            let _ = RegCloseKey(key);
        }
    }
}

pub fn disable_recall() {
    unsafe {
        let key_path = w!("SOFTWARE\\Policies\\Microsoft\\Windows\\WindowsAI");
        let mut key = HKEY::default();
        if RegCreateKeyExW(HKEY_LOCAL_MACHINE, key_path, Some(0), None, REG_OPTION_NON_VOLATILE, KEY_SET_VALUE, None, &raw mut key, None) == ERROR_SUCCESS {
            let value: u32 = 1;
            let _ = RegSetValueExW(key, w!("TurnOffWindowsRecall"), Some(0), REG_DWORD, Some(&value.to_le_bytes()));
            let _ = RegCloseKey(key);
        }

        let key_path3 = w!("SYSTEM\\CurrentControlSet\\Control\\FeatureManagement\\Overrides\\0\\2093230218");
        let mut key3 = HKEY::default();
        if RegCreateKeyExW(HKEY_LOCAL_MACHINE, key_path3, Some(0), None, REG_OPTION_NON_VOLATILE, KEY_SET_VALUE, None, &raw mut key3, None) == ERROR_SUCCESS {
            let value2: u32 = 2;
            let _ = RegSetValueExW(key3, w!("EnabledState"), Some(0), REG_DWORD, Some(&value2.to_le_bytes()));
            let _ = RegCloseKey(key3);
        }
    }
}

pub fn disable_wpbt() {
    unsafe {
        let key_path = w!("SYSTEM\\CurrentControlSet\\Control\\DeviceGuard\\Scenarios\\WindowsPlatformBinaryTable");
        let mut key = HKEY::default();
        if RegOpenKeyExW(HKEY_LOCAL_MACHINE, key_path, Some(0), KEY_SET_VALUE, &raw mut key) == ERROR_SUCCESS {
            let value: u32 = 0;
            let _ = RegSetValueExW(key, w!("Enabled"), Some(0), REG_DWORD, Some(&value.to_le_bytes()));
            let _ = RegCloseKey(key);
        }
    }
}

pub fn disable_explorer_auto_discovery() {
    unsafe {
        let key_path = w!("SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Advanced");
        let mut key = HKEY::default();
        if RegOpenKeyExW(HKEY_CURRENT_USER, key_path, Some(0), KEY_SET_VALUE, &raw mut key) == ERROR_SUCCESS {
            let value: u32 = 1;
            let _ = RegSetValueExW(key, w!("DisablePreviewHandlers"), Some(0), REG_DWORD, Some(&value.to_le_bytes()));
            let _ = RegSetValueExW(key, w!("DisableThumbnailCache"), Some(0), REG_DWORD, Some(&value.to_le_bytes()));
            let _ = RegCloseKey(key);
        }
    }
}

pub fn remove_onedrive_registry() {
    unsafe {
        let key_path1 = w!("CLSID\\{018D5C66-4533-4307-9B53-224DE2ED1FE6}");
        let _ = RegDeleteTreeW(HKEY_CLASSES_ROOT, key_path1);

        let key_path2 = w!("Wow6432Node\\CLSID\\{018D5C66-4533-4307-9B53-224DE2ED1FE6}");
        let _ = RegDeleteTreeW(HKEY_CLASSES_ROOT, key_path2);

        let key_path3 = w!("Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Desktop\\NameSpace\\{018D5C66-4533-4307-9B53-224DE2ED1FE6}");
        let _ = RegDeleteTreeW(HKEY_CURRENT_USER, key_path3);
    }
}

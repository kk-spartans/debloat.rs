use windows::Win32::Foundation::ERROR_SUCCESS;
use windows::Win32::System::Registry::{
    HKEY, HKEY_CURRENT_USER, HKEY_LOCAL_MACHINE, KEY_SET_VALUE, REG_DWORD, REG_OPTION_NON_VOLATILE,
    RegCloseKey, RegCreateKeyExW, RegOpenKeyExW, RegSetValueExW,
};
use windows::core::w;

pub fn disable_action_center() {
    unsafe {
        let key_path1 = w!("SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\PushNotifications");
        let mut key1 = HKEY::default();
        if RegOpenKeyExW(
            HKEY_CURRENT_USER,
            key_path1,
            Some(0),
            KEY_SET_VALUE,
            &raw mut key1,
        ) == ERROR_SUCCESS
        {
            let value: u32 = 0;
            let _ = RegSetValueExW(
                key1,
                w!("ToastEnabled"),
                Some(0),
                REG_DWORD,
                Some(&value.to_le_bytes()),
            );
            let _ = RegCloseKey(key1);
        }

        let key_path2 = w!("SOFTWARE\\Policies\\Microsoft\\Windows\\Explorer");
        let mut key2 = HKEY::default();
        if RegCreateKeyExW(
            HKEY_CURRENT_USER,
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
                w!("DisableNotificationCenter"),
                Some(0),
                REG_DWORD,
                Some(&value.to_le_bytes()),
            );
            let _ = RegCloseKey(key2);
        }
    }
}

pub fn disable_consumer_features() {
    unsafe {
        let key_path = w!("SOFTWARE\\Policies\\Microsoft\\Windows\\CloudContent");
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
                w!("DisableWindowsConsumerFeatures"),
                Some(0),
                REG_DWORD,
                Some(&value.to_le_bytes()),
            );
            let _ = RegSetValueExW(
                key,
                w!("DisableCloudOptimizedContent"),
                Some(0),
                REG_DWORD,
                Some(&value.to_le_bytes()),
            );
            let _ = RegCloseKey(key);
        }
    }
}

pub fn remove_copilot() {
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
            let value: u32 = 0;
            let _ = RegSetValueExW(
                key,
                w!("ShowCopilotButton"),
                Some(0),
                REG_DWORD,
                Some(&value.to_le_bytes()),
            );
            let _ = RegCloseKey(key);
        }
    }
}

pub fn configure_wifi() {
    unsafe {
        let key_path1 = w!("SOFTWARE\\Microsoft\\WcmSvc\\wifinetworkmanager\\config");
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
                w!("AutoConnectAllowedOEM"),
                Some(0),
                REG_DWORD,
                Some(&value.to_le_bytes()),
            );
            let _ = RegCloseKey(key1);
        }

        let key_path2 =
            w!("SOFTWARE\\Microsoft\\PolicyManager\\default\\WiFi\\AllowWiFiHotspotReporting");
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
}

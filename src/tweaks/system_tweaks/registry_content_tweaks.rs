use windows::Win32::Foundation::ERROR_SUCCESS;
use windows::Win32::System::Registry::{
    HKEY, HKEY_LOCAL_MACHINE, HKEY_CURRENT_USER, KEY_SET_VALUE, REG_DWORD, REG_OPTION_NON_VOLATILE,
    REG_SZ, RegCloseKey, RegCreateKeyExW, RegOpenKeyExW, RegSetValueExW,
};
use windows::core::w;

pub fn disable_desktop_spotlight() {
    unsafe {
        let key_path = w!("SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\ContentDeliveryManager");
        let mut key = HKEY::default();
        if RegOpenKeyExW(HKEY_CURRENT_USER, key_path, Some(0), KEY_SET_VALUE, &raw mut key) == ERROR_SUCCESS {
            let value: u32 = 0;
            let _ = RegSetValueExW(key, w!("RotatingLockScreenEnabled"), Some(0), REG_DWORD, Some(&value.to_le_bytes()));
            let _ = RegSetValueExW(key, w!("RotatingLockScreenOverlayEnabled"), Some(0), REG_DWORD, Some(&value.to_le_bytes()));
            let _ = RegSetValueExW(key, w!("SubscribedContent-338393Enabled"), Some(0), REG_DWORD, Some(&value.to_le_bytes()));
            let _ = RegCloseKey(key);
        }
    }
}

pub fn disable_settings_365_ads() {
    unsafe {
        let key_path = w!("SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\ContentDeliveryManager");
        let mut key = HKEY::default();
        if RegOpenKeyExW(HKEY_CURRENT_USER, key_path, Some(0), KEY_SET_VALUE, &raw mut key) == ERROR_SUCCESS {
            let value: u32 = 0;
            let _ = RegSetValueExW(key, w!("SubscribedContent-310093Enabled"), Some(0), REG_DWORD, Some(&value.to_le_bytes()));
            let _ = RegSetValueExW(key, w!("SubscribedContent-338388Enabled"), Some(0), REG_DWORD, Some(&value.to_le_bytes()));
            let _ = RegCloseKey(key);
        }
    }
}

pub fn disable_settings_home() {
    unsafe {
        let key_path = w!("SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Policies\\Explorer");
        let mut key = HKEY::default();
        if RegCreateKeyExW(
            HKEY_CURRENT_USER,
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
            let value = "hide:home\0";
            let wide: Vec<u16> = value.encode_utf16().collect();
            let data: Vec<u8> = wide.iter().flat_map(|&c| c.to_le_bytes()).collect();
            let _ = RegSetValueExW(key, w!("SettingsPageVisibility"), Some(0), REG_SZ, Some(&data));
            let _ = RegCloseKey(key);
        }
    }
}

pub fn disable_paint_ai() {
    unsafe {
        let key_path1 = w!("SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Policies\\Paint");
        let mut key1 = HKEY::default();
        if RegCreateKeyExW(HKEY_CURRENT_USER, key_path1, Some(0), None, REG_OPTION_NON_VOLATILE, KEY_SET_VALUE, None, &raw mut key1, None) == ERROR_SUCCESS {
            let value: u32 = 1;
            let _ = RegSetValueExW(key1, w!("DisableAIFeatures"), Some(0), REG_DWORD, Some(&value.to_le_bytes()));
            let _ = RegCloseKey(key1);
        }

        let key_path2 = w!("SOFTWARE\\Policies\\Microsoft\\Windows\\Paint");
        let mut key2 = HKEY::default();
        if RegCreateKeyExW(HKEY_LOCAL_MACHINE, key_path2, Some(0), None, REG_OPTION_NON_VOLATILE, KEY_SET_VALUE, None, &raw mut key2, None) == ERROR_SUCCESS {
            let value: u32 = 1;
            let _ = RegSetValueExW(key2, w!("DisableAIFeatures"), Some(0), REG_DWORD, Some(&value.to_le_bytes()));
            let _ = RegCloseKey(key2);
        }
    }
}

pub fn disable_notepad_ai() {
    unsafe {
        let key_path1 = w!("SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Policies\\Notepad");
        let mut key1 = HKEY::default();
        if RegCreateKeyExW(HKEY_CURRENT_USER, key_path1, Some(0), None, REG_OPTION_NON_VOLATILE, KEY_SET_VALUE, None, &raw mut key1, None) == ERROR_SUCCESS {
            let value: u32 = 1;
            let _ = RegSetValueExW(key1, w!("DisableAIFeatures"), Some(0), REG_DWORD, Some(&value.to_le_bytes()));
            let _ = RegCloseKey(key1);
        }

        let key_path2 = w!("SOFTWARE\\Policies\\Microsoft\\Windows\\Notepad");
        let mut key2 = HKEY::default();
        if RegCreateKeyExW(HKEY_LOCAL_MACHINE, key_path2, Some(0), None, REG_OPTION_NON_VOLATILE, KEY_SET_VALUE, None, &raw mut key2, None) == ERROR_SUCCESS {
            let value: u32 = 1;
            let _ = RegSetValueExW(key2, w!("DisableAIFeatures"), Some(0), REG_DWORD, Some(&value.to_le_bytes()));
            let _ = RegCloseKey(key2);
        }
    }
}

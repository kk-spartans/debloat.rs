use windows::Win32::Foundation::ERROR_SUCCESS;
use windows::Win32::System::Registry::{
    HKEY, HKEY_CURRENT_USER, HKEY_LOCAL_MACHINE, KEY_SET_VALUE, REG_DWORD, RegCloseKey,
    RegOpenKeyExW, RegSetValueExW,
};
use windows::core::w;

pub fn explorer_to_this_pc() {
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
                w!("LaunchTo"),
                Some(0),
                REG_DWORD,
                Some(&value.to_le_bytes()),
            );
            let _ = RegCloseKey(key);
        }
    }
}

pub fn disable_dvr() {
    unsafe {
        let key_path1 = w!("System\\GameConfigStore");
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
                w!("GameDVR_Enabled"),
                Some(0),
                REG_DWORD,
                Some(&value.to_le_bytes()),
            );
            let _ = RegCloseKey(key1);
        }

        let key_path2 = w!("SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\GameDVR");
        let mut key2 = HKEY::default();
        if RegOpenKeyExW(
            HKEY_CURRENT_USER,
            key_path2,
            Some(0),
            KEY_SET_VALUE,
            &raw mut key2,
        ) == ERROR_SUCCESS
        {
            let value: u32 = 0;
            let _ = RegSetValueExW(
                key2,
                w!("AppCaptureEnabled"),
                Some(0),
                REG_DWORD,
                Some(&value.to_le_bytes()),
            );
            let _ = RegCloseKey(key2);
        }

        let key_path3 =
            w!("SOFTWARE\\Microsoft\\PolicyManager\\default\\ApplicationManagement\\AllowGameDVR");
        let mut key3 = HKEY::default();
        if RegOpenKeyExW(
            HKEY_LOCAL_MACHINE,
            key_path3,
            Some(0),
            KEY_SET_VALUE,
            &raw mut key3,
        ) == ERROR_SUCCESS
        {
            let value: u32 = 0;
            let _ = RegSetValueExW(
                key3,
                w!("value"),
                Some(0),
                REG_DWORD,
                Some(&value.to_le_bytes()),
            );
            let _ = RegCloseKey(key3);
        }
    }
}

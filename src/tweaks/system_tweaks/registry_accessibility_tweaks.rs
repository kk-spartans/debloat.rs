use windows::Win32::Foundation::ERROR_SUCCESS;
use windows::Win32::System::Registry::{
    HKEY, HKEY_CURRENT_USER, KEY_SET_VALUE, REG_SZ, RegCloseKey, RegOpenKeyExW, RegSetValueExW,
};
use windows::core::w;

pub fn disable_mouse_acceleration() {
    unsafe {
        let key_path = w!("Control Panel\\Mouse");
        let mut key = HKEY::default();
        if RegOpenKeyExW(
            HKEY_CURRENT_USER,
            key_path,
            Some(0),
            KEY_SET_VALUE,
            &raw mut key,
        ) == ERROR_SUCCESS
        {
            let value_bytes = "0".as_bytes();
            let _ = RegSetValueExW(key, w!("MouseSpeed"), Some(0), REG_SZ, Some(value_bytes));
            let _ = RegSetValueExW(
                key,
                w!("MouseThreshold1"),
                Some(0),
                REG_SZ,
                Some("0".as_bytes()),
            );
            let _ = RegSetValueExW(
                key,
                w!("MouseThreshold2"),
                Some(0),
                REG_SZ,
                Some("0".as_bytes()),
            );
            let _ = RegCloseKey(key);
        }
    }
}

pub fn disable_sticky_keys() {
    unsafe {
        let key_path1 = w!("Control Panel\\Accessibility\\StickyKeys");
        let mut key1 = HKEY::default();
        if RegOpenKeyExW(
            HKEY_CURRENT_USER,
            key_path1,
            Some(0),
            KEY_SET_VALUE,
            &raw mut key1,
        ) == ERROR_SUCCESS
        {
            let value = "506\0";
            let wide: Vec<u16> = value.encode_utf16().collect();
            let data: Vec<u8> = wide.iter().flat_map(|&c| c.to_le_bytes()).collect();
            let _ = RegSetValueExW(key1, w!("Flags"), Some(0), REG_SZ, Some(&data));
            let _ = RegCloseKey(key1);
        }

        let key_path2 = w!("Control Panel\\Accessibility\\Keyboard Response");
        let mut key2 = HKEY::default();
        if RegOpenKeyExW(
            HKEY_CURRENT_USER,
            key_path2,
            Some(0),
            KEY_SET_VALUE,
            &raw mut key2,
        ) == ERROR_SUCCESS
        {
            let value = "122\0";
            let wide: Vec<u16> = value.encode_utf16().collect();
            let data: Vec<u8> = wide.iter().flat_map(|&c| c.to_le_bytes()).collect();
            let _ = RegSetValueExW(key2, w!("Flags"), Some(0), REG_SZ, Some(&data));
            let _ = RegCloseKey(key2);
        }

        let key_path3 = w!("Control Panel\\Accessibility\\ToggleKeys");
        let mut key3 = HKEY::default();
        if RegOpenKeyExW(
            HKEY_CURRENT_USER,
            key_path3,
            Some(0),
            KEY_SET_VALUE,
            &raw mut key3,
        ) == ERROR_SUCCESS
        {
            let value = "58\0";
            let wide: Vec<u16> = value.encode_utf16().collect();
            let data: Vec<u8> = wide.iter().flat_map(|&c| c.to_le_bytes()).collect();
            let _ = RegSetValueExW(key3, w!("Flags"), Some(0), REG_SZ, Some(&data));
            let _ = RegCloseKey(key3);
        }
    }
}

use windows::core::w;
use windows::Win32::Foundation::ERROR_SUCCESS;
use windows::Win32::System::Registry::{
    RegCloseKey, RegCreateKeyExW, RegOpenKeyExW, RegSetValueExW, HKEY, HKEY_CURRENT_USER,
    HKEY_LOCAL_MACHINE, KEY_SET_VALUE, REG_DWORD, REG_OPTION_NON_VOLATILE, REG_SZ,
};

pub fn disable_dvr() {
    println!("[*] Disabling DVR...");

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

    println!("[+] DVR disabled");
}

pub fn explorer_to_this_pc() {
    println!("[*] Setting Explorer to open to This PC...");

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
            let value: u32 = 1; // 1 for This PC
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

    println!("[+] Explorer set to open to This PC");
}

pub fn disable_mouse_acceleration() {
    println!("[*] Disabling mouse acceleration...");

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
            let value: &str = "0";
            let value_bytes = value.as_bytes();
            let _ = RegSetValueExW(key, w!("MouseSpeed"), Some(0), REG_SZ, Some(value_bytes));

            let mouse_threshold1: &str = "0";
            let mouse_threshold1_bytes = mouse_threshold1.as_bytes();
            let _ = RegSetValueExW(
                key,
                w!("MouseThreshold1"),
                Some(0),
                REG_SZ,
                Some(mouse_threshold1_bytes),
            );

            let mouse_threshold2: &str = "0";
            let mouse_threshold2_bytes = mouse_threshold2.as_bytes();
            let _ = RegSetValueExW(
                key,
                w!("MouseThreshold2"),
                Some(0),
                REG_SZ,
                Some(mouse_threshold2_bytes),
            );

            let _ = RegCloseKey(key);
        }
    }

    println!("[+] Mouse acceleration disabled");
}

pub fn disable_desktop_spotlight() {
    println!("[*] Disabling desktop spotlight...");

    unsafe {
        let key_path = w!("SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\ContentDeliveryManager");
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
                w!("RotatingLockScreenEnabled"),
                Some(0),
                REG_DWORD,
                Some(&value.to_le_bytes()),
            );
            let _ = RegSetValueExW(
                key,
                w!("RotatingLockScreenOverlayEnabled"),
                Some(0),
                REG_DWORD,
                Some(&value.to_le_bytes()),
            );
            let _ = RegSetValueExW(
                key,
                w!("SubscribedContent-338393Enabled"),
                Some(0),
                REG_DWORD,
                Some(&value.to_le_bytes()),
            );
            let _ = RegCloseKey(key);
        }
    }

    println!("[+] Desktop spotlight disabled");
}

pub fn disable_settings_365_ads() {
    println!("[*] Disabling 365 ads in settings...");

    unsafe {
        let key_path = w!("SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\ContentDeliveryManager");
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
                w!("SubscribedContent-310093Enabled"),
                Some(0),
                REG_DWORD,
                Some(&value.to_le_bytes()),
            );
            let _ = RegSetValueExW(
                key,
                w!("SubscribedContent-338388Enabled"),
                Some(0),
                REG_DWORD,
                Some(&value.to_le_bytes()),
            );
            let _ = RegCloseKey(key);
        }
    }

    println!("[+] 365 ads disabled");
}

pub fn disable_settings_home() {
    println!("[*] Disabling settings home...");

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
            let _ = RegSetValueExW(
                key,
                w!("SettingsPageVisibility"),
                Some(0),
                REG_SZ,
                Some(&data),
            );
            let _ = RegCloseKey(key);
        }
    }

    println!("[+] Settings home disabled");
}

pub fn disable_paint_ai() {
    println!("[*] Disabling Paint AI...");

    unsafe {
        // HKCU\Software\Microsoft\Windows\CurrentVersion\Policies\Paint
        let key_path1 = w!("SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Policies\\Paint");
        let mut key1 = HKEY::default();
        if RegCreateKeyExW(
            HKEY_CURRENT_USER,
            key_path1,
            Some(0),
            None,
            REG_OPTION_NON_VOLATILE,
            KEY_SET_VALUE,
            None,
            &raw mut key1,
            None,
        ) == ERROR_SUCCESS
        {
            let value: u32 = 1;
            let _ = RegSetValueExW(
                key1,
                w!("DisableAIFeatures"),
                Some(0),
                REG_DWORD,
                Some(&value.to_le_bytes()),
            );
            let _ = RegCloseKey(key1);
        }

        // HKLM\SOFTWARE\Policies\Microsoft\Windows\Paint
        let key_path2 = w!("SOFTWARE\\Policies\\Microsoft\\Windows\\Paint");
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
                w!("DisableAIFeatures"),
                Some(0),
                REG_DWORD,
                Some(&value.to_le_bytes()),
            );
            let _ = RegCloseKey(key2);
        }
    }

    println!("[+] Paint AI disabled");
}

pub fn disable_notepad_ai() {
    println!("[*] Disabling Notepad AI...");

    unsafe {
        // HKCU\Software\Microsoft\Windows\CurrentVersion\Policies\Notepad
        let key_path1 = w!("SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Policies\\Notepad");
        let mut key1 = HKEY::default();
        if RegCreateKeyExW(
            HKEY_CURRENT_USER,
            key_path1,
            Some(0),
            None,
            REG_OPTION_NON_VOLATILE,
            KEY_SET_VALUE,
            None,
            &raw mut key1,
            None,
        ) == ERROR_SUCCESS
        {
            let value: u32 = 1;
            let _ = RegSetValueExW(
                key1,
                w!("DisableAIFeatures"),
                Some(0),
                REG_DWORD,
                Some(&value.to_le_bytes()),
            );
            let _ = RegCloseKey(key1);
        }

        // HKLM\SOFTWARE\Policies\Microsoft\Windows\Notepad
        let key_path2 = w!("SOFTWARE\\Policies\\Microsoft\\Windows\\Notepad");
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
                w!("DisableAIFeatures"),
                Some(0),
                REG_DWORD,
                Some(&value.to_le_bytes()),
            );
            let _ = RegCloseKey(key2);
        }
    }

    println!("[+] Notepad AI disabled");
}

pub fn disable_sticky_keys() {
    println!("[*] Disabling sticky keys...");

    unsafe {
        // HKCU\Control Panel\Accessibility\StickyKeys
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

        // HKCU\Control Panel\Accessibility\Keyboard Response
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

        // HKCU\Control Panel\Accessibility\ToggleKeys
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

    println!("[+] Sticky keys disabled");
}

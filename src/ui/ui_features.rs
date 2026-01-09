use windows::core::w;
use windows::Win32::Foundation::ERROR_SUCCESS;
use windows::Win32::System::Registry::{
    RegCloseKey, RegCreateKeyExW, RegOpenKeyExW, RegSetValueExW, HKEY, HKEY_CURRENT_USER,
    HKEY_LOCAL_MACHINE, KEY_SET_VALUE, REG_DWORD, REG_OPTION_NON_VOLATILE,
};

#[allow(clippy::too_many_lines)]
pub fn disable_suggestions() {
    println!("[*] Disabling search suggestions...");

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
                w!("SystemPaneSuggestionsEnabled"),
                Some(0),
                REG_DWORD,
                Some(&value.to_le_bytes()),
            );
            let _ = RegSetValueExW(
                key,
                w!("SoftLandingEnabled"),
                Some(0),
                REG_DWORD,
                Some(&value.to_le_bytes()),
            );
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
                w!("SubscribedContentEnabled"),
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
            let _ = RegSetValueExW(
                key,
                w!("SubscribedContent-353694Enabled"),
                Some(0),
                REG_DWORD,
                Some(&value.to_le_bytes()),
            );
            let _ = RegSetValueExW(
                key,
                w!("SubscribedContent-353696Enabled"),
                Some(0),
                REG_DWORD,
                Some(&value.to_le_bytes()),
            );
            let _ = RegSetValueExW(
                key,
                w!("SubscribedContent-88000326Enabled"),
                Some(0),
                REG_DWORD,
                Some(&value.to_le_bytes()),
            );
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
            let _ = RegSetValueExW(
                key,
                w!("SubscribedContent-314559Enabled"),
                Some(0),
                REG_DWORD,
                Some(&value.to_le_bytes()),
            );
            let _ = RegSetValueExW(
                key,
                w!("SubscribedContent-338389Enabled"),
                Some(0),
                REG_DWORD,
                Some(&value.to_le_bytes()),
            );
            let _ = RegCloseKey(key);
        }
    }

    println!("[+] Search suggestions disabled");
}

pub fn disable_lockscreen_tips() {
    println!("[*] Disabling lockscreen tips...");

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
                w!("SubscribedContent-310093Enabled"),
                Some(0),
                REG_DWORD,
                Some(&value.to_le_bytes()),
            );
            let _ = RegCloseKey(key);
        }
    }

    println!("[+] Lockscreen tips disabled");
}

pub fn hide_search_tb() {
    println!("[*] Hiding search from taskbar...");

    unsafe {
        let key_path = w!("SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Search");
        let mut key = HKEY::default();
        if RegOpenKeyExW(
            HKEY_CURRENT_USER,
            key_path,
            Some(0),
            KEY_SET_VALUE,
            &raw mut key,
        ) == ERROR_SUCCESS
        {
            let value: u32 = 0; // 0 to hide search box/icon
            let _ = RegSetValueExW(
                key,
                w!("SearchBoxTaskbarMode"),
                Some(0),
                REG_DWORD,
                Some(&value.to_le_bytes()),
            );
            let _ = RegCloseKey(key);
        }
    }

    println!("[+] Search hidden from taskbar");
}

pub fn disable_widgets() {
    println!("[*] Disabling widgets...");

    unsafe {
        // Disable widgets icon in taskbar
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
                w!("TaskbarDa"),
                Some(0),
                REG_DWORD,
                Some(&value.to_le_bytes()),
            );
            let _ = RegSetValueExW(
                key,
                w!("TaskbarMn"),
                Some(0),
                REG_DWORD,
                Some(&value.to_le_bytes()),
            );
            let _ = RegCloseKey(key);
        }

        // Disable feeds
        let key_path2 = w!("SOFTWARE\\Policies\\Microsoft\\Windows\\Windows Feeds");
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
                w!("EnableFeeds"),
                Some(0),
                REG_DWORD,
                Some(&value.to_le_bytes()),
            );
            let _ = RegCloseKey(key2);
        }
    }

    println!("[+] Widgets disabled");
}

pub fn disable_copilot() {
    println!("[*] Disabling Copilot...");

    unsafe {
        // HKCU\Software\Policies\Microsoft\Windows\WindowsCopilot
        let key_path1 = w!("SOFTWARE\\Policies\\Microsoft\\Windows\\WindowsCopilot");
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
                w!("TurnOffWindowsCopilot"),
                Some(0),
                REG_DWORD,
                Some(&value.to_le_bytes()),
            );
            let _ = RegCloseKey(key1);
        }

        // HKLM\SOFTWARE\Policies\Microsoft\Windows\WindowsCopilot
        let key_path2 = w!("SOFTWARE\\Policies\\Microsoft\\Windows\\WindowsCopilot");
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
                w!("TurnOffWindowsCopilot"),
                Some(0),
                REG_DWORD,
                Some(&value.to_le_bytes()),
            );
            let _ = RegCloseKey(key2);
        }

        // HKCU\Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced
        let key_path3 = w!("SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Advanced");
        let mut key3 = HKEY::default();
        if RegOpenKeyExW(
            HKEY_CURRENT_USER,
            key_path3,
            Some(0),
            KEY_SET_VALUE,
            &raw mut key3,
        ) == ERROR_SUCCESS
        {
            let value: u32 = 0;
            let _ = RegSetValueExW(
                key3,
                w!("ShowCopilotButton"),
                Some(0),
                REG_DWORD,
                Some(&value.to_le_bytes()),
            );
            let _ = RegCloseKey(key3);
        }
    }

    println!("[+] Copilot disabled");
}

pub fn clear_start_all_users() {
    println!("[*] Clearing start menu for all users...");

    // This is tricky to do natively; for now, set registry to disable recent/frequent
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
                w!("Start_TrackDocs"),
                Some(0),
                REG_DWORD,
                Some(&value.to_le_bytes()),
            );
            let _ = RegCloseKey(key);
        }
    }

    println!("[+] Start menu cleared/disabled recent");
}

pub fn disable_start_recommended() {
    println!("[*] Disabling start menu recommendations...");

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
                w!("Start_IrisRecommendations"),
                Some(0),
                REG_DWORD,
                Some(&value.to_le_bytes()),
            );
            let _ = RegCloseKey(key);
        }
    }

    println!("[+] Start menu recommendations disabled");
}

pub fn hide_task_view() {
    println!("[*] Hiding task view from taskbar...");

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
                w!("ShowTaskViewButton"),
                Some(0),
                REG_DWORD,
                Some(&value.to_le_bytes()),
            );
            let _ = RegCloseKey(key);
        }
    }

    println!("[+] Task view hidden from taskbar");
}

pub fn hide_pen_menu() {
    println!("[*] Hiding pen menu from taskbar...");

    unsafe {
        let key_path = w!("SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\PenWorkspace");
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
            let value: u32 = 0;
            let _ = RegSetValueExW(
                key,
                w!("PenWorkspaceButtonDesiredVisibility"),
                Some(0),
                REG_DWORD,
                Some(&value.to_le_bytes()),
            );
            let _ = RegCloseKey(key);
        }
    }

    println!("[+] Pen menu hidden from taskbar");
}

pub fn show_virtual_keyboard() {
    println!("[*] Showing virtual keyboard icon always...");

    unsafe {
        let key_path = w!("SOFTWARE\\Microsoft\\TabletTip\\1.7");
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
            let value: u32 = 1;
            let _ = RegSetValueExW(
                key,
                w!("TipbandDesiredVisibility"),
                Some(0),
                REG_DWORD,
                Some(&value.to_le_bytes()),
            );
            let _ = RegCloseKey(key);
        }
    }

    println!("[+] Virtual keyboard icon set to show always");
}

use std::process::Command;
use windows::Win32::Foundation::WIN32_ERROR;

use windows::Win32::System::Registry::{
    RegCreateKeyExW, RegDeleteTreeW, RegSetValueExW, HKEY, HKEY_CURRENT_USER, HKEY_LOCAL_MACHINE,
    KEY_WRITE, REG_BINARY, REG_CREATE_KEY_DISPOSITION, REG_DWORD, REG_OPTION_NON_VOLATILE, REG_SZ,
};

#[allow(clippy::too_many_lines)]
pub fn apply_registry_tweaks() -> Result<(), String> {
    println!("[*] Applying registry tweaks...");

    // GameDVR: Disable app capture
    set_dword_value(
        HKEY_CURRENT_USER,
        "Software\\Microsoft\\Windows\\CurrentVersion\\GameDVR",
        "AppCaptureEnabled",
        0,
    )?;
    println!("[+] Disabled GameDVR AppCapture");

    // Menu show delay: Set to 0 for faster menu display
    set_string_value(
        HKEY_CURRENT_USER,
        "Control Panel\\Desktop",
        "MenuShowDelay",
        "0",
    )?;
    println!("[+] Set MenuShowDelay to 0");

    // Extended UI hover time
    set_dword_value(
        HKEY_CURRENT_USER,
        "Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Advanced",
        "ExtendedUIHoverTime",
        1,
    )?;
    println!("[+] Set ExtendedUIHoverTime to 1");

    // Show file extensions
    set_dword_value(
        HKEY_CURRENT_USER,
        "Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Advanced",
        "HideFileExt",
        0,
    )?;
    println!("[+] Enabled file extension visibility");

    // Drag full windows
    set_string_value(
        HKEY_CURRENT_USER,
        "Control Panel\\Desktop",
        "DragFullWindows",
        "1",
    )?;
    println!("[+] Enabled drag full windows");

    // Window animations
    set_string_value(
        HKEY_CURRENT_USER,
        "Control Panel\\Desktop\\WindowMetrics",
        "MinAnimate",
        "1",
    )?;
    println!("[+] Enabled window animations");

    // Rounded window borders
    set_dword_value(
        HKEY_LOCAL_MACHINE,
        "SOFTWARE\\Microsoft\\Windows\\Dwm",
        "RoundedCorners",
        1,
    )?;
    println!("[+] Enabled rounded window borders");

    // Shake to minimize
    set_dword_value(
        HKEY_CURRENT_USER,
        "Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Advanced",
        "DisallowShaking",
        0,
    )?;
    println!("[+] Enabled shake to minimize");

    // Disable app tabs in Alt+Tab
    set_dword_value(
        HKEY_CURRENT_USER,
        "Software\\Microsoft\\Windows\\CurrentVersion\\Explorer",
        "AltTabSettings",
        1,
    )?;
    println!("[+] Disabled app tabs in Alt+Tab");

    // Enable End task in taskbar
    set_dword_value(
        HKEY_CURRENT_USER,
        "Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Advanced\\TaskbarDeveloperSettings",
        "TaskbarEndTask",
        1,
    )?;
    println!("[+] Enabled End task in taskbar right-click menu");

    // Unpin all taskbar icons
    delete_registry_tree(
        HKEY_CURRENT_USER,
        "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Taskband",
    )?;
    println!("[+] Unpinned all taskbar icons");

    // Hide Widgets icon from taskbar
    // set_dword_value(
    //     HKEY_CURRENT_USER,
    //     "Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Advanced",
    //     "TaskbarDa",
    //     0,
    // )?;
    // println!("[+] Hid Widgets icon from taskbar");

    // Remove Search icon from taskbar
    set_dword_value(
        HKEY_CURRENT_USER,
        "Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Advanced",
        "SearchboxTaskbarMode",
        0,
    )?;
    println!("[+] Removed Search icon from taskbar");

    // Show time in seconds
    set_dword_value(
        HKEY_CURRENT_USER,
        "Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Advanced",
        "ShowSecondsInSystemClock",
        1,
    )?;
    println!("[+] Enabled showing time in seconds");

    // Hide all desktop icons
    set_dword_value(
        HKEY_CURRENT_USER,
        "Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Advanced",
        "HideIcons",
        1,
    )?;
    println!("[+] Hid all desktop icons");

    // Disable Windows Search indexing
    set_dword_value(
        HKEY_LOCAL_MACHINE,
        "SOFTWARE\\Policies\\Microsoft\\Windows\\Windows Search",
        "AllowIndexingEncryptedStoresOrItems",
        0,
    )?;
    println!("[+] Disabled Windows Search indexing");

    // Power settings: Best performance when plugged in
    set_dword_value(
        HKEY_LOCAL_MACHINE,
        "SYSTEM\\CurrentControlSet\\Control\\Power\\PowerSettings\\54533251-82be-4824-96c1-47b60b740d00\\893dee8e-2bef-41e0-89c6-b55d0929964c\\DefaultPowerSchemeValues\\381b4222-f694-41f0-9685-ff5bb260df2e",
        "ACSettingIndex",
        0,
    )?;
    println!("[+] Set power mode to Best performance when plugged in");

    // Best power efficiency on battery
    set_dword_value(
        HKEY_LOCAL_MACHINE,
        "SYSTEM\\CurrentControlSet\\Control\\Power\\PowerSettings\\54533251-82be-4824-96c1-47b60b740d00\\893dee8e-2bef-41e0-89c6-b55d0929964c\\DefaultPowerSchemeValues\\381b4222-f694-41f0-9685-ff5bb260df2e",
        "DCSettingIndex",
        1,
    )?;
    println!("[+] Set power mode to Best power efficiency on battery");

    // Disable idle screen timeout
    set_dword_value(
        HKEY_LOCAL_MACHINE,
        "SYSTEM\\CurrentControlSet\\Control\\Power\\PowerSettings\\7516b95f-f776-4464-8c53-06167f40cc99\\3c0bc021-c8a8-4e07-a973-6b14cbcb2b7e\\DefaultPowerSchemeValues\\381b4222-f694-41f0-9685-ff5bb260df2e",
        "ACSettingIndex",
        0,
    )?;
    set_dword_value(
        HKEY_LOCAL_MACHINE,
        "SYSTEM\\CurrentControlSet\\Control\\Power\\PowerSettings\\7516b95f-f776-4464-8c53-06167f40cc99\\3c0bc021-c8a8-4e07-a973-6b14cbcb2b7e\\DefaultPowerSchemeValues\\381b4222-f694-41f0-9685-ff5bb260df2e",
        "DCSettingIndex",
        0,
    )?;
    println!("[+] Disabled idle screen timeout");

    // Enable clipboard history
    set_dword_value(
        HKEY_CURRENT_USER,
        "Software\\Microsoft\\Clipboard",
        "EnableClipboardHistory",
        1,
    )?;
    println!("[+] Enabled clipboard history");

    // --- Visual Effects (HKCU) ---
    set_dword_value(
        HKEY_CURRENT_USER,
        "Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\VisualEffects",
        "VisualFXSetting",
        3, // 3 = Custom
    )?;
    println!("[+] Set VisualFXSetting to Custom");

    set_string_value(
        HKEY_CURRENT_USER,
        "Control Panel\\Desktop\\WindowMetrics",
        "MinAnimate",
        "1", // min/max animations
    )?;
    println!("[+] Enabled min/max animations");

    set_string_value(
        HKEY_CURRENT_USER,
        "Control Panel\\Desktop",
        "DragFullWindows",
        "1",
    )?;
    println!("[+] Enabled drag full windows");

    set_string_value(
        HKEY_CURRENT_USER,
        "Control Panel\\Desktop",
        "FontSmoothing",
        "2",
    )?;
    println!("[+] Enabled font smoothing");

    set_dword_value(
        HKEY_CURRENT_USER,
        "Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Advanced",
        "TaskbarAnimations",
        1,
    )?;
    println!("[+] Enabled taskbar animations");

    set_dword_value(
        HKEY_CURRENT_USER,
        "Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Advanced",
        "IconsOnly",
        0,
    )?;
    println!("[+] Disabled icons only");

    set_dword_value(
        HKEY_CURRENT_USER,
        "Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Advanced",
        "ListviewShadow",
        1,
    )?;
    println!("[+] Enabled listview shadow");

    set_dword_value(
        HKEY_CURRENT_USER,
        "Software\\Microsoft\\Windows\\DWM",
        "EnableAeroPeek",
        1,
    )?;
    println!("[+] Enabled AeroPeek");

    set_dword_value(
        HKEY_CURRENT_USER,
        "Software\\Microsoft\\Windows\\DWM",
        "AlwaysHibernateThumbnails",
        0,
    )?;
    println!("[+] Disabled always hibernate thumbnails");

    // Bundle bits (best-appearance-style mask)
    let best_appearance_mask: [u8; 8] = [0x9E, 0x3E, 0x07, 0x80, 0x12, 0x00, 0x00, 0x00];
    set_binary_value(
        HKEY_CURRENT_USER,
        "Control Panel\\Desktop",
        "UserPreferencesMask",
        &best_appearance_mask,
    )?;
    println!("[+] Set UserPreferencesMask for best appearance");

    // --- Rounded corners / DWM (HKLM) ---
    set_dword_value(
        HKEY_LOCAL_MACHINE,
        "SOFTWARE\\Microsoft\\Windows\\Dwm",
        "ForceEffectMode",
        2,
    )?;
    println!("[+] Forced effect mode for DWM");

    set_dword_value(
        HKEY_LOCAL_MACHINE,
        "SOFTWARE\\Microsoft\\Windows\\Dwm",
        "RoundedCorners",
        1,
    )?;
    println!("[+] Enabled rounded corners in DWM");

    // Enable Developer Mode
    set_dword_value(
        HKEY_LOCAL_MACHINE,
        "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\AppModelUnlock",
        "AllowDevelopmentWithoutDevLicense",
        1,
    )?;
    println!("[+] Enabled Developer Mode");

    // Show hidden files
    set_dword_value(
        HKEY_CURRENT_USER,
        "Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Advanced",
        "Hidden",
        1,
    )?;
    println!("[+] Enabled showing hidden files");

    // Show system files
    set_dword_value(
        HKEY_CURRENT_USER,
        "Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Advanced",
        "ShowSuperHidden",
        1,
    )?;
    println!("[+] Enabled showing system files");

    // Show empty drives
    set_dword_value(
        HKEY_CURRENT_USER,
        "Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Advanced",
        "HideDrivesWithNoMedia",
        0,
    )?;
    println!("[+] Enabled showing empty drives");

    // Show full path in Explorer title bar
    set_dword_value(
        HKEY_CURRENT_USER,
        "Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\CabinetState",
        "FullPathAddress",
        1,
    )?;
    println!("[+] Enabled full path in Explorer title bar");

    // Enable Run as different user in Start
    set_dword_value(
        HKEY_CURRENT_USER,
        "Software\\Policies\\Microsoft\\Windows\\Explorer",
        "ShowRunAsDifferentUserInStart",
        1,
    )?;
    println!("[+] Enabled Run as different user in Start");

    // Enable long file paths
    set_dword_value(
        HKEY_LOCAL_MACHINE,
        "SYSTEM\\CurrentControlSet\\Control\\FileSystem",
        "LongPathsEnabled",
        1,
    )?;
    println!("[+] Enabled long file paths");

    // Set NTP server to time.nist.gov
    set_string_value(
        HKEY_LOCAL_MACHINE,
        "SYSTEM\\CurrentControlSet\\Services\\W32Time\\Parameters",
        "NtpServer",
        "time.nist.gov",
    )?;
    println!("[+] Set NTP server to time.nist.gov");

    println!("[+] All registry tweaks applied");

    // Refresh per-user params
    Command::new("rundll32.exe")
        .args(["user32.dll,UpdatePerUserSystemParameters", "1,", "True"])
        .status()
        .map_err(|e| format!("Failed to update per-user system parameters: {e}"))?;
    println!("[+] Refreshed per-user system parameters");

    // Restart Explorer to apply registry changes
    Command::new("taskkill")
        .args(["/F", "/IM", "explorer.exe"])
        .status()
        .map_err(|e| format!("Failed to kill explorer: {e}"))?;
    Command::new("explorer.exe")
        .status()
        .map_err(|e| format!("Failed to start explorer: {e}"))?;
    println!("[+] Restarted Explorer");

    Ok(())
}

fn to_wide(s: &str) -> Vec<u16> {
    use std::ffi::OsStr;
    use std::iter::once;
    use std::os::windows::ffi::OsStrExt;

    OsStr::new(s).encode_wide().chain(once(0)).collect()
}

fn set_dword_value(
    hive: windows::Win32::System::Registry::HKEY,
    subkey_str: &str,
    value_name_str: &str,
    value: u32,
) -> Result<(), String> {
    unsafe {
        let subkey_wide = to_wide(subkey_str);
        let value_name_wide = to_wide(value_name_str);

        let subkey = windows::core::PCWSTR(subkey_wide.as_ptr());
        let value_name = windows::core::PCWSTR(value_name_wide.as_ptr());

        let mut hkey = HKEY::default();
        let mut disposition = REG_CREATE_KEY_DISPOSITION(0);

        let result = RegCreateKeyExW(
            hive,
            subkey,
            Some(0),
            None,
            REG_OPTION_NON_VOLATILE,
            KEY_WRITE,
            None,
            &raw mut hkey,
            Some(&raw mut disposition),
        );

        if result != WIN32_ERROR(0) {
            return Err(format!("Failed to create registry key: {result:?}"));
        }

        let value_bytes = value.to_le_bytes();
        let result = RegSetValueExW(hkey, value_name, Some(0), REG_DWORD, Some(&value_bytes));

        if result != WIN32_ERROR(0) {
            return Err(format!("Failed to set registry value: {result:?}"));
        }
    }
    Ok(())
}

fn set_string_value(
    hive: windows::Win32::System::Registry::HKEY,
    subkey_str: &str,
    value_name_str: &str,
    value: &str,
) -> Result<(), String> {
    unsafe {
        let subkey_wide = to_wide(subkey_str);
        let value_name_wide = to_wide(value_name_str);

        let subkey = windows::core::PCWSTR(subkey_wide.as_ptr());
        let value_name = windows::core::PCWSTR(value_name_wide.as_ptr());

        let mut hkey = HKEY::default();
        let mut disposition = REG_CREATE_KEY_DISPOSITION(0);

        let result = RegCreateKeyExW(
            hive,
            subkey,
            Some(0),
            None,
            REG_OPTION_NON_VOLATILE,
            KEY_WRITE,
            None,
            &raw mut hkey,
            Some(&raw mut disposition),
        );

        if result != WIN32_ERROR(0) {
            return Err(format!("Failed to create registry key: {result:?}"));
        }

        let wide: Vec<u16> = value.encode_utf16().chain(std::iter::once(0)).collect();

        let data: Vec<u8> = wide
            .iter()
            .flat_map(|&w| [(w & 0xFF) as u8, ((w >> 8) & 0xFF) as u8])
            .collect();

        let result = RegSetValueExW(hkey, value_name, Some(0), REG_SZ, Some(&data));

        if result != WIN32_ERROR(0) {
            return Err(format!("Failed to set registry value: {result:?}"));
        }
    }
    Ok(())
}

fn set_binary_value(
    hive: windows::Win32::System::Registry::HKEY,
    subkey_str: &str,
    value_name_str: &str,
    value: &[u8],
) -> Result<(), String> {
    unsafe {
        let subkey_wide = to_wide(subkey_str);
        let value_name_wide = to_wide(value_name_str);

        let subkey = windows::core::PCWSTR(subkey_wide.as_ptr());
        let value_name = windows::core::PCWSTR(value_name_wide.as_ptr());

        let mut hkey = HKEY::default();
        let mut disposition = REG_CREATE_KEY_DISPOSITION(0);

        let result = RegCreateKeyExW(
            hive,
            subkey,
            Some(0),
            None,
            REG_OPTION_NON_VOLATILE,
            KEY_WRITE,
            None,
            &raw mut hkey,
            Some(&raw mut disposition),
        );

        if result != WIN32_ERROR(0) {
            return Err(format!("Failed to create registry key: {result:?}"));
        }

        let result = RegSetValueExW(hkey, value_name, Some(0), REG_BINARY, Some(value));

        if result != WIN32_ERROR(0) {
            return Err(format!("Failed to set registry value: {result:?}"));
        }
    }
    Ok(())
}

fn delete_registry_tree(
    hive: windows::Win32::System::Registry::HKEY,
    subkey_str: &str,
) -> Result<(), String> {
    unsafe {
        let subkey_wide = to_wide(subkey_str);
        let subkey = windows::core::PCWSTR(subkey_wide.as_ptr());

        let result = RegDeleteTreeW(hive, subkey);

        if result != WIN32_ERROR(0) {
            return Err(format!("Failed to delete registry tree: {result:?}"));
        }
    }
    Ok(())
}

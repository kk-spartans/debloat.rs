use super::registry_helpers::{set_binary_value, set_dword_value, set_string_value};

use windows::Win32::System::Registry::{HKEY_CURRENT_USER, HKEY_LOCAL_MACHINE};

pub fn apply_ui_tweaks() -> Result<(), String> {
    set_string_value(HKEY_CURRENT_USER, "Control Panel\\Desktop", "MenuShowDelay", "0")?;
    set_string_value(HKEY_CURRENT_USER, "Control Panel\\Desktop", "DragFullWindows", "1")?;
    set_string_value(HKEY_CURRENT_USER, "Control Panel\\Desktop\\WindowMetrics", "MinAnimate", "1")?;
    set_string_value(HKEY_CURRENT_USER, "Control Panel\\Desktop", "FontSmoothing", "2")?;
    set_dword_value(HKEY_CURRENT_USER, "Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\VisualEffects", "VisualFXSetting", 3)?;
    set_dword_value(HKEY_CURRENT_USER, "Software\\Microsoft\\Windows\\DWM", "EnableAeroPeek", 1)?;
    set_dword_value(HKEY_CURRENT_USER, "Software\\Microsoft\\Windows\\DWM", "AlwaysHibernateThumbnails", 0)?;
    set_dword_value(HKEY_LOCAL_MACHINE, "SOFTWARE\\Microsoft\\Windows\\Dwm", "RoundedCorners", 1)?;
    set_dword_value(HKEY_LOCAL_MACHINE, "SOFTWARE\\Microsoft\\Windows\\Dwm", "ForceEffectMode", 2)?;
    set_dword_value(HKEY_CURRENT_USER, "Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Advanced", "ExtendedUIHoverTime", 1)?;
    set_dword_value(HKEY_CURRENT_USER, "Software\\Microsoft\\Windows\\CurrentVersion\\Clipboard", "EnableClipboardHistory", 1)?;
    let best_appearance_mask: [u8; 8] = [0x9E, 0x3E, 0x07, 0x80, 0x12, 0x00, 0x00, 0x00];
    set_binary_value(HKEY_CURRENT_USER, "Control Panel\\Desktop", "UserPreferencesMask", &best_appearance_mask)?;
    Ok(())
}

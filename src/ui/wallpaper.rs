use std::ffi::OsStr;
use std::fs;
use std::io::Read;
use std::iter::once;
use std::os::windows::ffi::OsStrExt;

use windows::Win32::Foundation::ERROR_SUCCESS;
use windows::Win32::System::Registry::{
    HKEY, HKEY_LOCAL_MACHINE, KEY_SET_VALUE, REG_OPTION_NON_VOLATILE, REG_SZ, RegCloseKey,
    RegCreateKeyExW, RegSetValueExW,
};
use windows::Win32::UI::WindowsAndMessaging::{
    SPI_SETDESKWALLPAPER, SPIF_SENDCHANGE, SPIF_UPDATEINIFILE, SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS,
    SystemParametersInfoW,
};
use windows::core::{Result as WinResult, w};

fn to_wide(s: &str) -> Vec<u16> {
    OsStr::new(s).encode_wide().chain(once(0)).collect()
}

pub fn download_wallpaper(url: &str, path: &str) -> Result<(), String> {
    let response = ureq::get(url)
        .call()
        .map_err(|e| format!("Failed to download: {e}"))?;
    let mut bytes = Vec::new();
    response
        .into_reader()
        .read_to_end(&mut bytes)
        .map_err(|e| format!("Failed to read response: {e}"))?;
    fs::write(path, &bytes).map_err(|e| format!("Failed to write file: {e}"))?;
    Ok(())
}

pub fn set_wallpaper_desktop(path: &str) -> WinResult<()> {
    let wide = to_wide(path);
    unsafe {
        SystemParametersInfoW(
            SPI_SETDESKWALLPAPER,
            0,
            Some(wide.as_ptr() as *mut _),
            SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS(SPIF_UPDATEINIFILE.0 | SPIF_SENDCHANGE.0),
        )?;
    }
    Ok(())
}

#[allow(dead_code)]
pub fn set_wallpaper_lock_screen(path: &str) {
    unsafe {
        let key_path = w!("SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\PersonalizationCSP");
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
            let value_bytes = to_wide(path);
            let data: Vec<u8> = value_bytes.iter().flat_map(|&c| c.to_le_bytes()).collect();
            let _ = RegSetValueExW(key, w!("LockScreenImagePath"), Some(0), REG_SZ, Some(&data));
            let _ = RegCloseKey(key);
        }
    }
}

use windows::core::{w, Result as WinResult};
use windows::Win32::Foundation::WIN32_ERROR;
use windows::Win32::System::Registry::{
    RegCreateKeyExW, RegSetValueExW, HKEY, HKEY_CURRENT_USER, KEY_WRITE,
    REG_CREATE_KEY_DISPOSITION, REG_DWORD, REG_OPEN_CREATE_OPTIONS, REG_OPTION_NON_VOLATILE,
    REG_SAM_FLAGS,
};

pub fn enable_dark_mode() -> WinResult<()> {
    println!("[*] Enabling dark mode...");
    unsafe {
        let subkey = w!("Software\\Microsoft\\Windows\\CurrentVersion\\Themes\\Personalize");
        let mut hkey = HKEY::default();
        let mut disposition = REG_CREATE_KEY_DISPOSITION(0);

        let err = RegCreateKeyExW(
            HKEY_CURRENT_USER,
            subkey,
            Some(0),
            None,
            REG_OPEN_CREATE_OPTIONS(REG_OPTION_NON_VOLATILE.0),
            REG_SAM_FLAGS(KEY_WRITE.0),
            None,
            &raw mut hkey,
            Some(&raw mut disposition),
        );
        if err != WIN32_ERROR(0) {
            eprintln!("Failed to open registry key: {err:?}");
            return Err(windows::core::Error::from(err));
        }

        let zero: u32 = 0;

        let name1 = w!("AppsUseLightTheme");
        let err = RegSetValueExW(
            hkey,
            name1,
            Some(0),
            REG_DWORD,
            Some(std::slice::from_raw_parts(
                (&raw const zero).cast::<u8>(),
                std::mem::size_of::<u32>(),
            )),
        );
        if err != WIN32_ERROR(0) {
            return Err(windows::core::Error::from(err));
        }

        let name2 = w!("SystemUsesLightTheme");
        let err = RegSetValueExW(
            hkey,
            name2,
            Some(0),
            REG_DWORD,
            Some(std::slice::from_raw_parts(
                (&raw const zero).cast::<u8>(),
                std::mem::size_of::<u32>(),
            )),
        );
        if err != WIN32_ERROR(0) {
            return Err(windows::core::Error::from(err));
        }
    }
    println!("[+] Dark mode enabled");
    Ok(())
}

pub fn enable_transparency() -> WinResult<()> {
    println!("[*] Enabling transparency effects...");
    unsafe {
        let subkey = w!("Software\\Microsoft\\Windows\\CurrentVersion\\Themes\\Personalize");
        let mut hkey = HKEY::default();
        let mut disposition = REG_CREATE_KEY_DISPOSITION(0);

        let err = RegCreateKeyExW(
            HKEY_CURRENT_USER,
            subkey,
            Some(0),
            None,
            REG_OPEN_CREATE_OPTIONS(REG_OPTION_NON_VOLATILE.0),
            REG_SAM_FLAGS(KEY_WRITE.0),
            None,
            &raw mut hkey,
            Some(&raw mut disposition),
        );
        if err != WIN32_ERROR(0) {
            eprintln!("Failed to open registry key: {err:?}");
            return Err(windows::core::Error::from(err));
        }

        let one: u32 = 1;

        let name = w!("EnableTransparency");
        let err = RegSetValueExW(
            hkey,
            name,
            Some(0),
            REG_DWORD,
            Some(std::slice::from_raw_parts(
                (&raw const one).cast::<u8>(),
                std::mem::size_of::<u32>(),
            )),
        );
        if err != WIN32_ERROR(0) {
            return Err(windows::core::Error::from(err));
        }
    }
    println!("[+] Transparency effects enabled");
    Ok(())
}

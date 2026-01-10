use std::ffi::OsStr;
use std::iter::once;
use std::os::windows::ffi::OsStrExt;
use windows::Win32::Foundation::WIN32_ERROR;
use windows::Win32::System::Registry::{
    HKEY, KEY_WRITE, REG_BINARY, REG_CREATE_KEY_DISPOSITION, REG_DWORD, REG_OPTION_NON_VOLATILE,
    REG_SZ, RegCreateKeyExW, RegDeleteTreeW, RegSetValueExW,
};

pub fn to_wide(s: &str) -> Vec<u16> {
    OsStr::new(s).encode_wide().chain(once(0)).collect()
}

pub fn set_dword_value(
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

pub fn set_string_value(
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

pub fn set_binary_value(
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

pub fn delete_registry_tree(
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

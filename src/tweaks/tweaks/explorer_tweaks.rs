use windows::Win32::Foundation::ERROR_SUCCESS;
use windows::Win32::System::Registry::{
    HKEY, HKEY_CURRENT_USER, KEY_SET_VALUE, REG_DWORD, REG_OPTION_NON_VOLATILE,
    RegCloseKey, RegCreateKeyExW, RegOpenKeyExW, RegSetValueExW,
};
use windows::core::w;

pub fn remove_home_from_explorer() {
    unsafe {
        let key_path1 = w!("SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Advanced");
        let mut key1 = HKEY::default();
        if RegOpenKeyExW(HKEY_CURRENT_USER, key_path1, Some(0), KEY_SET_VALUE, &raw mut key1) == ERROR_SUCCESS {
            let value: u32 = 0;
            let _ = RegSetValueExW(key1, w!("Start_IrisRecommendations"), Some(0), REG_DWORD, Some(&value.to_le_bytes()));
            let _ = RegCloseKey(key1);
        }

        let key_path2 = w!("SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Policies\\Explorer");
        let mut key2 = HKEY::default();
        if RegCreateKeyExW(HKEY_CURRENT_USER, key_path2, Some(0), None, REG_OPTION_NON_VOLATILE, KEY_SET_VALUE, None, &raw mut key2, None) == ERROR_SUCCESS {
            let value: u32 = 1;
            let _ = RegSetValueExW(key2, w!("NoStartMenuPinnedList"), Some(0), REG_DWORD, Some(&value.to_le_bytes()));
            let _ = RegCloseKey(key2);
        }
    }
}

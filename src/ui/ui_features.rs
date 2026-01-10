use std::ffi::OsStr;
use std::iter::once;
use std::os::windows::ffi::OsStrExt;
use windows::Win32::Foundation::ERROR_SUCCESS;
use windows::Win32::System::Registry::{
    HKEY, HKEY_CURRENT_USER, HKEY_LOCAL_MACHINE, KEY_SET_VALUE, REG_DWORD, REG_OPTION_NON_VOLATILE,
    RegCloseKey, RegCreateKeyExW, RegOpenKeyExW, RegSetValueExW,
};
use windows::core::PCWSTR;

fn to_wide(s: &str) -> Vec<u16> {
    OsStr::new(s).encode_wide().chain(once(0)).collect()
}

fn set_dword_value(hive: HKEY, key_path: &str, value_name: &str, value: u32) {
    unsafe {
        let key_path_w = to_wide(key_path);
        let value_name_w = to_wide(value_name);
        let mut key = HKEY::default();

        let open_result = RegOpenKeyExW(
            hive,
            PCWSTR(key_path_w.as_ptr()),
            Some(0),
            KEY_SET_VALUE,
            &raw mut key,
        );
        if open_result != ERROR_SUCCESS {
            return;
        }

        let _ = RegSetValueExW(
            key,
            PCWSTR(value_name_w.as_ptr()),
            Some(0),
            REG_DWORD,
            Some(&value.to_le_bytes()),
        );
        let _ = RegCloseKey(key);
    }
}

fn create_and_set_dword(hive: HKEY, key_path: &str, value_name: &str, value: u32) {
    unsafe {
        let key_path_w = to_wide(key_path);
        let value_name_w = to_wide(value_name);
        let mut key = HKEY::default();

        let create_result = RegCreateKeyExW(
            hive,
            PCWSTR(key_path_w.as_ptr()),
            Some(0),
            None,
            REG_OPTION_NON_VOLATILE,
            KEY_SET_VALUE,
            None,
            &raw mut key,
            None,
        );
        if create_result != ERROR_SUCCESS {
            return;
        }

        let _ = RegSetValueExW(
            key,
            PCWSTR(value_name_w.as_ptr()),
            Some(0),
            REG_DWORD,
            Some(&value.to_le_bytes()),
        );
        let _ = RegCloseKey(key);
    }
}

pub fn disable_suggestions() {
    let key_path = "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\ContentDeliveryManager";
    let values = [
        "SystemPaneSuggestionsEnabled",
        "SoftLandingEnabled",
        "RotatingLockScreenEnabled",
        "RotatingLockScreenOverlayEnabled",
        "SubscribedContentEnabled",
        "SubscribedContent-338393Enabled",
        "SubscribedContent-353694Enabled",
        "SubscribedContent-353696Enabled",
        "SubscribedContent-88000326Enabled",
        "SubscribedContent-310093Enabled",
        "SubscribedContent-338388Enabled",
        "SubscribedContent-314559Enabled",
        "SubscribedContent-338389Enabled",
    ];

    for value in values {
        set_dword_value(HKEY_CURRENT_USER, key_path, value, 0);
    }
}

pub fn disable_lockscreen_tips() {
    let key_path = "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\ContentDeliveryManager";
    let values = [
        "RotatingLockScreenEnabled",
        "RotatingLockScreenOverlayEnabled",
        "SubscribedContent-310093Enabled",
    ];

    for value in values {
        set_dword_value(HKEY_CURRENT_USER, key_path, value, 0);
    }
}

pub fn hide_search_tb() {
    set_dword_value(
        HKEY_CURRENT_USER,
        "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Search",
        "SearchBoxTaskbarMode",
        0,
    );
}

pub fn disable_widgets() {
    set_dword_value(
        HKEY_CURRENT_USER,
        "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Advanced",
        "TaskbarDa",
        0,
    );
    set_dword_value(
        HKEY_CURRENT_USER,
        "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Advanced",
        "TaskbarMn",
        0,
    );
    create_and_set_dword(
        HKEY_LOCAL_MACHINE,
        "SOFTWARE\\Policies\\Microsoft\\Windows\\Windows Feeds",
        "EnableFeeds",
        0,
    );
}

pub fn disable_copilot() {
    create_and_set_dword(
        HKEY_CURRENT_USER,
        "SOFTWARE\\Policies\\Microsoft\\Windows\\WindowsCopilot",
        "TurnOffWindowsCopilot",
        1,
    );
    create_and_set_dword(
        HKEY_LOCAL_MACHINE,
        "SOFTWARE\\Policies\\Microsoft\\Windows\\WindowsCopilot",
        "TurnOffWindowsCopilot",
        1,
    );
    set_dword_value(
        HKEY_CURRENT_USER,
        "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Advanced",
        "ShowCopilotButton",
        0,
    );
}

pub fn clear_start_all_users() {
    set_dword_value(
        HKEY_CURRENT_USER,
        "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Advanced",
        "Start_TrackDocs",
        0,
    );
}

pub fn disable_start_recommended() {
    set_dword_value(
        HKEY_CURRENT_USER,
        "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Advanced",
        "Start_IrisRecommendations",
        0,
    );
}

pub fn hide_task_view() {
    set_dword_value(
        HKEY_CURRENT_USER,
        "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Advanced",
        "ShowTaskViewButton",
        0,
    );
}

pub fn hide_pen_menu() {
    create_and_set_dword(
        HKEY_CURRENT_USER,
        "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\PenWorkspace",
        "PenWorkspaceButtonDesiredVisibility",
        0,
    );
}

pub fn show_virtual_keyboard() {
    create_and_set_dword(
        HKEY_CURRENT_USER,
        "SOFTWARE\\Microsoft\\TabletTip\\1.7",
        "TipbandDesiredVisibility",
        1,
    );
}

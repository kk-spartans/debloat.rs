use windows::Win32::System::Registry::HKEY_CURRENT_USER;

use super::registry_helpers::{delete_registry_tree, set_dword_value};

const EXPLORER_ADVANCED: &str = "Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Advanced";

fn set_explorer_advanced(name: &str, value: u32) -> Result<(), String> {
    set_dword_value(HKEY_CURRENT_USER, EXPLORER_ADVANCED, name, value)
}

fn apply_advanced_tweaks() -> Result<(), String> {
    set_explorer_advanced("HideFileExt", 0)?;
    set_explorer_advanced("Hidden", 1)?;
    set_explorer_advanced("ShowSuperHidden", 1)?;
    set_explorer_advanced("HideDrivesWithNoMedia", 0)?;
    set_explorer_advanced("DisablePreviewHandlers", 1)?;
    set_explorer_advanced("DisableThumbnailCache", 1)?;
    set_explorer_advanced("TaskbarAnimations", 1)?;
    set_explorer_advanced("IconsOnly", 0)?;
    set_explorer_advanced("ListviewShadow", 1)?;
    set_explorer_advanced("LaunchTo", 1)?;
    set_explorer_advanced("SearchboxTaskbarMode", 0)?;
    set_explorer_advanced("ShowSecondsInSystemClock", 1)?;
    set_explorer_advanced("HideIcons", 1)?;
    set_explorer_advanced("DisallowShaking", 0)?;
    set_explorer_advanced("AltTabSettings", 1)
}

fn apply_misc_tweaks() -> Result<(), String> {
    set_dword_value(
        HKEY_CURRENT_USER,
        "Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Advanced\\TaskbarDeveloperSettings",
        "TaskbarEndTask",
        1,
    )?;
    set_dword_value(
        HKEY_CURRENT_USER,
        "Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\CabinetState",
        "FullPathAddress",
        1,
    )?;
    delete_registry_tree(
        HKEY_CURRENT_USER,
        "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Taskband",
    )
}

pub fn apply_explorer_tweaks() -> Result<(), String> {
    apply_advanced_tweaks()?;
    apply_misc_tweaks()
}

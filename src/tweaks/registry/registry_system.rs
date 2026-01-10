use super::registry_helpers::{set_dword_value, set_string_value};

use windows::Win32::System::Registry::{HKEY_CURRENT_USER, HKEY_LOCAL_MACHINE};

pub fn apply_system_tweaks() -> Result<(), String> {
    set_dword_value(
        HKEY_LOCAL_MACHINE,
        "SOFTWARE\\Policies\\Microsoft\\Windows\\Windows Search",
        "AllowIndexingEncryptedStoresOrItems",
        0,
    )?;
    set_dword_value(
        HKEY_LOCAL_MACHINE,
        "SYSTEM\\CurrentControlSet\\Control\\Power\\PowerSettings\\54533251-82be-4824-96c1-47b60b740d00\\893dee8e-2bef-41e0-89c6-b55d0929964c\\DefaultPowerSchemeValues\\381b4222-f694-41f0-9685-ff5bb260df2e",
        "ACSettingIndex",
        0,
    )?;
    set_dword_value(
        HKEY_LOCAL_MACHINE,
        "SYSTEM\\CurrentControlSet\\Control\\Power\\PowerSettings\\54533251-82be-4824-96c1-47b60b740d00\\893dee8e-2bef-41e0-89c6-b55d0929964c\\DefaultPowerSchemeValues\\381b4222-f694-41f0-9685-ff5bb260df2e",
        "DCSettingIndex",
        1,
    )?;
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
    set_dword_value(
        HKEY_LOCAL_MACHINE,
        "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\AppModelUnlock",
        "AllowDevelopmentWithoutDevLicense",
        1,
    )?;
    set_dword_value(
        HKEY_LOCAL_MACHINE,
        "SYSTEM\\CurrentControlSet\\Control\\FileSystem",
        "LongpathsEnabled",
        1,
    )?;
    set_string_value(
        HKEY_LOCAL_MACHINE,
        "SYSTEM\\CurrentControlSet\\Services\\W32Time\\Parameters",
        "NtpServer",
        "time.nist.gov",
    )?;
    set_dword_value(
        HKEY_CURRENT_USER,
        "Software\\Policies\\Microsoft\\Windows\\Explorer",
        "ShowRunAsDifferentUserInStart",
        1,
    )?;
    set_dword_value(
        HKEY_LOCAL_MACHINE,
        "SOFTWARE\\Microsoft\\Windows\\Dwm",
        "RoundedCorners",
        1,
    )?;
    Ok(())
}

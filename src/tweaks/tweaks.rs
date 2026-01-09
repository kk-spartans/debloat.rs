use std::process::{Command, Stdio};
use windows::core::w;
use windows::Win32::Foundation::ERROR_SUCCESS;
use windows::Win32::System::Registry::{
    RegCloseKey, RegCreateKeyExW, RegOpenKeyExW, RegSetValueExW, HKEY, HKEY_CURRENT_USER,
    HKEY_LOCAL_MACHINE, KEY_SET_VALUE, REG_DWORD, REG_OPTION_NON_VOLATILE,
};

pub fn wpf_tweaks_debloat() {
    println!("[*] Removing bloatware apps...");
    let apps = vec![
        "Microsoft.BingWeather",
        "Microsoft.GetHelp",
        "Microsoft.Getstarted",
        "Microsoft.Messaging",
        "Microsoft.Microsoft3DViewer",
        "Microsoft.MicrosoftOfficeHub",
        "Microsoft.MicrosoftSolitaireCollection",
        "Microsoft.MixedReality.Portal",
        "Microsoft.Office.OneNote",
        "Microsoft.People",
        "Microsoft.Print3D",
        "Microsoft.SkypeApp",
        "Microsoft.Wallet",
        "Microsoft.WindowsAlarms",
        "Microsoft.WindowsCamera",
        "Microsoft.WindowsMaps",
        "Microsoft.WindowsSoundRecorder",
        "Microsoft.Windows.Photos",
        "Microsoft.XboxApp",
        "Microsoft.XboxGameOverlay",
        "Microsoft.XboxGamingOverlay",
        "Microsoft.XboxIdentityProvider",
        "Microsoft.XboxSpeechToTextOverlay",
        "Microsoft.YourPhone",
        "Microsoft.ZuneMusic",
        "Microsoft.ZuneVideo",
    ];

    for app in apps {
        let _ = Command::new("powershell")
            .args([
                "-NoProfile",
                "-ExecutionPolicy",
                "Bypass",
                "-Command",
                &format!(
                    "Get-AppxPackage {app} | Remove-AppxPackage -ErrorAction SilentlyContinue"
                ),
            ])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .output();
    }
}

pub fn wpf_tweaks_remove_home() {
    println!("[*] Removing home from explorer...");

    unsafe {
        // HKCU\Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced
        let key_path1 = w!("SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Advanced");
        let mut key1 = HKEY::default();
        if RegOpenKeyExW(
            HKEY_CURRENT_USER,
            key_path1,
            Some(0),
            KEY_SET_VALUE,
            &raw mut key1,
        ) == ERROR_SUCCESS
        {
            let value: u32 = 0;
            let _ = RegSetValueExW(
                key1,
                w!("Start_IrisRecommendations"),
                Some(0),
                REG_DWORD,
                Some(&value.to_le_bytes()),
            );
            let _ = RegCloseKey(key1);
        }

        // HKCU\Software\Microsoft\Windows\CurrentVersion\Policies\Explorer
        let key_path2 = w!("SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Policies\\Explorer");
        let mut key2 = HKEY::default();
        if RegCreateKeyExW(
            HKEY_CURRENT_USER,
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
                w!("NoStartMenuPinnedList"),
                Some(0),
                REG_DWORD,
                Some(&value.to_le_bytes()),
            );
            let _ = RegCloseKey(key2);
        }
    }

    println!("[+] Home removed from explorer");
}

pub fn wpf_tweaks_ah() {
    println!("[*] Configuring Action Center/Notification settings...");

    unsafe {
        // HKCU\Software\Microsoft\Windows\CurrentVersion\PushNotifications
        let key_path1 = w!("SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\PushNotifications");
        let mut key1 = HKEY::default();
        if RegOpenKeyExW(
            HKEY_CURRENT_USER,
            key_path1,
            Some(0),
            KEY_SET_VALUE,
            &raw mut key1,
        ) == ERROR_SUCCESS
        {
            let value: u32 = 0;
            let _ = RegSetValueExW(
                key1,
                w!("ToastEnabled"),
                Some(0),
                REG_DWORD,
                Some(&value.to_le_bytes()),
            );
            let _ = RegCloseKey(key1);
        }

        // HKCU\Software\Policies\Microsoft\Windows\Explorer
        let key_path2 = w!("SOFTWARE\\Policies\\Microsoft\\Windows\\Explorer");
        let mut key2 = HKEY::default();
        if RegCreateKeyExW(
            HKEY_CURRENT_USER,
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
                w!("DisableNotificationCenter"),
                Some(0),
                REG_DWORD,
                Some(&value.to_le_bytes()),
            );
            let _ = RegCloseKey(key2);
        }
    }

    println!("[+] Action Center/Notification settings configured");
}

pub fn wpf_tweaks_consumer_features() {
    println!("[*] Disabling consumer features...");

    unsafe {
        let key_path = w!("SOFTWARE\\Policies\\Microsoft\\Windows\\CloudContent");
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
            let value: u32 = 1;
            let _ = RegSetValueExW(
                key,
                w!("DisableWindowsConsumerFeatures"),
                Some(0),
                REG_DWORD,
                Some(&value.to_le_bytes()),
            );
            let _ = RegSetValueExW(
                key,
                w!("DisableCloudOptimizedContent"),
                Some(0),
                REG_DWORD,
                Some(&value.to_le_bytes()),
            );
            let _ = RegCloseKey(key);
        }
    }

    println!("[+] Consumer features disabled");
}

pub fn wpf_tweaks_remove_copilot() {
    println!("[*] Removing Copilot...");

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
                w!("ShowCopilotButton"),
                Some(0),
                REG_DWORD,
                Some(&value.to_le_bytes()),
            );
            let _ = RegCloseKey(key);
        }
    }

    println!("[+] Copilot removed");
}

pub fn wpf_tweaks_wifi() {
    println!("[*] Configuring WiFi settings...");

    unsafe {
        // HKLM\SOFTWARE\Microsoft\WcmSvc\wifinetworkmanager\config
        let key_path1 = w!("SOFTWARE\\Microsoft\\WcmSvc\\wifinetworkmanager\\config");
        let mut key1 = HKEY::default();
        if RegOpenKeyExW(
            HKEY_LOCAL_MACHINE,
            key_path1,
            Some(0),
            KEY_SET_VALUE,
            &raw mut key1,
        ) == ERROR_SUCCESS
        {
            let value: u32 = 0;
            let _ = RegSetValueExW(
                key1,
                w!("AutoConnectAllowedOEM"),
                Some(0),
                REG_DWORD,
                Some(&value.to_le_bytes()),
            );
            let _ = RegCloseKey(key1);
        }

        // HKLM\SOFTWARE\Microsoft\PolicyManager\default\WiFi\AllowWiFiHotspotReporting
        let key_path2 =
            w!("SOFTWARE\\Microsoft\\PolicyManager\\default\\WiFi\\AllowWiFiHotspotReporting");
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
                w!("value"),
                Some(0),
                REG_DWORD,
                Some(&value.to_le_bytes()),
            );
            let _ = RegCloseKey(key2);
        }
    }

    println!("[+] WiFi settings configured");
}

pub fn wpf_tweaks_loc() {}

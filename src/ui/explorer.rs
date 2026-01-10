use std::process::Command;

pub fn unpin_start_menu() {
    println!("    Unpinning Start menu items...");
    let _ = Command::new("powershell")
        .args(["-NoProfile", "-ExecutionPolicy", "Bypass", "-Command", r#" $key = Get-ItemProperty -Path "HKCU:\Software\Microsoft\Windows\CurrentVersion\CloudStore\Store\DefaultAccount\*start.tilegrid$windows.data.curatedtilecollection.tilecollection\Current" $key.Data[0..25] = 1 $key.Data[26..518] = 0 Set-ItemProperty -Path $key.PSPath -Name "Data" -Type Binary -Value $key.Data "#])
        .output();
}

pub fn enable_powershell_execution() {
    println!("    Setting PowerShell execution policy to Unrestricted...");
    let _ = Command::new("powershell")
        .args([
            "-NoProfile",
            "-Command",
            "Set-ExecutionPolicy Unrestricted -Force",
        ])
        .output();
}

pub fn set_windows_terminal_default() {
    println!("    Setting Windows Terminal as default console...");
    let _ = Command::new("powershell")
        .args(["-NoProfile", "-ExecutionPolicy", "Bypass", "-Command", r#"Set-ItemProperty -Path "HKCU:\Console\%%Startup" -Name "DelegationConsole" -Value "{2EACA947-7F5F-4CFA-BA87-8F7FBEEFBE69}"; Set-ItemProperty -Path "HKCU:\Console\%%Startup" -Name "DelegationTerminal" -Value "{E12CFF52-A866-4C77-9A90-F570A7AA2C6B}""#])
        .output();
}

pub fn remove_edge_shortcut() {
    println!("    Removing Edge desktop shortcut...");
    if let Some(mut desktop) = dirs::desktop_dir() {
        desktop.push("Microsoft Edge.lnk");
        if desktop.exists() {
            let _ = std::fs::remove_file(&desktop);
        }
    }
}

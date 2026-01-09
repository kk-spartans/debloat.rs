mod apps;
mod debloat;
mod system;
mod tweaks;
mod ui;

use dirs::home_dir;
use std::process;

use debloat::apps_remove::remove_built_in_apps;
use debloat::edge_vanisher::remove_edge;
use debloat::uninstall_oo::uninstall_outlook_onedrive;
use debloat::win11debloat::apply_win11debloat;
use system::admin_check::check_admin;
use system::winutil::apply_winutil_tweaks;
use tweaks::registry::apply_registry_tweaks;

use ui::dark_mode::{enable_dark_mode, enable_transparency};
use ui::snap_button::enable_snap_to_default_button;
use ui::taskbar::set_taskbar_autohide;
use ui::wallpaper::{download_wallpaper, set_wallpaper_desktop};

fn main() {
    // Check for admin privileges
    if let Err(e) = check_admin() {
        eprintln!("[ERROR] {e}");
        process::exit(1);
    }

    // Execute initial debloating scripts
    println!("\n[STEP 1] Executing initial debloating scripts...");
    println!("\n[*] Removing Microsoft Edge");
    if let Err(e) = remove_edge() {
        eprintln!("[ERROR] {e}");
        process::exit(1);
    }
    println!("[+] Successfully removed Microsoft Edge");

    println!("\n[*] Uninstalling Outlook and OneDrive");
    if let Err(e) = uninstall_outlook_onedrive() {
        eprintln!("[ERROR] {e}");
        process::exit(1);
    }
    println!("[+] Successfully uninstalled Outlook and OneDrive");

    // Apply system tweaks
    println!("\n[STEP 2] Applying system tweaks...");
    apply_winutil_tweaks();

    // Apply Windows 11 debloat optimizations
    println!("\n[STEP 3] Applying Windows 11 optimizations...");
    apply_win11debloat();

    // Remove built-in apps
    println!("\n[STEP 4] Removing built-in apps...");
    remove_built_in_apps();

    // Apply registry tweaks
    println!("\n[STEP 5] Applying registry tweaks...");
    if let Err(e) = apply_registry_tweaks() {
        eprintln!("[ERROR] {e}");
        process::exit(1);
    }

    // Apply system tweaks
    println!("\n[STEP 7] Applying system tweaks...");
    if let Err(e) = apply_system_tweaks() {
        eprintln!("[ERROR] {e}");
        process::exit(1);
    }

    println!("\nAll debloating steps complete!");
}

fn apply_system_tweaks() -> Result<(), String> {
    // Build wallpaper path: <userhome>\wallpaper.jpg
    if let Some(mut path) = home_dir() {
        path.push("wallpaper.jpg");
        let path_str = path.to_string_lossy().to_string();

        let url = "https://raw.githubusercontent.com/kk-spartans/dotfiles/refs/heads/main/wallpaper.jpg";
        download_wallpaper(url, &path_str)
            .map_err(|e| format!("Failed to download wallpaper: {e}"))?;

        set_wallpaper_desktop(&path_str).map_err(|e| format!("Failed to set wallpaper: {e}"))?;
    }

    // Taskbar autohide
    set_taskbar_autohide(true);

    // Enable dark mode
    enable_dark_mode().map_err(|e| format!("Failed to enable dark mode: {e}"))?;

    // Enable transparency effects
    enable_transparency().map_err(|e| format!("Failed to enable transparency: {e}"))?;

    // Enable snap to default button
    enable_snap_to_default_button(true)
        .map_err(|e| format!("Failed to enable snap to default button: {e}"))?;

    // Delete Microsoft Edge desktop shortcut
    if let Some(mut desktop) = dirs::desktop_dir() {
        desktop.push("Microsoft Edge.lnk");
        if desktop.exists() {
            std::fs::remove_file(&desktop)
                .map_err(|e| format!("Failed to delete Edge shortcut: {e}"))?;
        }
    }

    // Unpin everything from Start menu
    unpin_start_menu();

    // Enable PowerShell script execution
    enable_powershell_execution();

    // Set Windows Terminal as default terminal
    set_windows_terminal_default();

    Ok(())
}

fn unpin_start_menu() {
    println!("[*] Unpinning everything from Start menu...");
    let _ = std::process::Command::new("powershell")
        .args([
            "-NoProfile",
            "-ExecutionPolicy",
            "Bypass",
            "-Command",
            r#"
$key = Get-ItemProperty -Path "HKCU:\Software\Microsoft\Windows\CurrentVersion\CloudStore\Store\Cache\DefaultAccount\*start.tilegrid$windows.data.curatedtilecollection.tilecollection\Current"
$key.Data[0..25] = 1
$key.Data[26..518] = 0
Set-ItemProperty -Path $key.PSPath -Name "Data" -Type Binary -Value $key.Data
"#,
        ])
        .output();
    println!("[+] Unpinned everything from Start menu");
}

fn enable_powershell_execution() {
    println!("[*] Enabling PowerShell script execution...");
    let _ = std::process::Command::new("powershell")
        .args([
            "-NoProfile",
            "-Command",
            "Set-ExecutionPolicy Unrestricted -Force",
        ])
        .output();
    println!("[+] Enabled PowerShell script execution");
}

fn set_windows_terminal_default() {
    println!("[*] Setting Windows Terminal as default terminal...");
    let _ = std::process::Command::new("powershell")
        .args([
            "-NoProfile",
            "-ExecutionPolicy",
            "Bypass",
            "-Command",
            r#"Set-ItemProperty -Path "HKCU:\Console\%%Startup" -Name "DelegationConsole" -Value "{2EACA947-7F5F-4CFA-BA87-8F7FBEEFBE69}"; Set-ItemProperty -Path "HKCU:\Console\%%Startup" -Name "DelegationTerminal" -Value "{E12CFF52-A866-4C77-9A90-F570A7AA2C6B}""#,
        ])
        .output();
    println!("[+] Set Windows Terminal as default terminal");
}

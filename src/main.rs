mod apps;
mod debloat;
mod system;
mod tweaks;
mod ui;

use std::process;

use debloat::apps_remove::remove_built_in_apps;
use debloat::edge_vanisher::remove_edge;
use debloat::system_debloat::apply_debloat_tweaks;
use debloat::uninstall_oo::uninstall_outlook_onedrive;
use system::admin_check::{check_admin, elevate_and_continue};
use system::winutil::apply_winutil_tweaks;
use tweaks::registry::apply_registry_tweaks;
use ui::dark_mode::{enable_dark_mode, enable_transparency};
use ui::explorer::{
    enable_powershell_execution, remove_edge_shortcut, set_windows_terminal_default,
    unpin_start_menu,
};
use ui::snap_button::enable_snap_to_default_button;
use ui::taskbar::set_taskbar_autohide;
use ui::wallpaper::{download_wallpaper, set_wallpaper_desktop};

fn main() {
    println!("[START] Checking administrator privileges...");

    if check_admin().is_err() {
        println!("[WARN] Not running as Administrator. Attempting to elevate...");
        elevate_and_continue();
    }

    println!("[OK] Administrator privileges confirmed.");

    println!("[START] Removing Microsoft Edge...");
    if let Err(e) = remove_edge() {
        eprintln!("[ERROR] Failed to remove Edge: {e}");
        process::exit(1);
    }
    println!("[OK] Microsoft Edge removed.");

    println!("[START] Uninstalling Outlook and OneDrive...");
    if let Err(e) = uninstall_outlook_onedrive() {
        eprintln!("[ERROR] Failed to remove Outlook/OneDrive: {e}");
        process::exit(1);
    }
    println!("[OK] Outlook and OneDrive uninstalled.");

    println!("[START] Applying WinUtil tweaks...");
    apply_winutil_tweaks();
    println!("[OK] WinUtil tweaks applied.");

    println!("[START] Applying debloat tweaks...");
    apply_debloat_tweaks();
    println!("[OK] Debloat tweaks applied.");

    println!("[START] Removing built-in apps...");
    if let Err(e) = remove_built_in_apps() {
        eprintln!("[ERROR] Failed to remove built-in apps: {e}");
        process::exit(1);
    }
    println!("[OK] Built-in apps removal complete.");

    println!("[START] Applying registry tweaks...");
    if let Err(e) = apply_registry_tweaks() {
        eprintln!("[ERROR] Failed to apply registry tweaks: {e}");
        process::exit(1);
    }
    println!("[OK] Registry tweaks applied.");

    println!("[START] Applying UI tweaks...");
    if let Err(e) = apply_ui_tweaks() {
        eprintln!("[ERROR] Failed to apply UI tweaks: {e}");
        process::exit(1);
    }
    println!("[OK] UI tweaks applied.");

    println!("[DONE] All debloating operations completed successfully.");
}

fn apply_ui_tweaks() -> Result<(), String> {
    println!("  Downloading wallpaper...");
    if let Some(mut path) = dirs::home_dir() {
        path.push("wallpaper.jpg");
        let path_str = path.to_string_lossy().to_string();

        let url =
            "https://raw.githubusercontent.com/kk-spartans/dotfiles/refs/heads/main/wallpaper.jpg";
        download_wallpaper(url, &path_str)
            .map_err(|e| format!("Failed to download wallpaper: {e}"))?;
        set_wallpaper_desktop(&path_str).map_err(|e| format!("Failed to set wallpaper: {e}"))?;
    }

    println!("  Setting taskbar to autohide...");
    set_taskbar_autohide(true);

    println!("  Enabling dark mode...");
    enable_dark_mode().map_err(|e| format!("Failed to enable dark mode: {e}"))?;

    println!("  Enabling transparency...");
    enable_transparency().map_err(|e| format!("Failed to enable transparency: {e}"))?;

    println!("  Enabling snap to default button...");
    enable_snap_to_default_button(true)
        .map_err(|e| format!("Failed to enable snap button: {e}"))?;

    println!("  Removing Edge shortcut...");
    remove_edge_shortcut();

    println!("  Unpinning Start menu...");
    unpin_start_menu();

    println!("  Enabling PowerShell execution policy...");
    enable_powershell_execution();

    println!("  Setting Windows Terminal as default...");
    set_windows_terminal_default();

    Ok(())
}

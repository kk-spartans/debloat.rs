mod apps;
mod cli;
mod debloat;
mod system;
mod tweaks;
mod ui;

use std::io::{self, Read};
use std::process;

use clap::Parser;
use cli::Cli;
use debloat::apps_remove::remove_built_in_apps;
use debloat::edge_vanisher::remove_edge;
use debloat::system_debloat::apply_debloat_tweaks;
use debloat::uninstall_oo::uninstall_outlook_onedrive;
use system::privacy_tweaks::apply_privacy_tweaks;
use tracing::{error, info};
use tracing_subscriber::fmt::format::FmtSpan;
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
    let cli = Cli::parse();

    tracing_subscriber::fmt()
        .with_max_level(cli.log_level())
        .with_span_events(FmtSpan::NONE)
        .with_target(false)
        .without_time()
        .init();

    if !cli.no_edge_removal {
        info!("Removing Microsoft Edge...");
        if let Err(e) = remove_edge() {
            error!("Failed to remove Edge: {e}");
            process::exit(1);
        }
        info!("Microsoft Edge removed.");
    }

    if !cli.no_outlook_onedrive {
        info!("Uninstalling Outlook and OneDrive...");
        if let Err(e) = uninstall_outlook_onedrive() {
            error!("Failed to remove Outlook/OneDrive: {e}");
            process::exit(1);
        }
        info!("Outlook and OneDrive uninstalled.");
    }

    if !cli.no_privacy_tweaks {
        info!("Applying privacy and system tweaks...");
        apply_privacy_tweaks();
        info!("Privacy and system tweaks applied.");
    }

    if !cli.no_debloat_tweaks {
        info!("Applying debloat tweaks...");
        apply_debloat_tweaks();
        info!("Debloat tweaks applied.");
    }

    if !cli.no_builtin_apps {
        info!("Removing built-in apps...");
        if let Err(e) = remove_built_in_apps() {
            error!("Failed to remove built-in apps: {e}");
            process::exit(1);
        }
        info!("Built-in apps removal complete.");
    }

    if !cli.no_registry_tweaks {
        info!("Applying registry tweaks...");
        if let Err(e) = apply_registry_tweaks() {
            error!("Failed to apply registry tweaks: {e}");
            process::exit(1);
        }
        info!("Registry tweaks applied.");
    }

    info!("Applying UI tweaks...");
    if let Err(e) = apply_ui_tweaks(&cli) {
        error!("Failed to apply UI tweaks: {e}");
        process::exit(1);
    }
    info!("UI tweaks applied.");

    info!("All debloating operations completed successfully.");

    if cli.debug {
        println!("\nPress Enter to exit...");
        let _ = io::stdin().read(&mut [0u8]);
    }
}

fn apply_ui_tweaks(cli: &Cli) -> Result<(), String> {
    if !cli.no_wallpaper {
        info!("Downloading wallpaper...");
        if let Some(mut path) = dirs::home_dir() {
            path.push("wallpaper.jpg");
            let path_str = path.to_string_lossy().to_string();

            let url = "https://raw.githubusercontent.com/kk-spartans/dotfiles/refs/heads/main/wallpaper.jpg";
            download_wallpaper(url, &path_str)
                .map_err(|e| format!("Failed to download wallpaper: {e}"))?;
            set_wallpaper_desktop(&path_str)
                .map_err(|e| format!("Failed to set wallpaper: {e}"))?;
        }
    }

    if !cli.no_taskbar_autohide {
        info!("Setting taskbar to autohide...");
        set_taskbar_autohide(true);
    }

    if !cli.no_dark_mode {
        info!("Enabling dark mode...");
        enable_dark_mode().map_err(|e| format!("Failed to enable dark mode: {e}"))?;

        info!("Enabling transparency...");
        enable_transparency().map_err(|e| format!("Failed to enable transparency: {e}"))?;
    }

    info!("Enabling snap to default button...");
    enable_snap_to_default_button(true)
        .map_err(|e| format!("Failed to enable snap button: {e}"))?;

    info!("Removing Edge shortcut...");
    remove_edge_shortcut();

    info!("Unpinning Start menu...");
    unpin_start_menu();

    info!("Enabling PowerShell execution policy...");
    enable_powershell_execution();

    info!("Setting Windows Terminal as default...");
    set_windows_terminal_default();

    Ok(())
}

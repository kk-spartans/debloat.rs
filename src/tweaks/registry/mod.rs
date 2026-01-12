use std::process::Command;
use tracing::debug;
pub mod registry_explorer;
pub mod registry_helpers;
pub mod registry_system;
pub mod registry_ui;

pub use registry_explorer::apply_explorer_tweaks;
pub use registry_system::apply_system_tweaks;
pub use registry_ui::apply_ui_tweaks;

pub fn apply_registry_tweaks() -> Result<(), String> {
    debug!("Applying Explorer tweaks...");
    apply_explorer_tweaks()?;
    debug!("Applying UI tweaks...");
    apply_ui_tweaks()?;
    debug!("Applying System tweaks...");
    apply_system_tweaks()?;

    debug!("Updating per-user system parameters...");
    Command::new("rundll32.exe")
        .args(["user32.dll,UpdatePerUserSystemParameters", "1,", "True"])
        .status()
        .map_err(|e| format!("Failed to update per-user system parameters: {e}"))?;

    debug!("Restarting Explorer...");
    Command::new("taskkill")
        .args(["/F", "/IM", "explorer.exe"])
        .status()
        .map_err(|e| format!("Failed to kill explorer: {e}"))?;
    Command::new("explorer.exe")
        .status()
        .map_err(|e| format!("Failed to start explorer: {e}"))?;

    Ok(())
}

use crate::apps::app_removal::remove_apps;
use crate::system::telemetry::{disable_bing, disable_telemetry};
use crate::tweaks::system_tweaks::registry_accessibility_tweaks::{
    disable_mouse_acceleration, disable_sticky_keys,
};
use crate::tweaks::system_tweaks::registry_content_tweaks::{
    disable_desktop_spotlight, disable_notepad_ai, disable_paint_ai, disable_settings_365_ads,
    disable_settings_home,
};
use crate::tweaks::system_tweaks::registry_explorer_tweaks::{disable_dvr, explorer_to_this_pc};
use crate::tweaks::tweaks::debloat_tweaks::remove_bloatware;
use crate::ui::ui_features::{
    clear_start_all_users, disable_copilot, disable_lockscreen_tips, disable_start_recommended,
    disable_suggestions, disable_widgets, hide_pen_menu, hide_search_tb, hide_task_view,
    show_virtual_keyboard,
};
use tracing::debug;

pub fn apply_debloat_tweaks() {
    debug!("Removing apps...");
    remove_apps();
    debug!("Removing bloatware...");
    remove_bloatware();
    debug!("Disabling telemetry...");
    disable_telemetry();
    debug!("Disabling Bing...");
    disable_bing();
    debug!("Disabling suggestions...");
    disable_suggestions();
    debug!("Disabling lockscreen tips...");
    disable_lockscreen_tips();
    debug!("Hiding search taskbar...");
    hide_search_tb();
    debug!("Hiding task view...");
    hide_task_view();
    debug!("Disabling widgets...");
    disable_widgets();
    debug!("Hiding pen menu...");
    hide_pen_menu();
    debug!("Showing virtual keyboard...");
    show_virtual_keyboard();
    debug!("Disabling Copilot...");
    disable_copilot();
    debug!("Clearing start menu for all users...");
    clear_start_all_users();
    debug!("Disabling DVR...");
    disable_dvr();
    debug!("Disabling start recommendations...");
    disable_start_recommended();
    debug!("Setting Explorer to This PC...");
    explorer_to_this_pc();
    debug!("Disabling mouse acceleration...");
    disable_mouse_acceleration();
    debug!("Disabling desktop spotlight...");
    disable_desktop_spotlight();
    debug!("Disabling Settings 365 ads...");
    disable_settings_365_ads();
    debug!("Disabling Settings home...");
    disable_settings_home();
    debug!("Disabling Paint AI...");
    disable_paint_ai();
    debug!("Disabling Notepad AI...");
    disable_notepad_ai();
    debug!("Disabling sticky keys...");
    disable_sticky_keys();
}

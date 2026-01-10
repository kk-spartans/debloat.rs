use crate::apps::app_removal::remove_apps;
use crate::system::telemetry::{disable_bing, disable_telemetry};
use crate::tweaks::tweaks::debloat_tweaks::remove_bloatware;
use crate::tweaks::system_tweaks::registry_content_tweaks::{
    disable_desktop_spotlight, disable_notepad_ai, disable_paint_ai, disable_settings_365_ads,
    disable_settings_home,
};
use crate::tweaks::system_tweaks::registry_explorer_tweaks::{disable_dvr, explorer_to_this_pc};
use crate::tweaks::system_tweaks::registry_accessibility_tweaks::{disable_mouse_acceleration, disable_sticky_keys};
use crate::ui::ui_features::{
    clear_start_all_users, disable_copilot, disable_lockscreen_tips, disable_start_recommended,
    disable_suggestions, disable_widgets, hide_pen_menu, hide_search_tb, hide_task_view,
    show_virtual_keyboard,
};

pub fn apply_debloat_tweaks() {
    println!("    Removing apps...");
    remove_apps();
    println!("    Removing bloatware...");
    remove_bloatware();
    println!("    Disabling telemetry...");
    disable_telemetry();
    println!("    Disabling Bing...");
    disable_bing();
    println!("    Disabling suggestions...");
    disable_suggestions();
    println!("    Disabling lockscreen tips...");
    disable_lockscreen_tips();
    println!("    Hiding search taskbar...");
    hide_search_tb();
    println!("    Hiding task view...");
    hide_task_view();
    println!("    Disabling widgets...");
    disable_widgets();
    println!("    Hiding pen menu...");
    hide_pen_menu();
    println!("    Showing virtual keyboard...");
    show_virtual_keyboard();
    println!("    Disabling Copilot...");
    disable_copilot();
    println!("    Clearing start menu for all users...");
    clear_start_all_users();
    println!("    Disabling DVR...");
    disable_dvr();
    println!("    Disabling start recommendations...");
    disable_start_recommended();
    println!("    Setting Explorer to This PC...");
    explorer_to_this_pc();
    println!("    Disabling mouse acceleration...");
    disable_mouse_acceleration();
    println!("    Disabling desktop spotlight...");
    disable_desktop_spotlight();
    println!("    Disabling Settings 365 ads...");
    disable_settings_365_ads();
    println!("    Disabling Settings home...");
    disable_settings_home();
    println!("    Disabling Paint AI...");
    disable_paint_ai();
    println!("    Disabling Notepad AI...");
    disable_notepad_ai();
    println!("    Disabling sticky keys...");
    disable_sticky_keys();
}

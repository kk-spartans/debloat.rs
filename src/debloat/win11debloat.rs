use crate::apps::apps::remove_apps;
use crate::system::telemetry::{disable_bing, disable_telemetry};
use crate::tweaks::system_tweaks::{
    disable_desktop_spotlight, disable_dvr, disable_mouse_acceleration, disable_notepad_ai,
    disable_paint_ai, disable_settings_365_ads, disable_settings_home, disable_sticky_keys,
    explorer_to_this_pc,
};
use crate::ui::ui_features::{
    clear_start_all_users, disable_copilot, disable_lockscreen_tips, disable_start_recommended,
    disable_suggestions, disable_widgets, hide_pen_menu, hide_search_tb, hide_task_view,
    show_virtual_keyboard,
};

pub fn apply_win11debloat() {
    println!("[*] Executing Win11Debloat with optimizations...");

    // Apply all flags (Silent mode is implicit - no prompts)
    remove_apps();
    disable_telemetry();
    disable_bing();
    disable_suggestions();
    disable_lockscreen_tips();
    hide_search_tb();
    hide_task_view();
    disable_widgets();
    hide_pen_menu();
    show_virtual_keyboard();
    disable_copilot();
    clear_start_all_users();
    disable_dvr();
    disable_start_recommended();
    explorer_to_this_pc();
    disable_mouse_acceleration();
    disable_desktop_spotlight();
    disable_settings_365_ads();
    disable_settings_home();
    disable_paint_ai();
    disable_notepad_ai();
    disable_sticky_keys();

    println!("[+] Successfully executed Win11Debloat");
}

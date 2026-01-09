use crate::system::services::{
    wpf_tweaks_disable_bgapps, wpf_tweaks_disable_explorer_auto_discovery, wpf_tweaks_disable_lms1,
    wpf_tweaks_disable_wpbt_execution, wpf_tweaks_dvr, wpf_tweaks_recall_off,
    wpf_tweaks_remove_onedrive, wpf_tweaks_services, wpf_tweaks_tele,
};
use crate::tweaks::tweaks::{
    wpf_tweaks_ah, wpf_tweaks_consumer_features, wpf_tweaks_debloat, wpf_tweaks_loc,
    wpf_tweaks_remove_copilot, wpf_tweaks_remove_home, wpf_tweaks_wifi,
};

pub fn apply_winutil_tweaks() {
    println!("[*] Executing WinUtil with configuration...");

    // Apply all WPFTweaks from config
    wpf_tweaks_debloat();
    wpf_tweaks_disable_lms1();
    wpf_tweaks_remove_home();
    wpf_tweaks_ah();
    let _ = wpf_tweaks_services();
    wpf_tweaks_consumer_features();
    wpf_tweaks_remove_copilot();
    wpf_tweaks_wifi();
    wpf_tweaks_tele();
    wpf_tweaks_disable_bgapps();
    wpf_tweaks_remove_onedrive();
    wpf_tweaks_recall_off();
    wpf_tweaks_loc();
    wpf_tweaks_disable_wpbt_execution();
    wpf_tweaks_dvr();
    wpf_tweaks_disable_explorer_auto_discovery();

    println!("[+] Successfully executed WinUtil");
}

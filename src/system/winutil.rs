use crate::system::registry_tweaks::{
    disable_background_apps, disable_explorer_auto_discovery, disable_location_services,
    disable_recall, disable_telemetry_registry, disable_wpbt, remove_onedrive_registry,
};
use crate::system::services::wpf_tweaks_services;
use crate::tweaks::tweaks::debloat_tweaks::remove_bloatware;
use crate::tweaks::tweaks::explorer_tweaks::remove_home_from_explorer;
use crate::tweaks::tweaks::feature_tweaks::{
    configure_wifi, disable_action_center, disable_consumer_features, remove_copilot,
};
use crate::tweaks::tweaks::wpf_tweaks_loc;

pub fn apply_winutil_tweaks() {
    println!("    Removing bloatware...");
    remove_bloatware();
    println!("    Disabling location services...");
    disable_location_services();
    println!("    Removing home from Explorer...");
    remove_home_from_explorer();
    println!("    Disabling action center...");
    disable_action_center();
    println!("    Applying WPF tweaks services...");
    wpf_tweaks_services();
    println!("    Disabling consumer features...");
    disable_consumer_features();
    println!("    Removing Copilot...");
    remove_copilot();
    println!("    Configuring WiFi...");
    configure_wifi();
    println!("    Disabling telemetry registry...");
    disable_telemetry_registry();
    println!("    Disabling background apps...");
    disable_background_apps();
    println!("    Removing OneDrive registry...");
    remove_onedrive_registry();
    println!("    Disabling Recall...");
    disable_recall();
    println!("    Applying WPF tweaks location...");
    wpf_tweaks_loc();
    println!("    Disabling WPBT...");
    disable_wpbt();
    println!("    Disabling Recall (again)...");
    disable_recall();
    println!("    Disabling Explorer auto discovery...");
    disable_explorer_auto_discovery();
}

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
use tracing::debug;

pub fn apply_winutil_tweaks() {
    debug!("Removing bloatware...");
    remove_bloatware();
    debug!("Disabling location services...");
    disable_location_services();
    debug!("Removing home from Explorer...");
    remove_home_from_explorer();
    debug!("Disabling action center...");
    disable_action_center();
    debug!("Applying WPF tweaks services...");
    wpf_tweaks_services();
    debug!("Disabling consumer features...");
    disable_consumer_features();
    debug!("Removing Copilot...");
    remove_copilot();
    debug!("Configuring WiFi...");
    configure_wifi();
    debug!("Disabling telemetry registry...");
    disable_telemetry_registry();
    debug!("Disabling background apps...");
    disable_background_apps();
    debug!("Removing OneDrive registry...");
    remove_onedrive_registry();
    debug!("Disabling Recall...");
    disable_recall();
    debug!("Applying WPF tweaks location...");
    wpf_tweaks_loc();
    debug!("Disabling WPBT...");
    disable_wpbt();
    debug!("Disabling Recall (again)...");
    disable_recall();
    debug!("Disabling Explorer auto discovery...");
    disable_explorer_auto_discovery();
}

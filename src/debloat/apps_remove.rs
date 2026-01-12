use crate::apps::app_removal::{DEBLOAT_APPS, remove_app};
use tracing::debug;

pub fn remove_built_in_apps() -> Result<(), String> {
    let total = DEBLOAT_APPS.len();
    debug!("Removing {total} built-in apps...");

    for app in DEBLOAT_APPS {
        remove_app(app)?;
    }

    debug!("App removal complete.");
    Ok(())
}

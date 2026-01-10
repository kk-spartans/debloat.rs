use crate::apps::app_removal::{remove_app, DEBLOAT_APPS};

pub fn remove_built_in_apps() -> Result<(), String> {
    let total = DEBLOAT_APPS.len();
    println!("    Removing {total} built-in apps...");

    for app in DEBLOAT_APPS {
        remove_app(app)?;
    }

    println!("    App removal complete.");
    Ok(())
}

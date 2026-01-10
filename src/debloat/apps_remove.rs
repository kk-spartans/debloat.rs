use crate::apps::app_removal::{DEBLOAT_APPS, remove_app};

pub fn remove_built_in_apps() -> Result<(), String> {
    let total = DEBLOAT_APPS.len();
    println!("    Removing {total} built-in apps...");

    for app in DEBLOAT_APPS {
        remove_app(app)?;
    }

    println!("    App removal complete.");
    Ok(())
}

use std::env;
use std::path::Path;
use windows::core::w;

use windows::Win32::System::Registry::{HKEY_CURRENT_USER, RegDeleteTreeW};

pub fn clean_taskbar_registry() {
    unsafe {
        let taskbar_keys = [
            w!("SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Taskband"),
            w!("SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Explorer\\TaskbarMRU"),
            w!("SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Explorer\\TaskBar"),
            w!("SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Advanced"),
        ];

        for key in taskbar_keys {
            let _ = RegDeleteTreeW(HKEY_CURRENT_USER, key);
        }
    }
}

pub fn clean_outlook_user_data() {
    let local_appdata = env::var("LOCALAPPDATA").unwrap_or_default();

    let cleanup_paths = [
        format!("{local_appdata}\\Microsoft\\Windows\\Shell\\LayoutModification.xml"),
        format!("{local_appdata}\\Microsoft\\Windows\\Explorer\\iconcache*"),
        format!("{local_appdata}\\Microsoft\\Windows\\Explorer\\thumbcache*"),
    ];

    let mut cleaned_count = 0;
    for path_pattern in cleanup_paths {
        if path_pattern.contains('*') {
            let base_path = path_pattern.split('*').next().unwrap_or("");
            if let Ok(parent) =
                std::fs::read_dir(Path::new(base_path).parent().unwrap_or(Path::new(".")))
            {
                for entry in parent.flatten() {
                    let entry_path = entry.path();
                    if let Some(name) = entry_path.file_name() {
                        let name_str = name.to_string_lossy();
                        let pattern_prefix = base_path.split('\\').next_back().unwrap_or("");
                        if name_str.starts_with(pattern_prefix) {
                            let _ = std::fs::remove_file(&entry_path);
                            cleaned_count += 1;
                        }
                    }
                }
            }
        } else if Path::new(&path_pattern).exists() {
            let _ = std::fs::remove_file(&path_pattern);
            cleaned_count += 1;
        }
    }
    if cleaned_count > 0 {
        println!("    Cleaned {cleaned_count} user data items");
    }
}

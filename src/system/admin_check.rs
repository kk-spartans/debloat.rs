use std::env;
use std::os::windows::ffi::OsStrExt;
use std::process::{Command, exit};
use tracing::error;
use windows::Win32::UI::Shell::ShellExecuteW;
use windows::Win32::UI::WindowsAndMessaging::SW_SHOW;
use windows::core::PCWSTR;
use windows::core::w;

pub fn check_admin() -> Result<(), String> {
    if is_admin() {
        Ok(())
    } else {
        Err("This application must be run as Administrator".to_string())
    }
}

pub fn elevate_and_continue() {
    let exe_path = env::current_exe().expect("Failed to get current executable path");
    let exe_path_wide: Vec<u16> = exe_path
        .as_os_str()
        .encode_wide()
        .chain(std::iter::once(0))
        .collect();

    let result = unsafe {
        ShellExecuteW(
            None,
            PCWSTR::from_raw(w!("runas").as_ptr()),
            PCWSTR::from_raw(exe_path_wide.as_ptr()),
            None,
            None,
            SW_SHOW,
        )
    };

    if result.0 as usize > 32 {
        exit(0);
    } else {
        error!("Failed to elevate privileges. Please run as Administrator.");
        exit(1);
    }
}

fn is_admin() -> bool {
    Command::new("net")
        .args(["session"])
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

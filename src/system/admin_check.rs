use std::env;
use std::os::windows::ffi::OsStrExt;
use std::process::exit;

use tracing::error;
use windows::Win32::Foundation::HANDLE;
use windows::Win32::Security::{GetTokenInformation, TOKEN_ELEVATION, TOKEN_QUERY, TokenElevation};
use windows::Win32::System::Threading::{GetCurrentProcess, OpenProcessToken};
use windows::Win32::UI::Shell::ShellExecuteW;
use windows::Win32::UI::WindowsAndMessaging::SW_SHOW;
use windows::core::{PCWSTR, w};

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

    let args: Vec<String> = env::args().skip(1).collect();
    let args_string = args.join(" ");
    let args_wide: Vec<u16> = args_string
        .encode_utf16()
        .chain(std::iter::once(0))
        .collect();

    let result = unsafe {
        ShellExecuteW(
            None,
            PCWSTR::from_raw(w!("runas").as_ptr()),
            PCWSTR::from_raw(exe_path_wide.as_ptr()),
            PCWSTR::from_raw(args_wide.as_ptr()),
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
    unsafe {
        let mut token_handle = HANDLE::default();
        if OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &raw mut token_handle).is_err() {
            return false;
        }

        let mut elevation = TOKEN_ELEVATION::default();
        let mut return_length = 0u32;

        let result = GetTokenInformation(
            token_handle,
            TokenElevation,
            Some(std::ptr::from_mut(&mut elevation).cast()),
            u32::try_from(std::mem::size_of::<TOKEN_ELEVATION>()).unwrap_or(0),
            &raw mut return_length,
        );

        let _ = windows::Win32::Foundation::CloseHandle(token_handle);

        result.is_ok() && elevation.TokenIsElevated != 0
    }
}

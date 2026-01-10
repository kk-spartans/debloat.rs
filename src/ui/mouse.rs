use windows::Win32::UI::WindowsAndMessaging::{
    SPI_GETMOUSE, SPI_SETMOUSE, SPIF_SENDCHANGE, SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS,
    SystemParametersInfoW,
};
use windows::core::Result as WinResult;

#[allow(dead_code)]
#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::ptr_as_ptr)]
#[allow(clippy::bool_to_int_with_if)]
pub fn set_mouse_accel(enabled: bool) -> WinResult<()> {
    unsafe {
        let mut mouse_params = [0i32; 3];

        let _ = SystemParametersInfoW(
            SPI_GETMOUSE,
            mouse_params.len() as u32,
            Some(mouse_params.as_mut_ptr() as *mut _),
            SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS(0),
        );

        mouse_params[2] = i32::from(enabled);

        SystemParametersInfoW(
            SPI_SETMOUSE,
            mouse_params.len() as u32,
            Some(mouse_params.as_mut_ptr() as *mut _),
            SPIF_SENDCHANGE,
        )?;
    }
    Ok(())
}

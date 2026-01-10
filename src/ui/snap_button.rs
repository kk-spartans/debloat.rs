use windows::Win32::UI::WindowsAndMessaging::{
    SPI_SETSNAPTODEFBUTTON, SPIF_SENDCHANGE, SystemParametersInfoW,
};
use windows::core::Result as WinResult;

pub fn enable_snap_to_default_button(enable: bool) -> WinResult<()> {
    unsafe {
        let flag: u32 = u32::from(enable);
        SystemParametersInfoW(SPI_SETSNAPTODEFBUTTON, flag, None, SPIF_SENDCHANGE)?;
    }
    Ok(())
}

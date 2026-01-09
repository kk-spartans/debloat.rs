use windows::core::Result as WinResult;
use windows::Win32::UI::WindowsAndMessaging::{
    SystemParametersInfoW, SPIF_SENDCHANGE, SPI_SETSNAPTODEFBUTTON,
};

pub fn enable_snap_to_default_button(enable: bool) -> WinResult<()> {
    println!(
        "[*] Setting snap to default button to {}...",
        if enable { "enabled" } else { "disabled" }
    );
    unsafe {
        let flag: u32 = u32::from(enable);
        SystemParametersInfoW(SPI_SETSNAPTODEFBUTTON, flag, None, SPIF_SENDCHANGE)?;
    }
    println!("[+] Snap to default button set");
    Ok(())
}

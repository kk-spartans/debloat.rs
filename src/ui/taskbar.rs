use windows::Win32::Foundation::LPARAM;
use windows::Win32::UI::Shell::{
    ABM_GETSTATE, ABM_SETSTATE, ABS_ALWAYSONTOP, ABS_AUTOHIDE, APPBARDATA, SHAppBarMessage,
};

#[allow(clippy::cast_possible_wrap)]
pub fn set_taskbar_autohide(enable: bool) {
    unsafe {
        let mut abd = APPBARDATA {
            cbSize: u32::try_from(std::mem::size_of::<APPBARDATA>()).unwrap(),
            ..Default::default()
        };

        let state = u32::try_from(SHAppBarMessage(ABM_GETSTATE, &raw mut abd)).unwrap();

        let mut new_state = state & ABS_ALWAYSONTOP;
        if enable {
            new_state |= ABS_AUTOHIDE;
        }

        abd.lParam = LPARAM(new_state as isize);
        SHAppBarMessage(ABM_SETSTATE, &raw mut abd);
    }
}

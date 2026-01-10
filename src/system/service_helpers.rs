use std::ffi::OsStr;
use std::iter::once;
use std::os::windows::ffi::OsStrExt;

pub fn to_wide(s: &str) -> Vec<u16> {
    OsStr::new(s).encode_wide().chain(once(0)).collect()
}

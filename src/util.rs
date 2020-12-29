use libc::c_char;
use std::ffi::{CStr, OsStr};

// Unix specific way to convert from bytes to OsStr
use std::os::unix::ffi::OsStrExt;

pub(crate) unsafe fn char_ptr_to_string(ptr: *mut c_char) -> String {
    let cstr =  CStr::from_ptr(ptr);
    let os_str = OsStr::from_bytes(cstr.to_bytes());
    os_str_to_string(os_str)
}

pub(crate) fn os_str_to_string(os_str: &OsStr) -> String {
    let string = String::from_utf8_lossy(os_str.as_bytes());
    string.into()
}

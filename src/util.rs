use libc::{self, c_char};

use std::{
    ffi::{CStr, OsStr},
    os::unix::ffi::OsStrExt,
    ptr,
};

pub(crate) unsafe fn char_ptr_to_string(ptr: *mut c_char) -> String {
    let cstr = CStr::from_ptr(ptr);
    let os_str = OsStr::from_bytes(cstr.to_bytes());
    os_str_to_string(os_str)
}

pub(crate) fn os_str_to_string(os_str: &OsStr) -> String {
    let string = String::from_utf8_lossy(os_str.as_bytes());
    string.into()
}

/// Simple rand function, wraps over libc::rand
/// It isn't super secure, but we don't really need security
pub(crate) fn get_rand(max: i32) -> i32 {
    unsafe {
        libc::srand(libc::time(ptr::null_mut()) as u32);
        libc::rand() % max
    }
}

// Extracts the last element of a path.
// Example: "/foo/bar/" -> "bar"
pub(crate) fn get_base(path: &str) -> String {
    path.rsplit(|a| a == '/')
            .next()
            .unwrap()
            .to_string()
}
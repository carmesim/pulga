use libc::{c_char, uname, utsname};
use std::mem;
use std::ffi::CStr;

pub struct UnameData {
    pub system_name: String,
    pub node_name:   String,
    pub release:     String,
    pub version:     String,
    pub machine:     String
}

impl UnameData {
    pub fn new () -> UnameData 
    {
        let mut uts_struct = unsafe { mem::zeroed() };
        // utsname is a struct defined in <sys/utsname.h>

        let ret_val = unsafe { uname(& mut uts_struct) };

        // uname returns a negative number upon failure
        if ret_val < 0 {
            // TODO: fallback? panic?
        };

        let sysname_cstr = unsafe { CStr::from_ptr(uts_struct.sysname[..].as_ptr())  };
        let nodename_cstr= unsafe { CStr::from_ptr(uts_struct.nodename[..].as_ptr()) };
        let release_cstr = unsafe { CStr::from_ptr(uts_struct.release[..].as_ptr())  };
        let version_cstr = unsafe { CStr::from_ptr(uts_struct.version[..].as_ptr())  };
        let machine_cstr = unsafe { CStr::from_ptr(uts_struct.machine[..].as_ptr())  };

        UnameData 
        {
            system_name: sysname_cstr.to_string_lossy().into_owned(),
            node_name:   nodename_cstr.to_string_lossy().into_owned(),
            release:     release_cstr.to_string_lossy().into_owned(),
            version:     version_cstr.to_string_lossy().into_owned(),
            machine:     machine_cstr.to_string_lossy().into_owned(),
        }
    }
}

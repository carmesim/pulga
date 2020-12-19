use crate::error::Error;

use libc::{getpwuid_r, getuid, passwd, gethostname, c_char, sysconf, _SC_HOST_NAME_MAX};

use std::{
    cmp,
    collections::HashMap,
    env,
    ffi::{CStr, OsStr, OsString},
    fs, mem,
    str::from_utf8,
    os::unix::ffi::OsStrExt,
    path::PathBuf,
    ptr,
};

#[derive(Debug)]
pub struct UserData {
    pub username: String,     // User's username
    pub hostname: String,     // User's hostname
    pub devicename: String,   // User's device name
    pub cwd: String,          // User's current working directory. TODO: unneeded?
    pub hmd: String,          // User's home directory
    pub shell: String,        // User's standard shell
    pub desk_env: String,     // User's desktop environment
    pub distro: String,       // User's distro
    pub platform: String,     // User's platform
    pub total_memory: String, // Total memory in human-readable form
    pub used_memory: String,  // Used memory in human-readable form
}

#[repr(C)]
#[derive(Debug)]
pub struct MemInfo {
    /// Total physical memory.
    pub total: u64,
    pub free: u64,
    pub avail: u64,

    pub buffers: u64,
    pub cached: u64,

    /// Total swap memory.
    pub swap_total: u64,
    pub swap_free: u64,
}

// This function was adapted from sys-info by Siyu Wang (MIT-licensed)
/// Returns current memory utilization info
pub fn mem_info() -> Result<MemInfo, Error> {
    let s = fs::read_to_string("/proc/meminfo")?;
    let mut mem_info_map = HashMap::new();

    for line in s.lines() {
        let mut split_line = line.split_whitespace();
        let label = split_line.next();
        let value = split_line.next();
        if value.is_some() && label.is_some() {
            let label = label.unwrap().split(':').next().ok_or(Error::Unknown)?;
            let value = value.unwrap().parse::<u64>().ok().ok_or(Error::Unknown)?;
            mem_info_map.insert(label, value);
        }
    }
    let total = mem_info_map.get("MemTotal").ok_or(Error::Unknown)?;
    let free = mem_info_map.get("MemFree").ok_or(Error::Unknown)?;
    let buffers = mem_info_map.get("Buffers").ok_or(Error::Unknown)?;
    let cached = mem_info_map.get("Cached").ok_or(Error::Unknown)?;
    let avail = mem_info_map
        .get("MemAvailable")
        .copied()
        .or_else(|| {
            let sreclaimable = *mem_info_map.get("SReclaimable")?;
            let shmem = mem_info_map.get("Shmem")?;
            Some(free + buffers + cached + sreclaimable - shmem)
        })
        .ok_or(Error::Unknown)?;
    let swap_total = mem_info_map.get("SwapTotal").ok_or(Error::Unknown)?;
    let swap_free = mem_info_map.get("SwapFree").ok_or(Error::Unknown)?;

    Ok(MemInfo {
        total: *total,
        free: *free,
        avail,
        buffers: *buffers,
        cached: *cached,
        swap_total: *swap_total,
        swap_free: *swap_free,
    })
}

/// pretty_bytes gets a value in bytes and returns a human-readable form of it
fn pretty_bytes(num: f64) -> String {
    let negative = if num < 0.0 { "-" } else { "" };
    let num = num.abs();

    const UNITS: &[&str] = &["B", "kB", "MB", "GB", "TB"];
    if num < 1.0 {
        return format!("{}{} {}", negative, num, "B");
    }
    let v1 = (num.ln() / 1024_f64.ln()).floor() as i32;
    let exponent = cmp::min(v1, 4_i32);
    let pretty_bytes = format!("{:.2}", num / 1024_f64.powi(exponent));
    let unit: &str = UNITS[exponent as usize];

    format!("{}{} {}", negative, pretty_bytes, unit)
}

/// get_user_data returns a new UserData structure
pub fn get_user_data() -> UserData {

    let res = get_username_home_dir_and_shell();
    let (username, home_dir, shell) = if res.is_some() {
        res.unwrap()
    } else {
        ("Unknown".to_string(), "Unknown".to_string(), "Unknown".to_string())
    };

    // Current working directory
    let cwd: String = env::current_dir().unwrap().to_string_lossy().into();

    let mem_info = mem_info().unwrap();

    let hostname_res = get_hostname();
    let hostname = if hostname_res.is_some() {
        hostname_res.unwrap()
    } else {
        "Unknown".to_string()
    };
    
    let distro = get_distro();
    let distro = if distro.is_some() {
        distro.unwrap()
    } else {
        "Linux".to_string()
    };

    UserData {
        username,
        hostname,
        devicename: whoami::devicename(),
        cwd,
        hmd: home_dir,
        shell,
        desk_env: whoami::desktop_env().to_string(),
        distro,
        platform: whoami::platform().to_string(),
        total_memory: pretty_bytes((mem_info.total * 1024) as f64),
        used_memory: pretty_bytes(((mem_info.total - mem_info.avail) * 1024) as f64),
    }
}

pub fn get_hostname() -> Option<String> {
    let hostname_max = unsafe { 
        sysconf(_SC_HOST_NAME_MAX) 
    } as usize;
    let mut buffer = vec![0 as u8; hostname_max + 1]; // +1 to account for the NUL character
    let ret = unsafe { 
        gethostname(buffer.as_mut_ptr() as *mut c_char, buffer.len()) 
    };
    if ret != 0 {
        return None;
    }
    let end = buffer
        .iter()
        .position(|&b| b == 0)
        .unwrap_or_else(|| buffer.len());
    buffer.resize(end, 0);
    let hostname = from_utf8(&buffer);
    if hostname.is_err() {
        None
    } else {
        Some(hostname.unwrap().to_string())
    }
}

pub fn get_distro() -> Option<String> {
    let program = std::fs::read_to_string("/etc/os-release");
    if program.is_err() {
        return None;
    }
    let program  = program.unwrap().into_bytes();

    let distro = String::from_utf8_lossy(&program);

    for i in distro.split('\n') {
        let mut j = i.split('=');

        match j.next()? {
            "PRETTY_NAME" => return Some(j.next()?.trim_matches('"').to_string()),
            _ => {}
        }
    }

    Some("Linux".to_string())
}

pub fn get_username_home_dir_and_shell () -> Option<(String, String, String)> {
    let mut buf = Vec::with_capacity(2048);
    let mut result = ptr::null_mut();
  
    let mut passwd : passwd = unsafe{mem::zeroed()};
  
    let getpwuid_r_code =
        unsafe{getpwuid_r(getuid(), &mut passwd, buf.as_mut_ptr(), buf.capacity(),
                          &mut result, )};
  
    if getpwuid_r_code == 0 && !result.is_null() {

        let username_cstr = unsafe{CStr::from_ptr(passwd.pw_name)};
        let username_os_str = OsStr::from_bytes(username_cstr.to_bytes());
        let username : PathBuf = OsString::from(username_os_str).into();
        let username = username.to_string_lossy().into();

        let hd_cstr = unsafe{CStr::from_ptr(passwd.pw_dir)};
        let hd_os_str = OsStr::from_bytes(hd_cstr.to_bytes());
        let hd_path : PathBuf = OsString::from(hd_os_str).into();
        let home_dir = hd_path.to_string_lossy().into();
        
        let sh_cstr = unsafe{CStr::from_ptr(passwd.pw_shell)};
        let sh_os_str = OsStr::from_bytes(sh_cstr.to_bytes());
        let sh_path : PathBuf = OsString::from(sh_os_str).into();
        let shell = sh_path.to_string_lossy().into();

        
        Some((username, home_dir, shell))
      }
    else {
      None
    }
}

pub fn get_shell()->Option<PathBuf> {
    let mut buf = Vec::with_capacity(2048);
    let mut result = ptr::null_mut();
  
    let mut passwd : passwd = unsafe{mem::zeroed()};
  
    let getpwuid_r_code =
        unsafe{getpwuid_r(getuid(), &mut passwd, buf.as_mut_ptr(), buf.capacity(),
                          &mut result, )};
  
    if getpwuid_r_code == 0 && !result.is_null() {
        let cstr = unsafe{CStr::from_ptr(passwd.pw_shell)};
        let os_str = OsStr::from_bytes(cstr.to_bytes());
        let path : PathBuf = OsString::from(os_str).into();
        Some(path)
      }
    else {
      None
    }
}

pub fn get_uptime() -> Option<String> {
    let meminfo = fs::read_to_string("/proc/uptime").ok()?;
    let meminfo: &str = meminfo.split(' ').next()?;
    let uptime_in_centiseconds = meminfo.parse::<f64>();

    let periods = vec![
        (60 * 60 * 24 * 365, "year"),
        (60 * 60 * 24 * 30, "month"),
        (60 * 60 * 24, "day"),
        (60 * 60, "hour"),
        (60, "minute"),
        (1, "second"),
    ];

    // Ignore decimal places
    let mut uptime_in_seconds = uptime_in_centiseconds.ok()? as u64;
    // Result
    let mut uptime = String::new();

    for (period, period_name) in periods {
        let times = uptime_in_seconds / period;

        if times > 0 {
            // Add space between entries
            if !uptime.is_empty() {
                uptime.push(' ');
            }

            uptime.push_str(&format!("{} ", times));

            // Add the "year" period name
            uptime.push_str(period_name);

            // Fix plural
            if times > 1 {
                uptime.push('s');
            }

            // Update for next
            uptime_in_seconds %= period;
        }
    }

    Some(uptime)
}

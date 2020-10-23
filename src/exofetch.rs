
use {std::env, dirs, whoami};
use std::collections::HashMap;
use std::io::{self, Read};

use std::fs::File;

/// Error types
#[derive(Debug)]
pub enum Error {
    IO(io::Error),
    Unknown,
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Error {
        Error::IO(e)
    }
}



#[derive(Debug)]
pub struct UserData {
    pub username:   String,       // User's username
    pub hostname:   String,       // User's hostname
    pub devicename: String,       // User's device name
    pub cwd:        String,       // User's current working directory. TODO: unneeded?
    pub hmd:        String,       // User's home directory
    pub desk_env:   String,       // User's desktop environment
    pub distro:     String,       // User's distro
    pub platform:   String,       // User's platform 
    pub total_memory: u64,
    pub used_memory: u64,   
}

#[repr(C)]
#[derive(Debug)]
pub struct MemInfo {
    /// Total physical memory.
    pub total:   u64,
    pub free:    u64,
    pub avail:   u64,

    pub buffers: u64,
    pub cached:  u64,

    /// Total swap memory.
    pub swap_total: u64,
    pub swap_free:  u64,
}

// This function was adapted from sys-info by Siyu Wang (MIT-licensed)
pub fn mem_info() -> Result<MemInfo, Error> {
    let mut s = String::new();
    File::open("/proc/meminfo")?.read_to_string(&mut s)?;
    let mut mem_info_map = HashMap::new();
    for line in s.lines() 
    {
        let mut split_line = line.split_whitespace();
        let label = split_line.next();
        let value = split_line.next();
        if value.is_some() && label.is_some() 
        {
            let label = label.unwrap().split(':').nth(0).ok_or(Error::Unknown)?;
            let value = value.unwrap().parse::<u64>().ok().ok_or(Error::Unknown)?;
            mem_info_map.insert(label, value);
        }
    }
    let total   = mem_info_map.get("MemTotal").ok_or(Error::Unknown)?;
    let free    = mem_info_map.get("MemFree").ok_or(Error::Unknown)?;
    let buffers = mem_info_map.get("Buffers").ok_or(Error::Unknown)?;
    let cached  = mem_info_map.get("Cached").ok_or(Error::Unknown)?;
    let avail   = mem_info_map.get("MemAvailable").map(|v| v.clone()).or_else(|| 
    {
        let sreclaimable = *mem_info_map.get("SReclaimable")?;
        let shmem = mem_info_map.get("Shmem")?;
        Some(free + buffers + cached + sreclaimable - shmem)
    }).ok_or(Error::Unknown)?;
    let swap_total = mem_info_map.get("SwapTotal").ok_or(Error::Unknown)?;
    let swap_free  = mem_info_map.get("SwapFree").ok_or(Error::Unknown)?;
    Ok(MemInfo 
    {
           total: *total,
           free: *free,
           avail,
           buffers: *buffers,
           cached: *cached,
           swap_total: *swap_total,
           swap_free: *swap_free,
       })
}

/// get_user_data returns a new UserData structure
pub fn get_user_data() -> UserData
{
    // Current working directory
    let cwd = env::current_dir().unwrap();
    let cwd_str: String = cwd.as_os_str().to_str().unwrap().to_string();
    drop(cwd);

    // Home directory
    let hmd = dirs::home_dir().unwrap();
    let hmd_str: String = hmd.as_os_str().to_str().unwrap().to_string();
    drop(hmd);
    
    let mem_info = mem_info().unwrap();

    return UserData
    {
        username:   whoami::username(),
        hostname:   whoami::hostname(),
        devicename: whoami::devicename(),
        cwd:        cwd_str,
        hmd:        hmd_str,
        desk_env:   whoami::desktop_env().to_string(),
        distro:     whoami::distro(),
        platform:   whoami::platform().to_string(),
        total_memory: mem_info.total,
        used_memory:  mem_info.cached, 
    };
}
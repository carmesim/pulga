use crate::{
    util::{char_ptr_to_string, os_str_to_string},
    uname::UnameData,
};

use libc::{
    c_char, gethostname, getpwuid_r, getuid, passwd, sysconf, CPU_ISSET, CPU_SETSIZE,
    _SC_HOST_NAME_MAX,
};

use std::{cmp, collections::HashMap, env, fs, mem, ptr};

#[derive(Debug)]
pub struct UserData {
    pub username: String,       // User's username
    pub hostname: String,       // User's hostname
    pub cpu_info: String,       // Some CPU info
    pub cwd: String,            // User's current working directory. TODO: unneeded?
    pub hmd: String,            // User's home directory
    pub shell: String,          // User's standard shell
    pub desk_env: String,       // User's desktop environment
    pub distro: String,         // User's distro
    pub kernel_version: String, // User's current kernel version
    pub total_memory: String,   // Total memory in human-readable form
    pub used_memory: String,    // Used memory in human-readable form
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
pub fn mem_info() -> Option<MemInfo> {
    let s = fs::read_to_string("/proc/meminfo").ok()?;
    let mut mem_info_map = HashMap::new();

    for line in s.lines() {
        let mut split_line = line.split_whitespace();
        let label = split_line.next();
        let value = split_line.next();
        if value.is_some() && label.is_some() {
            let label = label.unwrap().split(':').next()?;
            let value = value.unwrap().parse::<u64>().ok()?;
            mem_info_map.insert(label, value);
        }
    }
    let total = mem_info_map.get("MemTotal")?;
    let free = mem_info_map.get("MemFree")?;
    let buffers = mem_info_map.get("Buffers")?;
    let cached = mem_info_map.get("Cached")?;
    let avail = mem_info_map
        .get("MemAvailable")
        .copied()
        .or_else(|| {
            let sreclaimable = *mem_info_map.get("SReclaimable")?;
            let shmem = mem_info_map.get("Shmem")?;
            Some(free + buffers + cached + sreclaimable - shmem)
        })
        ?;
    let swap_total = mem_info_map.get("SwapTotal")?;
    let swap_free = mem_info_map.get("SwapFree")?;

    Some(MemInfo {
        total: *total,
        free: *free,
        avail,
        buffers: *buffers,
        cached: *cached,
        swap_total: *swap_total,
        swap_free: *swap_free,
    })
}

/// The number of threads the CPU can handle at any given time
fn get_logical_cpus() -> usize {
    use libc::{cpu_set_t, sched_getaffinity, _SC_NPROCESSORS_ONLN};

    let mut set: cpu_set_t = unsafe { mem::zeroed() };
    let code = unsafe { sched_getaffinity(0, mem::size_of::<cpu_set_t>(), &mut set) };

    // If sched_getaffinity returns 0 (succeeded)
    if code == 0 {
        let mut count = 0;
        for i in 0..CPU_SETSIZE as usize {
            if unsafe { CPU_ISSET(i, &set) } {
                count += 1
            }
        }
        count
    } else {
        let cpus = unsafe { sysconf(_SC_NPROCESSORS_ONLN) };
        cmp::max(1, cpus) as usize
    }
}

pub fn get_cpu_max_freq() -> String {
    let scaling_max_freq_str =
        match std::fs::read_to_string("/sys/devices/system/cpu/cpu0/cpufreq/scaling_max_freq") {
            Ok(freq) => freq,
            Err(_) => return "Unknown Frequency".to_string(),
        };

    let max_freq_hz: usize = match scaling_max_freq_str.trim().parse() {
        Ok(freq) => freq,
        Err(_) => return "Unknown Frequency".to_string(),
    };

    let max_freq_ghz = (max_freq_hz as f64) / 1000000.0;

    format!("{:.2} GHz", max_freq_ghz)
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
    let (username, home_dir, shell) = if let Some(res) = get_username_home_dir_and_shell() {
        res
    } else {
        let unknown = "Unknown".to_string();
        (unknown.clone(), unknown.clone(), unknown)
    };

    // Current working directory
    let cwd: String = os_str_to_string(env::current_dir().unwrap().as_ref());

    let uname_data = UnameData::new();
    
    let mem_info = mem_info().unwrap();

    let hostname = get_hostname().unwrap_or_else(|| "Unknown".to_string());
    let distro = get_distro().unwrap_or_else(|| "Linux".to_string());

    UserData {
        username,
        hostname,
        cpu_info: format!(
            "{}x {} ({})",
            get_logical_cpus(),
            get_cpu_max_freq(),
            uname_data.machine
        ),
        cwd,
        hmd: home_dir,
        shell,
        kernel_version: format!(
            "{} {}", 
            uname_data.system_name, 
            uname_data.release
        ),
        desk_env: get_desktop_environment(),
        distro,
        total_memory: pretty_bytes((mem_info.total * 1024) as f64),
        used_memory: pretty_bytes(((mem_info.total - mem_info.avail) * 1024) as f64),
    }
}

pub fn get_hostname() -> Option<String> {
    let hostname_max = unsafe { sysconf(_SC_HOST_NAME_MAX) } as usize;
    let mut buffer = vec![0_u8; hostname_max + 1]; // +1 to account for the NUL character
    let ret = unsafe { gethostname(buffer.as_mut_ptr() as *mut c_char, buffer.len()) };

    if ret == 0 {
        let end = buffer
            .iter()
            .position(|&b| b == 0)
            .unwrap_or_else(|| buffer.len());

        buffer.resize(end, 0);
        String::from_utf8(buffer).ok()
    } else {
        None
    }
}

pub fn get_distro() -> Option<String> {
    let distro = std::fs::read_to_string("/etc/os-release").ok()?;

    for i in distro.split('\n') {
        let mut j = i.split('=');

        if let "PRETTY_NAME" = j.next()? {
            return Some(j.next()?.trim_matches('"').to_string());
        }
    }

    Some("Linux".to_string())
}

pub fn get_username_home_dir_and_shell() -> Option<(String, String, String)> {
    let mut buf = Vec::with_capacity(2048);
    let mut result = ptr::null_mut();
    let mut passwd: passwd = unsafe { mem::zeroed() };

    let getpwuid_r_code = unsafe {
        getpwuid_r(
            getuid(),
            &mut passwd,
            buf.as_mut_ptr(),
            buf.capacity(),
            &mut result,
        )
    };

    if getpwuid_r_code == 0 && !result.is_null() {
        let username = char_ptr_to_string(passwd.pw_name);
        let home_dir = char_ptr_to_string(passwd.pw_dir);
        // From "/usr/bin/shell" to just "shell"
        let shell = char_ptr_to_string(passwd.pw_shell)
            .rsplit(|a| a == '/')
            .next()
            .unwrap()
            .to_string();

        Some((username, home_dir, shell))
    } else {
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
    // Final result
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

pub fn get_desktop_environment() -> String {
    std::env::var_os("DESKTOP_SESSION")
        .map(|env| env.to_string_lossy().to_string())
        .map(|env| {
            let env = env.to_lowercase();

            if env.contains("gnome") {
                "Gnome"
            } else if env.contains("lxde") {
                "LXDE"
            } else if env.contains("openbox") {
                "OpenBox"
            } else if env.contains("i3") {
                "i3"
            } else if env.contains("ubuntu") {
                "Ubuntu"
            } else if env.contains("plasma5") {
                "KDE"
            } else {
                env.as_ref()
            }
            .to_string()
        })
        .unwrap_or_else(|| String::from("Unknown"))
}

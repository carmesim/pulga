use crate::{
    sysinfo::SysInfo,
    uname::UnameData,
    util::{char_ptr_to_string, os_str_to_string},
};

use libc::{
    c_char, gethostname, getpwuid_r, getuid, passwd, sysconf, CPU_ISSET, CPU_SETSIZE,
    _SC_HOST_NAME_MAX,
};

use smallvec::{smallvec, SmallVec};

use std::{cmp, env, fs, mem, ptr};

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
    pub uptime: String,         // Time elapsed since boot
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

    let hostname = get_hostname().unwrap_or_else(|| "Unknown".to_string());
    let distro = get_distro().unwrap_or_else(|| "Linux".to_string());

    let sys_info = SysInfo::new();

    UserData {
        username,
        hostname,
        cpu_info: format!(
            "{} - {}x {}",
            get_cpu_model().unwrap_or_else(|| "Unknown".to_string()),
            get_logical_cpus(),
            get_cpu_max_freq(),
        ),
        cwd,
        hmd: home_dir,
        shell,
        kernel_version: format!(
            "{} {} {}",
            uname_data.system_name, uname_data.release, uname_data.machine
        ),
        desk_env: get_desktop_environment(),
        distro,
        uptime: get_uptime(
            // We pass to get_uptime the current uptime in seconds
            sys_info.uptime,
        ),
        total_memory: pretty_bytes(sys_info.total_ram as f64),
        used_memory: pretty_bytes(
            (sys_info.total_ram - sys_info.free_ram - sys_info.shared_ram) as f64,
        ),
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

    for line in distro.split('\n').filter(|line| line.len() >= 11) {
        if let "PRETTY_NAME" = &line[..11] {
            return Some(line[13..].trim_matches('"').to_string());
        }
    }

    Some("Linux".to_string())
}

pub fn get_username_home_dir_and_shell() -> Option<(String, String, String)> {
    let mut buf = [0_i8; 2048];
    let mut result = ptr::null_mut();
    let mut passwd: passwd = unsafe { mem::zeroed() };

    let getpwuid_r_code = unsafe {
        getpwuid_r(
            getuid(),
            &mut passwd,
            buf.as_mut_ptr(),
            buf.len(),
            &mut result,
        )
    };

    if getpwuid_r_code == 0 && !result.is_null() {
        let username = unsafe { char_ptr_to_string(passwd.pw_name) };
        let home_dir = unsafe { char_ptr_to_string(passwd.pw_dir) };
        // From "/usr/bin/shell" to just "shell"
        let shell = unsafe { char_ptr_to_string(passwd.pw_shell) }
            .rsplit(|a| a == '/')
            .next()
            .unwrap()
            .to_string();

        Some((username, home_dir, shell))
    } else {
        None
    }
}

pub fn get_cpu_model() -> Option<String> {
    let data = fs::read_to_string("/proc/cpuinfo").ok()?;
    for line in data.split('\n') {
        if line.len() < 11 {
            continue;
        }
        if let "model name" = &line[..10] {
            return Some(line[12..].splitn(2, '@').next().unwrap().trim().to_string());
        };
    }

    None
}

pub fn get_uptime(uptime_in_centiseconds: usize) -> String {
    let periods: SmallVec<[(u64, &str); 8]> = smallvec![
        (60 * 60 * 24 * 365, "year"),
        (60 * 60 * 24 * 30, "month"),
        (60 * 60 * 24, "day"),
        (60 * 60, "hour"),
        (60, "minute"),
        (1, "second"),
    ];

    // Ignore decimal places
    let mut uptime_in_seconds = uptime_in_centiseconds as u64;
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

    uptime
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

use phf::phf_map;

use crate::{arts::*, get_rand};

use std::fs;

// Unused for now
/// Should include (preferably) all package managers used by distros.
#[allow(dead_code)]
pub enum PackageManager {
    Pacman,
    EOPKG,
    DPKG,
    RPM,
}


// Return the enum variant
pub static DISTROS: phf::Map<&'static str, &'static str> = phf_map! {
    "arch"    => ARCH_LOGO   ,
    "debian"  => DEBIAN_LOGO ,
    "fedora"  => FEDORA_LOGO ,
    "manjaro" => MANJARO_LOGO,
};

pub fn choose_distro(random: bool) -> &'static str {
    if random {
        let keys: Vec<&str> = DISTROS.keys().map(|x: &&str| *x).collect();
        let idx = get_rand(keys.len() as i32) as usize;
        DISTROS
            .get(keys[idx])
            .map(Clone::clone)
            .unwrap_or("linux")
        
    } else {
        let id = get_id().unwrap();
        DISTROS
            .get(id.as_str())
            .map(Clone::clone)
            .unwrap_or("linux")
    }
}

pub fn get_id() -> Option<String> {
    let text = fs::read_to_string("/etc/os-release").ok()?;
    let id: usize = text.find("\nID=")?;
    let id: String = text[id + 4..].chars().take_while(|x| *x != '\n').collect();
    Some(id)
}

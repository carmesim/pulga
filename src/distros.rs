use phf::phf_map;

use crate::{arts::*, distros::Distro::*, get_rand};

use std::{fs, mem};

// Sync with enum below
const DISTRO_QUANTITY: i32 = 5;

#[allow(dead_code)]
#[repr(i32)]
#[derive(Debug, Clone)]
pub enum Distro {
    Arch,
    Manjaro,
    Debian,
    Fedora,
    Unknown,
}

// Return the enum variant
pub static DISTROS: phf::Map<&'static str, Distro> = phf_map! {
    "arch"    => Arch   ,
    "debian"  => Debian ,
    "fedora"  => Fedora ,
    "manjaro" => Manjaro,
};

pub fn choose_distro(random: bool) -> Distro {
    if random {
        unsafe { mem::transmute(get_rand(DISTRO_QUANTITY)) }
    } else {
        let id = get_id().unwrap();
        DISTROS
            .get(id.as_str())
            .map(Clone::clone)
            .unwrap_or(Distro::Unknown)
    }
}

pub fn choose_art(distro: Distro) -> &'static str {
    use Distro::*;
    match distro {
        Arch => ARCH_LOGO,
        Manjaro => MANJARO_LOGO,
        Debian => DEBIAN_LOGO,
        Fedora => FEDORA_LOGO,
        Unknown => "Default linux art here",
    }
}

pub fn get_id() -> Option<String> {
    let text = fs::read_to_string("/etc/os-release").ok()?;
    let id: usize = text.find("\nID=")?;
    let id: String = text[id + 4..].chars().take_while(|x| *x != '\n').collect();
    Some(id)
}

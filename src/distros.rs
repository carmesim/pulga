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
    "arch"        => ARCH_LOGO        ,
    "manjaro"     => MANJARO_LOGO     ,
    "debian"      => DEBIAN_LOGO      ,
    "fedora"      => FEDORA_LOGO      ,
    "aix"         => AIX_LOGO         ,
    "hash"        => HASH_LOGO        ,
    "alpine"      => ALPINE_LOGO      ,
    "alter"       => ALTER_LOGO       ,
    "amazon"      => AMAZON_LOGO      ,
    "anarchy"     => ANARCHY_LOGO     ,
    "android"     => ANDROID_LOGO     ,
    "antergos"    => ANTERGOS_LOGO    ,
    "antix"       => ANTIX_LOGO       ,
    "aoscosretro" => AOSCOSRETRO_LOGO ,
    "aoscos"      => AOSCOS_LOGO      ,
    "arcolinux"   => ARCOLINUX_LOGO   ,
    "archbox"     => ARCHBOX_LOGO     ,
    "archlabs"    => ARCHLABS_LOGO    ,
    "archstrike"  => ARCHSTRIKE_LOGO  ,
    "xferience"   => XFERIENCE_LOGO   ,
    "artix"       => ARTIX_LOGO       ,
    "arya"        => ARYA_LOGO        ,
    "bedrock"     => BEDROCK_LOGO     ,
    "bitrig"      => BITRIG_LOGO      ,
    "blackarch"   => BLACKARCH_LOGO   ,
    "blankon"     => BLANKON_LOGO     ,
    "bluelight"   => BLUELIGHT_LOGO   ,
    "bonsai"      => BONSAI_LOGO      ,
    "bsd"         => BSD_LOGO         ,
    "bunsenlabs"  => BUNSENLABS_LOGO  ,
    "calculate"   => CALCULATE_LOGO   ,
    "carbs"       => CARBS_LOGO       ,
    "centos"      => CENTOS_LOGO      ,
    "chakra"      => CHAKRA_LOGO      ,
    "chaletos"    => CHALETOS_LOGO    ,
    "chapeau"     => CHAPEAU_LOGO     ,
    "chrom"       => CHROM_LOGO       ,
    "clearos"     => CLEAROS_LOGO     ,
    "clearlinux"  => CLEARLINUX_LOGO  ,
    "clover"      => CLOVER_LOGO      ,
    "condres"     => CONDRES_LOGO     ,
    "containerlinuxbycoreos_or_containerlinux" => CONTAINERLINUXBYCOREOS_OR_CONTAINERLINUX_LOGO, // lmao
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

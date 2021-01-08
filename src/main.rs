#[allow(dead_code)]
mod _arts;
#[rustfmt::skip]
mod distros;
mod pulga;
mod screenres;
#[cfg(feature = "use_xlib")]
mod screenresx11;
mod sysinfo;
mod uname;
mod util;

use crate::{pulga::UserData, util::get_rand};

use smallvec::SmallVec;
use sugars::{boxed, hmap};
use termion::{color::*, cursor::*};

use std::{collections::HashMap, env, mem};

fn show(text: String, art: &str) {
    let lines: SmallVec<[&str; 128]> = text.lines().map(|x| x.trim()).collect();

    // Code to show colored logo
    #[rustfmt::skip]
    let color_map = {
        let mut m: HashMap<char, Box<dyn Color>> = hmap! {};
        m.insert('k', boxed!(Black  )); // k => Black
        m.insert('b', boxed!(Blue   )); // b => Blue
        m.insert('c', boxed!(Cyan   )); // c => Cyan
        m.insert('g', boxed!(Green  )); // g => Green
        m.insert('m', boxed!(Magenta)); // m => Magenta
        m.insert('r', boxed!(Red    )); // r => Red
        m.insert('w', boxed!(White  )); // w => White
        m.insert('y', boxed!(Yellow )); // y => Yellow
        m.insert('R', boxed!(Reset  )); // R => Reset all
        m
    };

    let logo = art.chars().collect::<SmallVec<[char; 8192]>>();

    let mut i = 0;
    while i < logo.len() - 2 {
        let slice = &logo[i..=i + 2];

        match slice {
            ['{', color_id, '}'] => {
                let color: &dyn Color = match color_map.get(color_id) {
                    None => panic!("Unexpected color_id '{}'", color_id),
                    Some(color) => color.as_ref(),
                };

                print!("{}", Fg(color));
                i += 3;
            },
            [first, ..] => {
                print!("{}", *first as char);
                i += 1;
            },
            _ => unreachable!(),
        }
    }

    // Show the remaining stuff
    for remaining in logo.iter().skip(i) {
        print!("{}", remaining);
    }
    println!();

    // Code to insert information at the side of the logo
    print!("{}", Up(14));

    for (_, line) in lines.iter().enumerate() {
        print!("{} {}{}{}", Right(32), line, Left(1000), Down(1));
    }
    print!("{}", Down(18));
}

fn main() {
    // dbg!(scwayland::get_screen_resolution());

    let UserData {
        username,
        hostname,
        cpu_info,
        uptime,
        hmd,
        shell,
        editor,
        distro,
        kernel_version,
        desk_env,
        monitor_res,
        used_memory,
        total_memory,
        cwd: _, // Unused
    } = pulga::get_user_data();

    #[rustfmt::skip]
    let text = format!(
        "{c}{}{R}@{c}{}{R}\n\
        \n\
         {c}{}{w}: {r}{}{R}\n\
         {c}{}{w}: {r}{}{R}\n\
         {c}{}{w}: {r}{}/{R}\n\
         {c}{}{w}: {r}{}{R}\n\
         {c}{}{w}: {r}{}{R}\n\
         {c}{}{w}: {r}{}{R}\n\
         {c}{}{w}: {r}{}{R}\n\
         {c}{}{w}: {r}{}{R}\n\
         {c}{}{w}: {r}{}{R}\n\
         {c}{}{w}: {r}{}{R} / {r}{}{R}",
        username, hostname,
        "cpu", cpu_info,
        "uptime", uptime,
        "home", hmd,
        "shell", shell,
        "editor", editor,
        "distro", distro,
        "kernel", kernel_version,
        "desktop env.", desk_env,
        "monitor", monitor_res,
        "memory usage", used_memory, total_memory,
        c = Fg(LightCyan),
        w = Fg(LightBlack),
        R = Fg(Reset),
        r = Fg(LightRed),
    );

    let mut is_random = false;

    // Small arg parsing out of nowhere
    for arg in env::args().skip(1) {
        if arg == "--random" || arg == "-r" {
            is_random = true;
        }
    }

    let distro = if is_random {
        // This is seriously hacky
        unsafe { mem::transmute(get_rand(distros::DISTROS) as i8) }
    } else {
        distros::Distro::Debian
    };
    let (_, art) = distros::choose_art(distro);

    show(text, art);
}

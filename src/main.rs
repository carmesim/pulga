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
use std::io::{self, BufWriter, Write};

use smallvec::SmallVec;
use sugars::boxed;
use termion::{color::*, cursor::*};
use phf;

use std::{collections::HashMap, env, mem};

fn display_information_and_logo(text: String, art: &str, horizontal_offset: u16) -> io::Result<()> {
    let lines: SmallVec<[&str; 128]> = text.lines().map(|x| x.trim()).collect();

    // Logo colors table
    #[rustfmt::skip]
    let color_map = {
        let mut m = HashMap::<char, Box<dyn Color>>::new();
        m.insert('k', boxed!(Black  )); // k => Black
        m.insert('r', boxed!(Red    )); // r => Red
        m.insert('g', boxed!(Green  )); // g => Green
        m.insert('y', boxed!(Yellow )); // y => Yellow
        m.insert('b', boxed!(Blue   )); // b => Blue
        m.insert('m', boxed!(Magenta)); // m => Magenta
        m.insert('c', boxed!(Cyan   )); // c => Cyan
        m.insert('w', boxed!(White  )); // w => White
        m.insert('R', boxed!(Reset  )); // R => Reset all
        m
    };

    let logo = art.chars().collect::<SmallVec<[char; 8192]>>();
    let mut output = BufWriter::new(io::stdout());

    let mut i = 0;

    while i < logo.len() - 2 {
        match &logo[i..=i + 2] {
            ['{', color_id, '}'] => {
                let color: &dyn Color = match color_map.get(&color_id) {
                    Some(color) => color.as_ref(),
                    None => panic!("Unexpected color_id '{}'", color_id),
                };

                i += 2;
                write!(output, "{}", Fg(color))?;
            },
            ['\n', ..] => {
                write!(output, "\n   ")?;
            },
            other => {
                write!(
                    output,
                    "{}",
                    other.get(0).expect("Slice shouldn't be empty")
                )?;
            },
        }
        i += 1;
    }

    // Show the remaining stuff
    for remaining in logo.iter().skip(i) {
        write!(output, "{}", remaining)?;
    }
    write!(output, "\n")?;

    let logo_lines = logo.iter().filter(|x| **x == '\n').count() as u16;
    let info_lines = lines.len() as u16;
    let up_how_many_times = info_lines + (logo_lines - info_lines) / 2 + 1;

    // Code to insert information at the side of the logo
    write!(output, "{}", Up(up_how_many_times))?;

    for line in lines.iter() {
        write!(
            output,
            "{} {}{}{}",
            Right(horizontal_offset),
            line,
            Left(u16::MAX), // Go to the beginning, for sure
            Down(1)
        )?;
    }
    write!(output, "{}", Down(up_how_many_times - lines.len() as u16))?;
    Ok(())
}

fn main() -> io::Result<()> {
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

    let (offset, art) = distros::choose_art(distro);
    display_information_and_logo(text, art, offset)
}

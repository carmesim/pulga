mod error;
mod logos;
mod newfetch;

use crate::newfetch::UserData;

use indoc::indoc;
// use sugars::{boxed, hmap};
use termion::{color::*, cursor::*};

use std::collections::HashMap;

fn show(text: String, logo: &str) {
    let mut lines: Vec<String> = text.lines().map(|x| x.trim().to_string()).collect();

    // Logo tem 14 de largura hmm
    println!("{}", logo);
    print!("{}", Up(14));

    for (_, line) in lines.iter().enumerate() {
        print!("{} {}{}{}", Right(31), line, Left(1000), Down(1));
    }

    print!("{}", Down(17));

    // Skip colors for now
    //    let mut color_map: HashMap<char, Box<dyn Color>> = hmap! {};
    //    color_map.insert('k', boxed!(Black));
    //    color_map.insert('b', boxed!(Blue));
    //    color_map.insert('c', boxed!(Cyan));
    //    color_map.insert('g', boxed!(Green));
    //    color_map.insert('m', boxed!(Magenta));
    //    color_map.insert('r', boxed!(Red));
    //    color_map.insert('R', boxed!(Reset));
    //    color_map.insert('w', boxed!(White));
    //    color_map.insert('y', boxed!(Yellow));

    // k // Black
    // b // Blue
    // c // Cyan
    // g // Green
    // m // Magenta
    // r // Red
    // R // Reset   Reset colors to defaults.
    // w // White
    // y // Yellow
    // Ignore for now?
    // LightBlack  High-intensity light black.
    // LightBlue   High-intensity light blue.
    // LightCyan   High-intensity light cyan.
    // LightGreen  High-intensity light green.
    // LightMagenta    High-intensity light magenta.
    // LightRed    High-intensity light red.
    // LightWhite  High-intensity light white.
    // LightYellow High-intensity light yellow.
}

fn main() {
    let data: UserData = newfetch::get_user_data();

    #[rustfmt::skip]
    let text = format!(indoc! {
        "{c}{}{w}: {r}{}{R}
         {c}{}{w}: {r}{}{R}
         {c}{}{w}: {r}{}{R}
         {c}{}{w}: {r}{}{R}
         {c}{}{w}: {r}{}{R}
         {c}{}{w}: {r}{}{R}
         {c}{}{w}: {r}{}{R}
         {c}{}{w}: {r}{}{R}
         {c}{}{w}: {r}{}{R} / {r}{}{R}"
        },
        "username", data.username,
        "hostname", data.hostname,
        "device name", data.devicename,
        "uptime", newfetch::get_uptime().unwrap(),
        "home dir.", data.hmd,
        "platform", data.platform,
        "distro", data.distro,
        "desktop env.", data.desk_env,
        "memory usage", data.used_memory, data.total_memory,
        c = Fg(LightCyan),
        w = Fg(LightBlack),
        R = Fg(Reset),
        r = Fg(LightRed)
    );

    // println!("{}", text.len());

    let logo = logos::choose_logo(logos::Logo::Manjaro);
    // println!("{}", logo);

    show(text, logo);
}

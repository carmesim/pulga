use std::env;

fn running_on_wayland() -> bool {
    let session_type = match env::var_os("XDG_SESSION_TYPE") {
        Some(session) => session.to_string_lossy().to_string(),
        None                  => return false
    };

    session_type == "wayland"
}


fn main() {
    // Checking if running on Wayland
    if running_on_wayland() {
        println!("cargo:warning=Wayland protocol detected.");
        println!("cargo:rustc-cfg=feature=\"on_wayland\"");
        println!("cargo:rustc-cfg=show_screen_res");
    }

    #[cfg(any(feature="on_x11", feature="on_wayland"))]
    println!("cargo:rustc-cfg=show_screen_res");

    #[cfg(feature = "on_x11")]
    {
        println!("cargo:warning=Compiling with X11 dependencies.");
        println!("cargo:rustc-flags=-lX11 -lXrandr");
    }
}
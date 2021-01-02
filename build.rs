// fn running_on_wayland() -> bool {
//     let session_type = match env::var_os("XDG_SESSION_TYPE") {
//         Some(session) => session.to_string_lossy().to_string(),
//         None                  => return false
//     };

//     session_type == "wayland"
// }


fn main() {
    #[cfg(feature = "use_xlib")]
    {
        println!("cargo:warning=Compiling with X11 dependencies.");
        println!("cargo:rustc-flags=-lX11 -lXrandr");
    }
}
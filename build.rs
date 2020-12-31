fn main() {
    #[cfg(any(feature="on_x11", feature="on_wayland"))]
    println!("cargo:rustc-cfg=show_screen_res");

    #[cfg(feature = "on_x11")]
    {
        println!("cargo:warning=Compiling with X11 dependencies.");
        println!("cargo:rustc-flags=-lX11 -lXrandr");
    }
}
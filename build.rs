fn main() {
    #[cfg(feature = "on_x11")]
    {
        println!("cargo:warning=Compiling with X11 dependencies.");
        println!("cargo:rustc-flags=-lX11 -lXrandr");
        println!("cargo:rustc-cfg=usingX11");
    }
}
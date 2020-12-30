use x11::xlib::{XOpenDisplay, Display, XDefaultRootWindow};
use x11::xrandr::{
    XRRGetScreenResources, XRRCrtcInfo, XRRGetCrtcInfo, XRRFreeCrtcInfo, XRRFreeScreenResources, XRRScreenResources
};
use std::{ptr, vec::Vec, mem};

    // let screens = XRRGetScreenResources(display as mut *, );
    // let mut screens: XRRScreenResources = unsafe { mem::zeroed() };

pub unsafe fn get_screen_resolution() -> Vec<String> {
    let mut display: *mut Display = XOpenDisplay(ptr::null());
    
    let mut screens: *mut XRRScreenResources =  XRRGetScreenResources(display, XDefaultRootWindow(display));

    let screens_no = (*screens).ncrtc as isize;

    for i in 0..screens_no {
        let info = XRRGetCrtcInfo(display, screens, *(*screens).crtcs.offset(i));
        println!("{}x{}", (*info).width, (*info).height);
        XRRFreeCrtcInfo(info);
    }

    XRRFreeScreenResources(screens);

    vec![]
}
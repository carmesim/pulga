use x11::xlib::{XOpenDisplay, Display, XDefaultRootWindow};
use x11::xrandr::{
    XRRGetScreenResources, XRRCrtcInfo, XRRGetCrtcInfo, XRRFreeCrtcInfo, XRRFreeScreenResources, XRRScreenResources
};
use std::{ptr, vec::Vec};

pub unsafe fn get_screen_resolution() -> Vec<String> {

    let mut resolutions = vec![];

    let display: *mut Display = XOpenDisplay(ptr::null());
    
    let screens: *mut XRRScreenResources =  XRRGetScreenResources(display, XDefaultRootWindow(display));

    let screens_no = (*screens).ncrtc as isize;

    for i in 0..screens_no {
        let info: *mut XRRCrtcInfo = XRRGetCrtcInfo(display, screens, *(*screens).crtcs.offset(i));
        match ((*info).width, (*info).height) {
            (wdt, hgt) if wdt != 0 && hgt != 0 => {
                resolutions.push(format!("{}p", hgt));
            },
            (_, _) => {},
        };

        XRRFreeCrtcInfo(info);
    }

    XRRFreeScreenResources(screens);

    resolutions
}
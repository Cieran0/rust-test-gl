use std::os::raw::{c_char, c_int, c_void};

#[repr(C)]
pub struct VisualInfo {
    pub visual: *mut c_void,
    pub visualid: u64,
    pub screen: c_int,
    pub depth: c_int,
    pub class: c_int,
    pub red_mask: u64,
    pub green_mask: u64,
    pub blue_mask: u64,
    pub colormap_size: c_int,
    pub bits_per_rgb: c_int,
}

#[link(name = "X11")]
unsafe extern "C" {
    pub unsafe fn XOpenDisplay(name: *const c_char) -> *mut c_void;
    pub unsafe fn XCloseDisplay(dpy: *mut c_void);
    pub unsafe fn XDefaultScreen(dpy: *mut c_void) -> c_int;
}
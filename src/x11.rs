use std::os::raw::{c_char, c_int, c_long, c_uint, c_ulong, c_void};

pub type Window = c_ulong;
pub type Colormap = c_ulong;
pub type Atom = c_ulong;
pub type Time = c_ulong;
pub type VisualID = c_ulong;


#[repr(C)]
#[derive(Debug)]
pub struct XKeyEvent {
    pub type_: c_int,        
    pub serial: c_ulong,     
    pub send_event: c_int,   
    pub display: *mut c_void,
    pub window: Window,      
    pub root: Window,        
    pub subwindow: Window,   
    pub time: Time,          
    pub x: c_int,            
    pub y: c_int,            
    pub x_root: c_int,       
    pub y_root: c_int,       
    pub state: c_uint,       
    pub keycode: c_uint,     
    pub same_screen: c_int,  
    // Pad to 96
    pub _padding: [c_char; 4],
}

#[repr(C)]
#[derive(Debug)]
pub struct XButtonEvent {
    pub type_: c_int,
    pub serial: c_ulong,
    pub send_event: c_int,
    pub display: *mut c_void,
    pub window: Window,
    pub root: Window,
    pub subwindow: Window,
    pub time: Time,
    pub x: c_int,
    pub y: c_int,
    pub x_root: c_int,
    pub y_root: c_int,
    pub state: c_uint,
    pub button: c_uint,
    pub same_screen: c_int,
    pub _padding: [c_char; 4],
}

#[repr(C)]
#[derive(Debug)]
pub struct XMotionEvent {
    pub type_: c_int,
    pub serial: c_ulong,
    pub send_event: c_int,
    pub display: *mut c_void,
    pub window: Window,
    pub root: Window,
    pub subwindow: Window,
    pub time: Time,
    pub x: c_int,
    pub y: c_int,
    pub x_root: c_int,
    pub y_root: c_int,
    pub state: c_uint,
    pub is_hint: c_char,
    pub same_screen: c_int,
    // Pad to 96
    pub _padding: [c_char; 3],
}

#[repr(C)]
#[derive(Debug)]
pub struct XMapEvent {
    pub type_: c_int,
    pub serial: c_ulong,
    pub send_event: c_int,
    pub display: *mut c_void,
    pub window: Window,
    pub event: Window,
    pub override_redirect: c_int,
    // Pad to 96
    pub _padding: [c_char; 12],
}

#[repr(C)]
#[derive(Debug)]
pub struct XEvent {
    pub data: [u8; 96], // sizeof(XEvent) = 96
}

pub enum Event<'a> {
    KeyPress(&'a XKeyEvent),
    KeyRelease(&'a XKeyEvent),
    ButtonPress(&'a XButtonEvent),
    ButtonRelease(&'a XButtonEvent),
    Motion(&'a XMotionEvent),
    Map(&'a XMapEvent),
    DestroyNotify,
    Expose,
    Unknown,
}

impl XEvent {
    pub fn into_event(&self) -> Event {
        match self.event_type() {
            KEY_PRESS => Event::KeyPress(unsafe { &*(self as *const XEvent as *const XKeyEvent) }),
            KEY_RELEASE => Event::KeyRelease(unsafe { &*(self as *const XEvent as *const XKeyEvent) }),
            BUTTON_PRESS => Event::ButtonPress(unsafe { &*(self as *const XEvent as *const XButtonEvent) }),
            BUTTON_RELEASE => Event::ButtonRelease(unsafe { &*(self as *const XEvent as *const XButtonEvent) }),
            MOTION_NOTIFY => Event::Motion(unsafe { &*(self as *const XEvent as *const XMotionEvent) }),
            MAP_NOTIFY => Event::Map(unsafe { &*(self as *const XEvent as *const XMapEvent) }),
            DESTROY_NOTIFY => Event::DestroyNotify,
            EXPOSE => Event::Expose,
            _ => Event::Unknown,
        }
    }

    pub fn event_type(&self) -> c_int {
        unsafe { *(self.data.as_ptr() as *const c_int) }
    }
}


pub const KEY_PRESS: c_int = 2;
pub const KEY_RELEASE: c_int = 3;
pub const BUTTON_PRESS: c_int = 4;
pub const BUTTON_RELEASE: c_int = 5;
pub const MOTION_NOTIFY: c_int = 6;
pub const DESTROY_NOTIFY: c_int = 17;
pub const MAP_NOTIFY: c_int = 19;
pub const EXPOSE: c_int = 12;

pub const EXPOSURE_MASK: c_long = 1 << 15;
pub const STRUCTURE_NOTIFY_MASK: c_long = 1 << 17;
pub const KEY_PRESS_MASK: c_long = 1 << 0;
pub const KEY_RELEASE_MASK: c_long = 1 << 1;
pub const BUTTON_PRESS_MASK: c_long = 1 << 2;
pub const BUTTON_RELEASE_MASK: c_long = 1 << 3;
pub const POINTER_MOTION_MASK: c_long = 1 << 6;

pub const INPUT_OUTPUT: c_uint = 1;
pub const CWCOLORMAP: c_ulong = 1 << 13;
pub const CWEVENT_MASK: c_ulong = 1 << 11;

pub const PROP_MODE_REPLACE: i32 = 0;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct XVisualInfo {
    pub visual: *mut c_void,
    pub visualid: VisualID,
    pub screen: c_int,
    pub depth: c_int,
    pub class: c_int,
    pub red_mask: c_ulong,
    pub green_mask: c_ulong,
    pub blue_mask: c_ulong,
    pub colormap_size: c_int,
    pub bits_per_rgb: c_int,
}

#[repr(C)]
pub struct XSetWindowAttributes {
    pub background_pixmap: c_ulong,
    pub background_pixel: c_ulong,
    pub border_pixmap: c_ulong,
    pub border_pixel: c_ulong,
    pub bit_gravity: c_int,
    pub win_gravity: c_int,
    pub backing_store: c_int,
    pub backing_planes: c_ulong,
    pub backing_pixel: c_ulong,
    pub save_under: c_int,
    pub event_mask: c_long,
    pub do_not_propagate_mask: c_long,
    pub override_redirect: c_int,
    pub colormap: Colormap,
    pub cursor: c_ulong,
}

#[link(name = "X11")]
unsafe extern "C" {
    pub unsafe fn XOpenDisplay(display_name: *const c_char) -> *mut c_void;
    pub unsafe fn XCloseDisplay(display: *mut c_void) -> c_int;
    pub unsafe fn XDefaultScreen(display: *mut c_void) -> c_int;
    pub unsafe fn XDefaultRootWindow(display: *mut c_void) -> Window;
    pub unsafe fn XWhitePixel(display: *mut c_void, screen: c_int) -> c_ulong;

    pub unsafe fn XCreateWindow(
        display: *mut c_void,
        parent: Window,
        x: c_int,
        y: c_int,
        width: c_uint,
        height: c_uint,
        border_width: c_uint,
        depth: c_int,
        class: c_uint,
        visual: *mut c_void,
        valuemask: c_ulong,
        attributes: *mut XSetWindowAttributes,
    ) -> Window;

    pub unsafe fn XMapWindow(display: *mut c_void, w: Window) -> c_int;
    pub unsafe fn XStoreName(display: *mut c_void, w: Window, window_name: *const c_char) -> c_int;
    pub unsafe fn XInternAtom(display: *mut c_void, atom_name: *const c_char, only_if_exists: c_int) -> Atom;
    pub unsafe fn XChangeProperty(
        display: *mut c_void,
        w: Window,
        property: Atom,
        r#type: Atom,
        format: c_int,
        mode: c_int,
        data: *const u8,
        nelements: c_int,
    ) -> c_int;

    pub unsafe fn XPending(display: *mut c_void) -> c_int;
    pub unsafe fn XNextEvent(display: *mut c_void, event_return: *mut XEvent);

    pub unsafe fn XCreateColormap(
        display: *mut c_void,
        w: Window,
        visual: *mut c_void,
        alloc: c_int,
    ) -> Colormap;

    pub unsafe fn XGetVisualInfo(
        display: *mut c_void,
        vinfo_mask: c_long,
        vinfo_template: *mut XVisualInfo,
        nitems_return: *mut c_int,
    ) -> *mut XVisualInfo;

    pub unsafe fn XFree(ptr: *mut c_void);
}
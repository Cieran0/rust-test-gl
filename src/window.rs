use std::ffi::CString;
use std::os::raw::{c_int, c_void};

use crate::{x11, glx};

pub fn create(dpy: *mut c_void, width: u16, height: u16) -> Result<x11::Window, Box<dyn std::error::Error>> {
    unsafe {
        let screen = x11::XDefaultScreen(dpy);
        let root = x11::XDefaultRootWindow(dpy);

        let fb_attrs = [
            glx::X_RENDERABLE, 1,
            glx::DRAWABLE_TYPE, glx::WINDOW_BIT,
            glx::RENDER_TYPE, glx::RGBA_BIT,
            glx::RED_SIZE, 8,
            glx::GREEN_SIZE, 8,
            glx::BLUE_SIZE, 8,
            glx::ALPHA_SIZE, 8,
            glx::DEPTH_SIZE, 24,
            glx::DOUBLEBUFFER, 1,
            0,
        ];

        let mut nelements: c_int = 0;
        let fbconfigs = glx::glXChooseFBConfig(dpy, screen, fb_attrs.as_ptr(), &mut nelements);
        assert!(!fbconfigs.is_null() && nelements > 0, "No FBConfig found");
        let fbconfig = *fbconfigs.add(0);

        let visual_info_ptr = glx::glXGetVisualFromFBConfig(dpy, fbconfig);
        assert!(!visual_info_ptr.is_null(), "No visual from FBConfig");
        let visual_info: x11::XVisualInfo = *visual_info_ptr;

        let cmap = x11::XCreateColormap(dpy, root, visual_info.visual, 0);

        let mut attr = x11::XSetWindowAttributes {
            background_pixel: x11::XWhitePixel(dpy, screen),
            colormap: cmap,
            event_mask: x11::KEY_PRESS_MASK
                | x11::KEY_RELEASE_MASK
                | x11::BUTTON_PRESS_MASK
                | x11::BUTTON_RELEASE_MASK
                | x11::POINTER_MOTION_MASK
                | x11::EXPOSURE_MASK
                | x11::STRUCTURE_NOTIFY_MASK,
            ..std::mem::zeroed()
        };

        let valuemask = x11::CWCOLORMAP | x11::CWEVENT_MASK;

        let window = x11::XCreateWindow(
            dpy,
            root,
            100,
            100,
            width as u32,
            height as u32,
            0,
            visual_info.depth,
            x11::INPUT_OUTPUT,
            visual_info.visual,
            valuemask,
            &mut attr,
        );

        let title = CString::new("GL Test").unwrap();
        x11::XStoreName(dpy, window, title.as_ptr());

        let net_wm_name = x11::XInternAtom(dpy, b"_NET_WM_NAME\0".as_ptr() as _, 0);
        let utf8_string = x11::XInternAtom(dpy, b"UTF8_STRING\0".as_ptr() as _, 0);
        x11::XChangeProperty(
            dpy,
            window,
            net_wm_name,
            utf8_string,
            8,
            x11::PROP_MODE_REPLACE,
            b"GL Test".as_ptr(),
            7,
        );

        let net_wm_window_type = x11::XInternAtom(dpy, b"_NET_WM_WINDOW_TYPE\0".as_ptr() as _, 0);
        let net_wm_window_type_dialog =
            x11::XInternAtom(dpy, b"_NET_WM_WINDOW_TYPE_DIALOG\0".as_ptr() as _, 0);
        x11::XChangeProperty(
            dpy,
            window,
            net_wm_window_type,
            4,
            32,
            0,
            &net_wm_window_type_dialog as *const _ as *const u8,
            1,
        );

        x11::XMapWindow(dpy, window);

        loop {
            let mut ev: x11::XEvent = std::mem::zeroed();
            x11::XNextEvent(dpy, &mut ev);
            if ev.event_type() == x11::MAP_NOTIFY {
                let map_ev: &x11::XMapEvent = std::mem::transmute(&ev.data);
                if map_ev.window == window {
                    break;
                }
            }
        }

        x11::XFree(visual_info_ptr as *mut c_void);
        x11::XFree(fbconfigs as *mut c_void);

        println!("X11 window created via raw Xlib (id: {window})");
        Ok(window)
    }
}
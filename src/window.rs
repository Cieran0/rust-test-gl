use std::os::raw::{c_int, c_void};

use x11rb::{connection::Connection, protocol::{xproto::{ConnectionExt, *}, Event}, rust_connection::RustConnection, wrapper::ConnectionExt as _};

use crate::{glx, x11};


pub fn create(dpy: *mut c_void) -> Result<(RustConnection, u32), Box<dyn std::error::Error>> {
    let (conn, screen_num) = RustConnection::connect(None)?;
    let screen = &conn.setup().roots[screen_num];

    let screen_id = unsafe { x11::XDefaultScreen(dpy) };
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
    let fbconfigs = unsafe { glx::glXChooseFBConfig(dpy, screen_id, fb_attrs.as_ptr(), &mut nelements) };
    assert!(!fbconfigs.is_null() && nelements > 0, "No FBConfig found");
    let fbconfig = unsafe { *fbconfigs.add(0) };

    let visual_info = unsafe { glx::glXGetVisualFromFBConfig(dpy, fbconfig) };
    assert!(!visual_info.is_null(), "No visual info found");

    let window = conn.generate_id()?;
    let width = 1024u16;
    let height = 768u16;

    let colormap = conn.generate_id()?;
    conn.create_colormap(ColormapAlloc::NONE, colormap, screen.root, unsafe { (*visual_info).visualid.try_into().unwrap() })?;

    let depth = unsafe { (*visual_info).depth as u8 };
    let visualid = unsafe { (*visual_info).visualid };

    conn.create_window(
        depth,
        window,
        screen.root,
        1280, 720,
        width,
        height,
        0,
        WindowClass::INPUT_OUTPUT,
        visualid.try_into().unwrap(),
        &CreateWindowAux::new()
            .colormap(colormap)
            .background_pixel(screen.white_pixel)
            .event_mask(EventMask::KEY_PRESS | EventMask::STRUCTURE_NOTIFY),
    )?;

    let net_wm_name = conn.intern_atom(false, b"_NET_WM_NAME")?.reply()?.atom;
    let utf8_string = conn.intern_atom(false, b"UTF8_STRING")?.reply()?.atom;
    conn.change_property8(
        x11rb::protocol::xproto::PropMode::REPLACE,
        window,
        net_wm_name,
        utf8_string,
        b"GL Test",
    )?;

    let net_wm_window_type = conn.intern_atom(false, b"_NET_WM_WINDOW_TYPE")?.reply()?.atom;
    let net_wm_window_type_dialog = conn.intern_atom(false, b"_NET_WM_WINDOW_TYPE_DIALOG")?.reply()?.atom;
    conn.change_property32(
        x11rb::protocol::xproto::PropMode::REPLACE,
        window,
        net_wm_window_type,
        AtomEnum::ATOM,
        &[net_wm_window_type_dialog],
    )?;

    conn.map_window(window)?;
    conn.flush()?;

    loop {
        if let Some(event) = conn.poll_for_event()? {
            if let Event::MapNotify(ev) = event {
                if ev.window == window {
                    break;
                }
            }
        }
    }

    println!("X11 window created via x11rb (id: {window})");
    Ok((conn, window))
}
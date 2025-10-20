use std::{ffi::{CStr, CString}, os::raw::{c_char, c_int, c_uchar, c_void}, ptr};

use crate::x11;

pub const X_RENDERABLE: c_int = 0x8012;
pub const DRAWABLE_TYPE: c_int = 0x8010;
pub const RENDER_TYPE: c_int = 0x8011;
pub const RGBA_BIT: c_int = 0x00000001;
pub const WINDOW_BIT: c_int = 0x00000001;
pub const RED_SIZE: c_int = 8;
pub const GREEN_SIZE: c_int = 9;
pub const BLUE_SIZE: c_int = 10;
pub const ALPHA_SIZE: c_int = 11;
pub const DEPTH_SIZE: c_int = 12;
pub const DOUBLEBUFFER: c_int = 5;
pub const CONTEXT_MAJOR_VERSION_ARB: c_int = 0x2091;
pub const CONTEXT_MINOR_VERSION_ARB: c_int = 0x2092;
pub const CONTEXT_PROFILE_MASK_ARB: c_int = 0x9126;
pub const CONTEXT_CORE_PROFILE_BIT_ARB: c_int = 0x00000001;

#[link(name = "GL")]
unsafe extern "C" {
    pub unsafe fn glXChooseFBConfig(
        dpy: *mut c_void,
        screen: c_int,
        attrib_list: *const c_int,
        nelements: *mut c_int,
    ) -> *mut *mut c_void;
    pub unsafe fn glXGetVisualFromFBConfig(dpy: *mut c_void, config: *mut c_void) -> *mut x11::VisualInfo;
    pub unsafe fn glXGetProcAddress(procName: *const c_uchar) -> *mut c_void;
    pub unsafe fn glXMakeCurrent(dpy: *mut c_void, drawable: u32, ctx: *mut c_void) -> c_int;
    pub unsafe fn glXSwapBuffers(dpy: *mut c_void, drawable: u32);
}

pub fn create_gl_context(dpy: *mut c_void, window: u32) -> *mut c_void {
    unsafe {
        let screen_id = x11::XDefaultScreen(dpy);

        let fb_attrs = [
            X_RENDERABLE, 1,
            DRAWABLE_TYPE, WINDOW_BIT,
            RENDER_TYPE, RGBA_BIT,
            RED_SIZE, 8,
            GREEN_SIZE, 8,
            BLUE_SIZE, 8,
            ALPHA_SIZE, 8,
            DEPTH_SIZE, 24,
            DOUBLEBUFFER, 1,
            0,
        ];

        let mut nelements: c_int = 0;
        let fbconfigs = glXChooseFBConfig(dpy, screen_id, fb_attrs.as_ptr(), &mut nelements);
        assert!(!fbconfigs.is_null() && nelements > 0, "No FBConfig found");

        let fbconfig = *fbconfigs.add(0);

        let proc_name = b"glXCreateContextAttribsARB\0";
        let ptr = glXGetProcAddress(proc_name.as_ptr());
        let create_context_arb: extern "C" fn(
            *mut c_void, *mut c_void, *mut c_void, c_int, *const c_int
        ) -> *mut c_void = std::mem::transmute(ptr);

        let ctx_attribs = [
            CONTEXT_MAJOR_VERSION_ARB, 4,
            CONTEXT_MINOR_VERSION_ARB, 2,
            CONTEXT_PROFILE_MASK_ARB, CONTEXT_CORE_PROFILE_BIT_ARB,
            0,
        ];

        let ctx = create_context_arb(dpy, fbconfig, ptr::null_mut(), 1, ctx_attribs.as_ptr());
        assert!(!ctx.is_null(), "Failed to create OpenGL 4.2 core context");

        let make_current_ret = glXMakeCurrent(dpy, window, ctx);
        assert!(make_current_ret != 0, "Failed to make GL context current");

        println!("OpenGL 4.2 context current");
        ctx
    }
}

pub fn init_gl_functions() {
    unsafe {
        gl::load_with(|name| {
            let c_name = CString::new(name).unwrap();
            glXGetProcAddress(c_name.as_ptr() as *const c_uchar) as *const _
        });

        let ver_ptr = gl::GetString(gl::VERSION);
        if !ver_ptr.is_null() {
            println!("OpenGL version: {:?}", CStr::from_ptr(ver_ptr as *const c_char));
        }

        let glsl_ptr = gl::GetString(gl::SHADING_LANGUAGE_VERSION);
        if !glsl_ptr.is_null() {
            println!("GLSL version: {:?}", CStr::from_ptr(glsl_ptr as *const c_char));
        }
    }
}
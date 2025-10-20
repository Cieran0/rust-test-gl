// main.rs
extern crate gl;
use std::ffi::{CStr, CString};
use std::fs;
use std::os::raw::{c_char, c_int, c_uchar, c_uint, c_void};
use std::ptr;

#[link(name = "GL")]
#[link(name = "X11")]
unsafe extern "C" {
    fn XOpenDisplay(name: *const c_char) -> *mut c_void;
    fn XCloseDisplay(dpy: *mut c_void);
    fn XDefaultScreen(dpy: *mut c_void) -> c_int;
    fn XRootWindow(dpy: *mut c_void, screen: c_int) -> c_ulong;
    fn XCreateColormap(dpy: *mut c_void, w: c_ulong, visual: *mut c_void, alloc: c_int) -> c_ulong;
    fn XCreateWindow(
        dpy: *mut c_void,
        parent: c_ulong,
        x: c_int,
        y: c_int,
        width: c_int,
        height: c_int,
        border_width: c_int,
        depth: c_int,
        class: c_int,
        visual: *mut c_void,
        valuemask: c_ulong,
        attributes: *mut XSetWindowAttributes,
    ) -> c_ulong;
    fn XMapWindow(dpy: *mut c_void, w: c_ulong);
    fn XFlush(dpy: *mut c_void);
    fn XPending(dpy: *mut c_void) -> c_int;
    fn XNextEvent(dpy: *mut c_void, event_return: *mut XEvent);
    fn XLookupKeysym(event: *mut XKeyEvent, index: c_int) -> c_ulong;
    fn XFree(ptr: *mut c_void);

    fn glXChooseFBConfig(
        dpy: *mut c_void,
        screen: c_int,
        attrib_list: *const c_int,
        nelements: *mut c_int,
    ) -> *mut *mut c_void;
    fn glXGetVisualFromFBConfig(dpy: *mut c_void, config: *mut c_void) -> *mut XVisualInfo;

    // glXGetProcAddress takes const GLubyte* -> use c_uchar for bytes
    fn glXGetProcAddress(procName: *const c_uchar) -> *mut c_void;

    // Correct ABI: Bool and return Status are c_int on X/GLX
    fn glXCreateContextAttribsARB(
        dpy: *mut c_void,
        config: *mut c_void,
        share_context: *mut c_void,
        direct: c_int,
        attrib_list: *const c_int,
    ) -> *mut c_void;

    fn glXMakeCurrent(dpy: *mut c_void, drawable: c_ulong, ctx: *mut c_void) -> c_int;
    fn glXSwapBuffers(dpy: *mut c_void, drawable: c_ulong);
}

#[repr(C)]
struct XVisualInfo {
    visual: *mut c_void,
    visualid: c_ulong,
    screen: c_int,
    depth: c_int,
    class: c_int,
    red_mask: c_ulong,
    green_mask: c_ulong,
    blue_mask: c_ulong,
    colormap_size: c_int,
    bits_per_rgb: c_int,
}

#[repr(C)]
struct XSetWindowAttributes {
    background_pixmap: c_ulong,
    background_pixel: c_ulong,
    border_pixmap: c_ulong,
    border_pixel: c_ulong,
    bit_gravity: c_int,
    win_gravity: c_int,
    backing_store: c_int,
    backing_planes: c_ulong,
    backing_pixel: c_ulong,
    save_under: c_int,
    event_mask: c_long,
    do_not_propagate_mask: c_long,
    override_redirect: c_int,
    colormap: c_ulong,
    cursor: c_ulong,
}

#[repr(C)]
union XEvent {
    type_: c_int,
    xkey: XKeyEvent,
    _pad: [c_char; 24],
}

#[repr(C)]
#[derive(Copy, Clone)]
struct XKeyEvent {
    type_: c_int,
    serial: c_ulong,
    send_event: c_int,
    display: *mut c_void,
    window: c_ulong,
    root: c_ulong,
    subwindow: c_ulong,
    time: c_ulong,
    x: c_int,
    y: c_int,
    x_root: c_int,
    y_root: c_int,
    state: c_uint,
    keycode: c_uint,
    same_screen: c_int,
}

type c_long = i64;
type c_ulong = u64;

// X11 constants
const AllocNone: c_int = 0;
const InputOutput: c_int = 1;
const CWColormap: c_ulong = 1 << 13;
const CWEventMask: c_ulong = 1 << 11;
const KeyPressMask: c_long = 1 << 0;
const StructureNotifyMask: c_long = 1 << 17;
const KeyPress: c_int = 2;
const XK_Escape: c_ulong = 0xFF1B;

// GLX constants
const GLX_X_RENDERABLE: c_int = 0x8012;
const GLX_DRAWABLE_TYPE: c_int = 0x8010;
const GLX_RENDER_TYPE: c_int = 0x8011;
const GLX_RGBA_BIT: c_int = 0x00000001;
const GLX_WINDOW_BIT: c_int = 0x00000001;
const GLX_RED_SIZE: c_int = 8;
const GLX_GREEN_SIZE: c_int = 9;
const GLX_BLUE_SIZE: c_int = 10;
const GLX_ALPHA_SIZE: c_int = 11;
const GLX_DEPTH_SIZE: c_int = 12;
const GLX_DOUBLEBUFFER: c_int = 5;

// Context creation (ARB_create_context)
const GLX_CONTEXT_MAJOR_VERSION_ARB: c_int = 0x2091;
const GLX_CONTEXT_MINOR_VERSION_ARB: c_int = 0x2092;
const GLX_CONTEXT_PROFILE_MASK_ARB: c_int = 0x9126;
const GLX_CONTEXT_CORE_PROFILE_BIT_ARB: c_int = 0x00000001;

// Safe shader loader
fn load_shader(path: &str, shader_type: u32) -> u32 {
    let source = fs::read_to_string(path).expect(&format!("Failed to read '{}'", path));
    let source_c = CString::new(source.clone()).expect("CString failed");

    unsafe {
        let shader = gl::CreateShader(shader_type);
        gl::ShaderSource(shader, 1, &source_c.as_ptr(), ptr::null());
        gl::CompileShader(shader);

        let mut success = 0;
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
        if success != gl::TRUE as i32 {
            let mut len = 0;
            gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
            let mut buf = vec![0u8; (len as usize).max(1)];
            gl::GetShaderInfoLog(
                shader,
                buf.len() as i32,
                ptr::null_mut(),
                buf.as_mut_ptr() as *mut i8,
            );
            panic!(
                "Shader compile failed:\n{}\nSource:\n{}",
                String::from_utf8_lossy(&buf),
                source
            );
        }
        shader
    }
}

fn main() {
    unsafe {
        println!("🔹 Opening X display...");
        let dpy = XOpenDisplay(ptr::null());
        assert!(!dpy.is_null(), "Cannot open X display");

        let screen = XDefaultScreen(dpy);
        let root = XRootWindow(dpy, screen);

        let fb_attrs = [
            GLX_X_RENDERABLE, 1,
            GLX_DRAWABLE_TYPE, GLX_WINDOW_BIT,
            GLX_RENDER_TYPE, GLX_RGBA_BIT,
            GLX_RED_SIZE, 8,
            GLX_GREEN_SIZE, 8,
            GLX_BLUE_SIZE, 8,
            GLX_ALPHA_SIZE, 8,
            GLX_DEPTH_SIZE, 24,
            GLX_DOUBLEBUFFER, 1,
            0,
        ];

        let mut nelements: c_int = 0;
        let fbconfigs = glXChooseFBConfig(dpy, screen, fb_attrs.as_ptr(), &mut nelements);
        assert!(!fbconfigs.is_null() && nelements > 0, "No FBConfig found");
        println!("Found {} FBConfig(s)", nelements);

        let fbconfig = *fbconfigs.add(0);
        let visual_info = glXGetVisualFromFBConfig(dpy, fbconfig);
        assert!(!visual_info.is_null(), "No visual info");
        XFree(fbconfigs as *mut c_void);

        let colormap = XCreateColormap(dpy, root, (*visual_info).visual, AllocNone);
        let mut attr = XSetWindowAttributes {
            colormap,
            event_mask: KeyPressMask | StructureNotifyMask,
            background_pixmap: 0,
            background_pixel: 0,
            border_pixmap: 0,
            border_pixel: 0,
            bit_gravity: 0,
            win_gravity: 0,
            backing_store: 0,
            backing_planes: 0,
            backing_pixel: 0,
            save_under: 0,
            do_not_propagate_mask: 0,
            override_redirect: 0,
            cursor: 0,
        };

        let win = XCreateWindow(
            dpy,
            root,
            100, 100,
            1024, 768,
            0,
            (*visual_info).depth,
            InputOutput,
            (*visual_info).visual,
            CWColormap | CWEventMask,
            &mut attr,
        );

        XMapWindow(dpy, win);
        XFlush(dpy);

        // Load glXCreateContextAttribsARB
        let proc_name = b"glXCreateContextAttribsARB\0";
        let ptr = glXGetProcAddress(proc_name.as_ptr() as *const c_uchar);
        assert!(!ptr.is_null(), "glXCreateContextAttribsARB not found");
        let create_context_arb: extern "C" fn(
            *mut c_void, *mut c_void, *mut c_void, c_int, *const c_int
        ) -> *mut c_void = std::mem::transmute(ptr);

        // Create 4.2 core context directly
        let ctx_attribs = [
            GLX_CONTEXT_MAJOR_VERSION_ARB, 4,
            GLX_CONTEXT_MINOR_VERSION_ARB, 2,
            GLX_CONTEXT_PROFILE_MASK_ARB, GLX_CONTEXT_CORE_PROFILE_BIT_ARB,
            0,
        ];
        let ctx = create_context_arb(dpy, fbconfig, ptr::null_mut(), 1 as c_int, ctx_attribs.as_ptr());
        assert!(!ctx.is_null(), "Failed to create GL 4.2 core context");

        let make_current_ret = glXMakeCurrent(dpy, win, ctx);
        assert!(make_current_ret != 0, "Failed to make GL context current");
        println!("✅ OpenGL context current");

        // === FIXED loader: convert &str -> CString inside closure ===
        gl::load_with(|name: &str| {
            let c_name = CString::new(name).expect("CString::new failed for GL function name");
            let p = glXGetProcAddress(c_name.as_ptr() as *const c_uchar);
            p as *const _
        });

        // Check a couple of core function pointers (sanity)
        if (gl::CreateShader as usize) == 0 {
            eprintln!("Warning: gl::CreateShader pointer is null after gl::load_with");
        }

        // Get strings safely
        let ver_ptr = gl::GetString(gl::VERSION);
        if ver_ptr.is_null() {
            eprintln!("glGetString(GL_VERSION) returned NULL — OpenGL functions not loaded or context invalid");
        } else {
            let version = CStr::from_ptr(ver_ptr as *const c_char);
            println!("OpenGL version: {:?}", version);
        }

        let glsl_ptr = gl::GetString(gl::SHADING_LANGUAGE_VERSION);
        if !glsl_ptr.is_null() {
            let glsl_version = CStr::from_ptr(glsl_ptr as *const c_char);
            println!("GLSL version: {:?}", glsl_version);
        }

        // Load shaders (will panic with helpful message on failure)
        let vertex_shader = load_shader("vert_attrib.vert", gl::VERTEX_SHADER);
        let fragment_shader = load_shader("vert_attrib.frag", gl::FRAGMENT_SHADER);

        let program = gl::CreateProgram();
        gl::AttachShader(program, vertex_shader);
        gl::AttachShader(program, fragment_shader);
        gl::LinkProgram(program);

        let mut status = 0;
        gl::GetProgramiv(program, gl::LINK_STATUS, &mut status);
        assert!(status == gl::TRUE as i32, "Program link failed");

        gl::DeleteShader(vertex_shader);
        gl::DeleteShader(fragment_shader);

        // Vertex data
        let positions: [f32; 12] = [0.75,0.75,0.0,1.0, 0.75,-0.75,0.0,1.0, -0.75,-0.75,0.0,1.0];
        let colors: [f32; 12] = [1.0,1.0,1.0,1.0, 0.0,1.0,0.0,1.0, 0.0,0.0,1.0,1.0];

        let mut vao = 0;
        let mut vbo = [0; 2];
        gl::GenVertexArrays(1, &mut vao);
        gl::GenBuffers(2, vbo.as_mut_ptr());
        gl::BindVertexArray(vao);

        gl::BindBuffer(gl::ARRAY_BUFFER, vbo[0]);
        gl::BufferData(gl::ARRAY_BUFFER, (positions.len()*4) as isize, positions.as_ptr() as *const _, gl::STATIC_DRAW);
        gl::VertexAttribPointer(0,4,gl::FLOAT,gl::FALSE,0,ptr::null());
        gl::EnableVertexAttribArray(0);

        gl::BindBuffer(gl::ARRAY_BUFFER, vbo[1]);
        gl::BufferData(gl::ARRAY_BUFFER, (colors.len()*4) as isize, colors.as_ptr() as *const _, gl::STATIC_DRAW);
        gl::VertexAttribPointer(1,4,gl::FLOAT,gl::FALSE,0,ptr::null());
        gl::EnableVertexAttribArray(1);

        println!("🔹 Entering main loop...");
        let mut event: XEvent = std::mem::zeroed();
        loop {
            while XPending(dpy) > 0 {
                XNextEvent(dpy, &mut event);
                if event.type_ == KeyPress {
                    let xkey: &mut XKeyEvent = &mut *( &mut event as *mut XEvent as *mut XKeyEvent );
                    let keysym = XLookupKeysym(xkey, 0);
                    if keysym == XK_Escape {
                        XCloseDisplay(dpy);
                        return;
                    }
                }
            }

            gl::ClearColor(1.0,1.0,1.0,1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::UseProgram(program);
            gl::BindVertexArray(vao);
            gl::DrawArrays(gl::TRIANGLES,0,3);
            glXSwapBuffers(dpy, win);
        }
    }
}

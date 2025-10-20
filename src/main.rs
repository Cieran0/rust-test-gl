extern crate gl;
use std::ffi::{CStr, CString};
use std::fs;
use std::os::raw::{c_char, c_int, c_uchar, c_void};
use std::ptr;

use glam::Vec4;
use x11rb::connection::Connection;
use x11rb::protocol::{xproto::*, Event};
use x11rb::rust_connection::RustConnection;

// === GLX FFI ===
#[link(name = "GL")]
#[link(name = "X11")]
unsafe extern "C" {
    fn XOpenDisplay(name: *const c_char) -> *mut c_void;
    fn XCloseDisplay(dpy: *mut c_void);
    fn XDefaultScreen(dpy: *mut c_void) -> c_int;
    fn glXChooseFBConfig(
        dpy: *mut c_void,
        screen: c_int,
        attrib_list: *const c_int,
        nelements: *mut c_int,
    ) -> *mut *mut c_void;
    fn glXGetVisualFromFBConfig(dpy: *mut c_void, config: *mut c_void) -> *mut XVisualInfo;
    fn glXGetProcAddress(procName: *const c_uchar) -> *mut c_void;
    fn glXMakeCurrent(dpy: *mut c_void, drawable: u32, ctx: *mut c_void) -> c_int;
    fn glXSwapBuffers(dpy: *mut c_void, drawable: u32);
}

#[repr(C)]
struct XVisualInfo {
    visual: *mut c_void,
    visualid: u64,
    screen: c_int,
    depth: c_int,
    class: c_int,
    red_mask: u64,
    green_mask: u64,
    blue_mask: u64,
    colormap_size: c_int,
    bits_per_rgb: c_int,
}

// === GLX constants ===
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
const GLX_CONTEXT_MAJOR_VERSION_ARB: c_int = 0x2091;
const GLX_CONTEXT_MINOR_VERSION_ARB: c_int = 0x2092;
const GLX_CONTEXT_PROFILE_MASK_ARB: c_int = 0x9126;
const GLX_CONTEXT_CORE_PROFILE_BIT_ARB: c_int = 0x00000001;

// === Shader Loader ===
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

/// === Step 1: Create X11 window via x11rb ===
fn create_window() -> Result<(RustConnection, u32), Box<dyn std::error::Error>> {
    let (conn, screen_num) = RustConnection::connect(None)?;
    let screen = &conn.setup().roots[screen_num];
    let window = conn.generate_id()?;

    let width = 1024u16;
    let height = 768u16;

    conn.create_window(
        x11rb::COPY_DEPTH_FROM_PARENT,
        window,
        screen.root,
        100, 100,
        width,
        height,
        0,
        WindowClass::INPUT_OUTPUT,
        0,
        &CreateWindowAux::new()
            .background_pixel(screen.white_pixel)
            .event_mask(EventMask::KEY_PRESS | EventMask::STRUCTURE_NOTIFY),
    )?;
    conn.map_window(window)?;
    conn.flush()?;

    println!("✅ X11 window created via x11rb (id: {window})");
    Ok((conn, window))
}

/// === Step 2: Create OpenGL context via GLX ===
unsafe fn create_gl_context(dpy: *mut c_void, window: u32) -> *mut c_void { unsafe {
    let screen_id = XDefaultScreen(dpy);

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
    let fbconfigs = glXChooseFBConfig(dpy, screen_id, fb_attrs.as_ptr(), &mut nelements);
    assert!(!fbconfigs.is_null() && nelements > 0, "No FBConfig found");

    let fbconfig = *fbconfigs.add(0);
    let visual_info = glXGetVisualFromFBConfig(dpy, fbconfig);
    assert!(!visual_info.is_null(), "No visual info found");

    let proc_name = b"glXCreateContextAttribsARB\0";
    let ptr = glXGetProcAddress(proc_name.as_ptr());
    let create_context_arb: extern "C" fn(
        *mut c_void, *mut c_void, *mut c_void, c_int, *const c_int
    ) -> *mut c_void = std::mem::transmute(ptr);

    let ctx_attribs = [
        GLX_CONTEXT_MAJOR_VERSION_ARB, 4,
        GLX_CONTEXT_MINOR_VERSION_ARB, 2,
        GLX_CONTEXT_PROFILE_MASK_ARB, GLX_CONTEXT_CORE_PROFILE_BIT_ARB,
        0,
    ];

    let ctx = create_context_arb(dpy, fbconfig, ptr::null_mut(), 1, ctx_attribs.as_ptr());
    assert!(!ctx.is_null(), "Failed to create OpenGL 4.2 core context");

    let make_current_ret = glXMakeCurrent(dpy, window, ctx);
    assert!(make_current_ret != 0, "Failed to make GL context current");

    println!("✅ OpenGL 4.2 context current");
    ctx
}}

/// === Step 3: Load OpenGL functions and print version info ===
unsafe fn init_gl_functions() { unsafe {
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
}}

/// === Step 4: Create and link shader program ===
unsafe fn create_shader_program() -> u32 { unsafe {
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
    program
}}

/// === Step 5: Setup VAO/VBO for geometry ===
unsafe fn setup_geometry() -> (u32, [u32; 2]) { unsafe {
    let positions = [
        Vec4::new( 0.75,  0.75, 0.0, 1.0),
        Vec4::new( 0.75, -0.75, 0.0, 1.0),
        Vec4::new(-0.75, -0.75, 0.0, 1.0),
    ];
    let colors = [
        Vec4::new(1.0, 1.0, 1.0, 1.0),
        Vec4::new(0.0, 1.0, 0.0, 1.0),
        Vec4::new(0.0, 0.0, 1.0, 1.0),
    ];

    let mut vao = 0;
    let mut vbo = [0; 2];
    gl::GenVertexArrays(1, &mut vao);
    gl::GenBuffers(2, vbo.as_mut_ptr());
    gl::BindVertexArray(vao);

    // Position buffer
    gl::BindBuffer(gl::ARRAY_BUFFER, vbo[0]);
    gl::BufferData(
        gl::ARRAY_BUFFER,
        std::mem::size_of_val(&positions) as isize,
        positions.as_ptr() as *const _,
        gl::STATIC_DRAW,
    );
    gl::VertexAttribPointer(0, 4, gl::FLOAT, gl::FALSE, 0, ptr::null());
    gl::EnableVertexAttribArray(0);

    // Color buffer
    gl::BindBuffer(gl::ARRAY_BUFFER, vbo[1]);
    gl::BufferData(
        gl::ARRAY_BUFFER,
        std::mem::size_of_val(&colors) as isize,
        colors.as_ptr() as *const _,
        gl::STATIC_DRAW,
    );
    gl::VertexAttribPointer(1, 4, gl::FLOAT, gl::FALSE, 0, ptr::null());
    gl::EnableVertexAttribArray(1);

    (vao, vbo)
}}

/// === Main ===
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (conn, window) = create_window()?;

    unsafe {
        let dpy = XOpenDisplay(ptr::null());
        assert!(!dpy.is_null(), "Cannot open X display");

        let _ctx = create_gl_context(dpy, window);
        init_gl_functions();
        let program = create_shader_program();
        let (vao, _vbo) = setup_geometry();

        println!("🔹 Entering main loop... (Press any key to exit)");

        // === Main loop (inline in main) ===
        loop {
            if let Some(event) = conn.poll_for_event()? {
                match event {
                    Event::KeyPress(ev) => {
                        if ev.detail == 9 {
                            break
                        }
                    }
                    Event::DestroyNotify(_) => break,
                    _ => {}
                }
            }

            gl::ClearColor(1.0, 1.0, 1.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::UseProgram(program);
            gl::BindVertexArray(vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
            glXSwapBuffers(dpy, window);
        }       


        XCloseDisplay(dpy);
    }

    Ok(())
}

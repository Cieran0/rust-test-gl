pub mod glx;
pub mod x11;
pub mod window;
pub mod shader;

extern crate gl;

use glam::{Mat4, Vec4};
use x11rb::connection::Connection;
use x11rb::protocol::Event;

fn setup_geometry() -> (u32, [u32; 2]) { unsafe {
    let positions: [Vec4; 36] = [
        // Front
        Vec4::new(-0.25,  0.25, -0.25, 1.0),
        Vec4::new(-0.25, -0.25, -0.25, 1.0),
        Vec4::new( 0.25, -0.25, -0.25, 1.0),
        Vec4::new( 0.25, -0.25, -0.25, 1.0),
        Vec4::new( 0.25,  0.25, -0.25, 1.0),
        Vec4::new(-0.25,  0.25, -0.25, 1.0),

        // Right
        Vec4::new( 0.25, -0.25, -0.25, 1.0),
        Vec4::new( 0.25, -0.25,  0.25, 1.0),
        Vec4::new( 0.25,  0.25, -0.25, 1.0),
        Vec4::new( 0.25, -0.25,  0.25, 1.0),
        Vec4::new( 0.25,  0.25,  0.25, 1.0),
        Vec4::new( 0.25,  0.25, -0.25, 1.0),

        // Back
        Vec4::new( 0.25, -0.25,  0.25, 1.0),
        Vec4::new(-0.25, -0.25,  0.25, 1.0),
        Vec4::new( 0.25,  0.25,  0.25, 1.0),
        Vec4::new(-0.25, -0.25,  0.25, 1.0),
        Vec4::new(-0.25,  0.25,  0.25, 1.0),
        Vec4::new( 0.25,  0.25,  0.25, 1.0),

        // Left
        Vec4::new(-0.25, -0.25,  0.25, 1.0),
        Vec4::new(-0.25, -0.25, -0.25, 1.0),
        Vec4::new(-0.25,  0.25,  0.25, 1.0),
        Vec4::new(-0.25, -0.25, -0.25, 1.0),
        Vec4::new(-0.25,  0.25, -0.25, 1.0),
        Vec4::new(-0.25,  0.25,  0.25, 1.0),

        // Bottom
        Vec4::new(-0.25, -0.25,  0.25, 1.0),
        Vec4::new( 0.25, -0.25,  0.25, 1.0),
        Vec4::new( 0.25, -0.25, -0.25, 1.0),
        Vec4::new( 0.25, -0.25, -0.25, 1.0),
        Vec4::new(-0.25, -0.25, -0.25, 1.0),
        Vec4::new(-0.25, -0.25,  0.25, 1.0),

        // Top
        Vec4::new(-0.25, 0.25, -0.25, 1.0),
        Vec4::new( 0.25, 0.25, -0.25, 1.0),
        Vec4::new( 0.25, 0.25,  0.25, 1.0),
        Vec4::new( 0.25, 0.25,  0.25, 1.0),
        Vec4::new(-0.25, 0.25,  0.25, 1.0),
        Vec4::new(-0.25, 0.25, -0.25, 1.0),
    ];

    let colors: [Vec4; 36] = [
        Vec4::new(0.0, 0.0, 1.0, 1.0),
        Vec4::new(0.0, 0.0, 1.0, 1.0),
        Vec4::new(0.0, 0.0, 1.0, 1.0),
        Vec4::new(0.0, 0.0, 1.0, 1.0),
        Vec4::new(0.0, 0.0, 1.0, 1.0),
        Vec4::new(0.0, 0.0, 1.0, 1.0),

        Vec4::new(0.0, 1.0, 0.0, 1.0),
        Vec4::new(0.0, 1.0, 0.0, 1.0),
        Vec4::new(0.0, 1.0, 0.0, 1.0),
        Vec4::new(0.0, 1.0, 0.0, 1.0),
        Vec4::new(0.0, 1.0, 0.0, 1.0),
        Vec4::new(0.0, 1.0, 0.0, 1.0),

        Vec4::new(1.0, 1.0, 0.0, 1.0),
        Vec4::new(1.0, 1.0, 0.0, 1.0),
        Vec4::new(1.0, 1.0, 0.0, 1.0),
        Vec4::new(1.0, 1.0, 0.0, 1.0),
        Vec4::new(1.0, 1.0, 0.0, 1.0),
        Vec4::new(1.0, 1.0, 0.0, 1.0),

        Vec4::new(1.0, 0.0, 0.0, 1.0),
        Vec4::new(1.0, 0.0, 0.0, 1.0),
        Vec4::new(1.0, 0.0, 0.0, 1.0),
        Vec4::new(1.0, 0.0, 0.0, 1.0),
        Vec4::new(1.0, 0.0, 0.0, 1.0),
        Vec4::new(1.0, 0.0, 0.0, 1.0),

        Vec4::new(1.0, 0.0, 1.0, 1.0),
        Vec4::new(1.0, 0.0, 1.0, 1.0),
        Vec4::new(1.0, 0.0, 1.0, 1.0),
        Vec4::new(1.0, 0.0, 1.0, 1.0),
        Vec4::new(1.0, 0.0, 1.0, 1.0),
        Vec4::new(1.0, 0.0, 1.0, 1.0),

        Vec4::new(0.0, 1.0, 1.0, 1.0),
        Vec4::new(0.0, 1.0, 1.0, 1.0),
        Vec4::new(0.0, 1.0, 1.0, 1.0),
        Vec4::new(0.0, 1.0, 1.0, 1.0),
        Vec4::new(0.0, 1.0, 1.0, 1.0),
        Vec4::new(0.0, 1.0, 1.0, 1.0),
    ];

    let mut vao = 0;
    let mut vbo = [0; 2];

    gl::GenVertexArrays(1, &mut vao);
    gl::GenBuffers(2, vbo.as_mut_ptr());
    gl::BindVertexArray(vao);

    gl::BindBuffer(gl::ARRAY_BUFFER, vbo[0]);
    gl::BufferData(
        gl::ARRAY_BUFFER,
        std::mem::size_of_val(&positions) as isize,
        positions.as_ptr() as *const _,
        gl::STATIC_DRAW,
    );
    gl::VertexAttribPointer(0, 4, gl::FLOAT, gl::FALSE, 0, std::ptr::null());
    gl::EnableVertexAttribArray(0);

    gl::BindBuffer(gl::ARRAY_BUFFER, vbo[1]);
    gl::BufferData(
        gl::ARRAY_BUFFER,
        std::mem::size_of_val(&colors) as isize,
        colors.as_ptr() as *const _,
        gl::STATIC_DRAW,
    );
    gl::VertexAttribPointer(1, 4, gl::FLOAT, gl::FALSE, 0, std::ptr::null());
    gl::EnableVertexAttribArray(1);

    (vao, vbo)
} }

const TARGET_FPS: u64 = 60;
const FRAME_TIME: std::time::Duration = std::time::Duration::from_nanos(1_000_000_000 / TARGET_FPS);

fn main() -> Result<(), Box<dyn std::error::Error>> { unsafe {
    let dpy = x11::XOpenDisplay(std::ptr::null());
    assert!(!dpy.is_null(), "Cannot open X display");

    let (conn, window) = window::create(dpy)?;
    let _ctx = glx::create_gl_context(dpy, window);
    glx::init_gl_functions();
    gl::Enable(gl::DEPTH_TEST);

    let program = shader::create_program();
    let (vao, _vbo) = setup_geometry();
    let model_loc = gl::GetUniformLocation(program, b"model\0".as_ptr() as *const _);

    println!("Entering main loop... (Press Escape to exit, W/S to adjust speed)");

    let mut angle_x: f32 = 0.0;
    let mut rotation_speed: f32 = 0.5;
    let mut last_time = std::time::Instant::now();

    loop {
        // Handle events
        while let Some(event) = conn.poll_for_event()? {
            match event {
                Event::KeyPress(ev) => {
                    match ev.detail {
                        9 => return Ok(()), // Escape
                        25 => rotation_speed += 0.1, // W
                        39 => rotation_speed -= 0.1, // S
                        _ => {}
                    }
                }
                Event::DestroyNotify(_) => return Ok(()),
                _ => {}
            }
        }

        let now = std::time::Instant::now();
        let delta = now.duration_since(last_time).as_secs_f32();
        last_time = now;

        // Update rotation angle incrementally
        angle_x += rotation_speed * delta;

        // Render
        gl::ClearColor(0.0, 0.0, 0.0, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        gl::UseProgram(program);

        if model_loc != -1 {
            let model = Mat4::from_rotation_x(-angle_x);
            gl::UniformMatrix4fv(model_loc, 1, gl::FALSE, model.to_cols_array().as_ptr());
        }

        gl::BindVertexArray(vao);
        gl::DrawArrays(gl::TRIANGLES, 0, 36);
        glx::glXSwapBuffers(dpy, window);

        // Optional: frame pacing
        std::thread::sleep(FRAME_TIME.saturating_sub(now.elapsed()));
    }
} }

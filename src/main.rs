pub mod glx;
pub mod x11;
pub mod window;
pub mod shader;

extern crate gl;

use glam::{Mat4, Vec3, Vec4};
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
    let view_loc = gl::GetUniformLocation(program, b"view\0".as_ptr() as *const _);
    let proj_loc = gl::GetUniformLocation(program, b"projection\0".as_ptr() as *const _);

    println!("Entering main loop... (Press Escape to exit, W/S to adjust speed)");

    let cam_x: f32 = 0.0;
    let cam_y: f32 = 0.0;
    let cam_z: f32 = 1.0;

    let mut angle_x: f32 = 0.0;
    let mut angle_y: f32 = 0.0;
    let mut angle_z: f32 = 0.0;

    let mut scale_x: f32 = 1.0;
    let mut scale_y: f32 = 1.0;
    let mut scale_z: f32 = 1.0;

    let mut trans_x: f32 = 0.0;
    let mut trans_y: f32 = 0.0;
    let trans_z: f32 = 0.0;

    let mut rotation_speed: f32 = 0.0;
    let mut last_time = std::time::Instant::now();

    loop {
        while let Some(event) = conn.poll_for_event()? {
            match event {
                Event::KeyPress(ev) => {
                    match ev.detail {
                        9 => return Ok(()), // Escape
                        25 => rotation_speed += 0.1, // W
                        39 => rotation_speed -= 0.1, // S
                        53 => scale_x += 0.1, // X
                        29 => scale_y += 0.1, // Y
                        52 => scale_z += 0.1, // Z
                        111 => trans_y += 0.1, // Up
                        116 => trans_y -= 0.1, // Down
                        113 => trans_x -= 0.1, // Left
                        114 => trans_x += 0.1, // Right
                        key => {
                            println!("Pressed: {}", key)
                        }
                    }
                }
                Event::DestroyNotify(_) => return Ok(()),
                _ => {}
            }
        }

        let now = std::time::Instant::now();
        let delta = now.duration_since(last_time).as_secs_f32();
        last_time = now;

        angle_x += rotation_speed * delta;
        angle_y += rotation_speed * delta;
        angle_z += rotation_speed * delta;

        // Render
        gl::ClearColor(0.0, 0.0, 0.0, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        gl::UseProgram(program);

        if view_loc != -1 {
            let eye = Vec3::new(cam_x, cam_y, cam_z);
            let center = Vec3::new(0.0, 0.0, 0.0);
            let up = Vec3::new(0.0, 1.0, 0.0);
            let view = Mat4::look_at_rh(eye, center, up); 
            gl::UniformMatrix4fv(view_loc, 1, gl::FALSE, view.to_cols_array().as_ptr());
        }

        let proj = Mat4::orthographic_rh(-2.0, 2.0, -2.0, 2.0, -5.0, 5.0);
        if proj_loc != -1 {
            gl::UniformMatrix4fv(proj_loc, 1, gl::FALSE, proj.to_cols_array().as_ptr());
        }

        if model_loc != -1 {

            let mut stack: Vec<Mat4> = vec![Mat4::IDENTITY];

            let push = |s: &mut Vec<Mat4>| s.push(*s.last().unwrap());
            let pop = |s: &mut Vec<Mat4>| { s.pop().unwrap(); };
            let apply = |s: &mut Vec<Mat4>, m: Mat4| {
                let current = s.pop().unwrap();
                s.push(current * m);
            };

            
            gl::BindVertexArray(vao);
            
            push(&mut stack);
            apply(&mut stack, Mat4::from_translation(Vec3::new(trans_x, trans_y, trans_z)));
            apply(&mut stack, Mat4::from_translation(Vec3::new(-0.5, 0.0, 0.0)));
            apply(&mut stack, Mat4::from_scale(Vec3::new(scale_x, scale_y, scale_z)));
            apply(&mut stack, Mat4::from_rotation_x(-angle_x));
            apply(&mut stack, Mat4::from_rotation_y(angle_y));
            apply(&mut stack, Mat4::from_rotation_z(angle_z));

            gl::UniformMatrix4fv(model_loc, 1, gl::FALSE, stack.last().unwrap().to_cols_array().as_ptr());
            gl::DrawArrays(gl::TRIANGLES, 0, 36);
            pop(&mut stack);

            push(&mut stack);
            apply(&mut stack, Mat4::from_translation(Vec3::new(0.5, 0.0, 0.0)));
            apply(&mut stack, Mat4::from_scale(Vec3::new(scale_x, scale_y, scale_z)));
            apply(&mut stack, Mat4::from_rotation_x(-angle_x));
            apply(&mut stack, Mat4::from_rotation_y(angle_y));
            apply(&mut stack, Mat4::from_rotation_z(angle_z));

            gl::UniformMatrix4fv(model_loc, 1, gl::FALSE, stack.last().unwrap().to_cols_array().as_ptr());
            gl::DrawArrays(gl::TRIANGLES, 0, 36);
            pop(&mut stack);
        }

        glx::glXSwapBuffers(dpy, window);

        std::thread::sleep(FRAME_TIME.saturating_sub(now.elapsed()));
    }
} }

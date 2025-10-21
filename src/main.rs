pub mod glx;
pub mod x11;
pub mod window;
pub mod shader;

extern crate gl;

use std::i16;

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

struct Camera {
    pub x: f32,
    pub y: f32,
    pub z: f32,

    pub yaw: f32,
    pub pitch: f32,
}

impl Default for Camera {
    fn default() -> Self {
        Self { 
            x: 0f32, 
            y: 0f32, 
            z: 2f32, 
            yaw: -std::f32::consts::FRAC_PI_2, 
            pitch: 0f32
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> { unsafe {
    let dpy = x11::XOpenDisplay(std::ptr::null());
    assert!(!dpy.is_null(), "Cannot open X display");

    const WIDTH: u16 = 1920;
    const HEIGHT: u16 = 1080;

    let (conn, window) = window::create(dpy, WIDTH, HEIGHT)?;
    let _ctx = glx::create_gl_context(dpy, window);
    glx::init_gl_functions();
    gl::Enable(gl::DEPTH_TEST);

    let program = shader::create_program();
    let (vao, _vbo) = setup_geometry();
    let model_loc = gl::GetUniformLocation(program, b"model\0".as_ptr() as *const _);
    let view_loc = gl::GetUniformLocation(program, b"view\0".as_ptr() as *const _);
    let proj_loc = gl::GetUniformLocation(program, b"projection\0".as_ptr() as *const _);

    println!("Entering main loop... (Press Escape to exit, W/S to adjust speed)");

    let mut cam: Camera = Camera::default();
    
    let mut angle: Vec3 = Vec3::splat(0.0);
    let mut scale: Vec3 = Vec3::splat(1.0);
    let mut trans: Vec3 = Vec3::splat(0.0);

    let mut rotation_speed: f32 = 0.0;
    let mut last_time = std::time::Instant::now();

    let mut mouse_held: bool = false;
    let mut mouse_x_abs = i16::MIN;
    let mut mouse_y_abs = i16::MIN;


    loop {
        while let Some(event) = conn.poll_for_event()? {
            match event {
                Event::ButtonPress(ev) => {
                    match ev.detail { 
                        1 => {
                            if !mouse_held {
                                mouse_held = true;
                                println!("Mouse down!")
                            }
                        }
                        4 => cam.z += 0.1,
                        5 => cam.z -= 0.1,
                        button => {
                            println!("Button: {}", button)
                        }
                    }
                }
                Event::ButtonRelease(ev) => {
                    match ev.detail { 
                        1 => {
                            if mouse_held {
                                mouse_held = false;
                                mouse_x_abs = i16::MIN;
                                mouse_y_abs = i16::MIN;
                                println!("Mouse up!")
                            }
                        }
                        button => {
                            println!("Button: {}", button)
                        }
                    }
                }
                Event::KeyPress(ev) => {
                    match ev.detail {
                        9 => return Ok(()), // Escape
                        25 => cam.z -= 0.01, // W
                        39 => cam.z += 0.01, // S
                        38 => cam.x -= 0.01, // A
                        40 => cam.x += 0.01, // D
                        53 => scale.x += 0.1, // X
                        29 => scale.y += 0.1, // Y
                        52 => scale.z += 0.1, // Z
                        111 => trans.y += 0.1, // Up
                        116 => trans.y -= 0.1, // Down
                        113 => trans.x -= 0.1, // Left
                        114 => trans.x += 0.1, // Right
                        48 => rotation_speed += 0.1, // @
                        51 => rotation_speed -= 0.1, // #
                        27 => { 
                            cam = Camera::default();
                            angle = Vec3::splat(0.0);
                            scale = Vec3::splat(1.0);
                            trans = Vec3::splat(0.0);
                            rotation_speed = 0.0;
                        }, // R
                        key => {
                            println!("Pressed: {}", key)
                        }
                    }
                }
                Event::MotionNotify(ev) => {
                    const SENSITIVITY: f32 = 0.001;

                    if !mouse_held {
                        continue;
                    }

                    if mouse_x_abs == i16::MIN && mouse_y_abs == i16::MIN {
                        mouse_x_abs=ev.event_x;
                        mouse_y_abs=ev.event_y;
                        continue;
                    }

                    let delta_mouse_x = -(ev.event_x - mouse_x_abs);
                    let delta_mouse_y = ev.event_y - mouse_y_abs;

                    cam.yaw -= delta_mouse_x as f32 * SENSITIVITY;
                    cam.pitch -= delta_mouse_y as f32 * SENSITIVITY;

                    mouse_x_abs=ev.event_x;
                    mouse_y_abs=ev.event_y;
                }
                Event::DestroyNotify(_) => return Ok(()),
                _ => {}
            }
        }

        let now = std::time::Instant::now();
        let delta = now.duration_since(last_time).as_secs_f32();
        last_time = now;

        angle.x += rotation_speed * delta;
        angle.y += rotation_speed * delta;
        angle.z += rotation_speed * delta;

        gl::ClearColor(0.0, 0.0, 0.0, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        gl::UseProgram(program);

        if view_loc != -1 {
            let front = Vec3::new(
                cam.yaw.cos() * cam.pitch.cos(),
                cam.pitch.sin(),
                cam.yaw.sin() * cam.pitch.cos(),
            ).normalize();

            let eye = Vec3::new(cam.x, cam.y, cam.z);
            let center = eye + front;

            let up = Vec3::new(0.0, 1.0, 0.0);

            let view = Mat4::look_at_rh(eye, center, up); 

            gl::UniformMatrix4fv(view_loc, 1, gl::FALSE, view.to_cols_array().as_ptr());
        }

        let aspect: f32 = (WIDTH as f32) / (HEIGHT as f32);
        let fovy: f32 = 45.0f32.to_radians();
        let proj: Mat4 = Mat4::perspective_rh(fovy, aspect, 0.1, 10.0);
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
            apply(&mut stack, Mat4::from_translation(trans));
            apply(&mut stack, Mat4::from_translation(Vec3::new(-0.5, 0.0, 0.0)));
            apply(&mut stack, Mat4::from_scale(scale));
            apply(&mut stack, Mat4::from_rotation_x(-angle.x));
            apply(&mut stack, Mat4::from_rotation_y(angle.y));
            apply(&mut stack, Mat4::from_rotation_z(angle.z));

            gl::UniformMatrix4fv(model_loc, 1, gl::FALSE, stack.last().unwrap().to_cols_array().as_ptr());
            gl::DrawArrays(gl::TRIANGLES, 0, 36);
            pop(&mut stack);

            push(&mut stack);
            apply(&mut stack, Mat4::from_translation(Vec3::new(0.5, 0.0, 0.0)));
            apply(&mut stack, Mat4::from_scale(scale));
            apply(&mut stack, Mat4::from_rotation_x(-angle.x));
            apply(&mut stack, Mat4::from_rotation_y(angle.y));
            apply(&mut stack, Mat4::from_rotation_z(angle.z));

            gl::UniformMatrix4fv(model_loc, 1, gl::FALSE, stack.last().unwrap().to_cols_array().as_ptr());
            gl::DrawArrays(gl::TRIANGLES, 0, 36);
            pop(&mut stack);
        }

        glx::glXSwapBuffers(dpy, window);

        std::thread::sleep(FRAME_TIME.saturating_sub(now.elapsed()));
    }
} }

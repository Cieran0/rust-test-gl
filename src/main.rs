pub mod glx;
pub mod x11;
pub mod window;
pub mod shape;
pub mod shader;

use std::i16;

use gl::types::GLuint;
use glam::{Mat4, Vec3};

use crate::shape::*;

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
            x: 0.0,
            y: 0.0,
            z: 2.0,
            yaw: -std::f32::consts::FRAC_PI_2,
            pitch: 0.0,
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        let dpy = x11::XOpenDisplay(std::ptr::null());
        assert!(!dpy.is_null(), "Cannot open X display");

        const WIDTH: u16 = 1920;
        const HEIGHT: u16 = 1080;

        let window = window::create(dpy, WIDTH, HEIGHT)?;
        let _ctx = glx::create_gl_context(dpy, window);
        glx::init_gl_functions();
        gl::Enable(gl::DEPTH_TEST);

        let program = shader::create_program();
        shader::set_current_program(program);

        let shapes: Vec<Box<dyn Shape>> = vec![
            Box::new(Cube::new()),
            Box::new(Cube::new()),
            Box::new(Pyramid::new()),
            Box::new(Sphere::new()),
        ];

        let model_loc = gl::GetUniformLocation(program, b"model\0".as_ptr() as *const _);
        let view_loc = gl::GetUniformLocation(program, b"view\0".as_ptr() as *const _);
        let proj_loc = gl::GetUniformLocation(program, b"projection\0".as_ptr() as *const _);
        let colourmode_loc = gl::GetUniformLocation(program, b"colourmode\0".as_ptr() as *const _);

        println!("Entering main loop... (Press Escape to exit)");

        let mut cam = Camera::default();
        let mut angle = Vec3::splat(0.0);
        let mut scale = Vec3::splat(1.0);
        let mut trans = Vec3::splat(0.0);
        let mut rotation_speed: f32 = 0.0;
        let mut last_time = std::time::Instant::now();
        let mut mouse_held = false;
        let mut mouse_x_abs = i16::MIN;
        let mut mouse_y_abs = i16::MIN;
        let mut colourmode: GLuint = 0;
        let mut pyramid_time: f32 = 0.0;

        loop {
            while x11::XPending(dpy) != 0 {
                let mut ev: x11::XEvent = std::mem::zeroed();
                x11::XNextEvent(dpy, &mut ev);
            
                match ev.into_event() {
                    x11::Event::KeyPress(key_ev) => {
                        match key_ev.keycode {
                            9 => return Ok(()),   // Escape
                            25 => cam.z -= 0.01,  // W
                            39 => cam.z += 0.01,  // S
                            38 => cam.x -= 0.01,  // A
                            40 => cam.x += 0.01,  // D
                            53 => scale.x += 0.1,
                            29 => scale.y += 0.1,
                            52 => scale.z += 0.1,
                            111 => trans.y += 0.1, // Up
                            116 => trans.y -= 0.1, // Down
                            113 => trans.x -= 0.1, // Left
                            114 => trans.x += 0.1, // Right
                            48 => rotation_speed += 0.1,
                            51 => rotation_speed -= 0.1,
                            27 => {
                                cam = Camera::default();
                                angle = Vec3::splat(0.0);
                                scale = Vec3::splat(1.0);
                                trans = Vec3::splat(0.0);
                                rotation_speed = 0.0;
                            }
                            54 => colourmode = 1 - colourmode,
                            k => println!("Keycode: {}", k),
                        }
                    }
                    x11::Event::ButtonPress(btn_ev) => {
                        match btn_ev.button {
                            1 => {
                                if !mouse_held {
                                    mouse_held = true;
                                    mouse_x_abs = i16::MIN;
                                    println!("Mouse down!");
                                }
                            }
                            4 => {
                                let front = Vec3::new(
                                    cam.yaw.cos() * cam.pitch.cos(),
                                    cam.pitch.sin(),
                                    cam.yaw.sin() * cam.pitch.cos(),
                                ).normalize();
                                cam.x += front.x * 0.1;
                                cam.y += front.y * 0.1;
                                cam.z += front.z * 0.1;
                            }
                            5 => {
                                let front = Vec3::new(
                                    cam.yaw.cos() * cam.pitch.cos(),
                                    cam.pitch.sin(),
                                    cam.yaw.sin() * cam.pitch.cos(),
                                ).normalize();
                                cam.x -= front.x * 0.1;
                                cam.y -= front.y * 0.1;
                                cam.z -= front.z * 0.1;
                            }
                            b => println!("Button: {}", b),
                        }
                    }
                    x11::Event::ButtonRelease(btn_ev) => {
                        match btn_ev.button {
                            1 => {
                                if mouse_held {
                                    mouse_held = false;
                                    println!("Mouse up!");
                                }
                            }
                            b => println!("Button: {}", b),
                        }
                    }
                    x11::Event::Motion(motion_ev) => {
                        if !mouse_held { continue; }
                        if mouse_x_abs == i16::MIN {
                            mouse_x_abs = motion_ev.x as i16;
                            mouse_y_abs = motion_ev.y as i16;
                            continue;
                        }
                        const SENSITIVITY: f32 = 0.001;
                        let dx = -(motion_ev.x as i16 - mouse_x_abs) as f32;
                        let dy = (motion_ev.y as i16 - mouse_y_abs) as f32;
                        cam.yaw += dx * SENSITIVITY;
                        cam.pitch += dy * SENSITIVITY;
                        cam.pitch = cam.pitch.clamp(-1.5, 1.5);
                        mouse_x_abs = motion_ev.x as i16;
                        mouse_y_abs = motion_ev.y as i16;
                    }
                    x11::Event::DestroyNotify => {
                        println!("Window destroyed, exiting loop.");
                        break;
                    }
                    x11::Event::Expose => {}
                    x11::Event::KeyRelease(_) => {}
                    x11::Event::Unknown => {
                        println!("Uknown Event Occured: {}", ev.event_type());
                    }
                    x11::Event::Map(_) => {
                        println!("Uknown Event Occured, MAP : {}", ev.event_type());
                    }
                }
            }
            
            let now = std::time::Instant::now();
            let delta = now.duration_since(last_time).as_secs_f32();
            last_time = now;
            pyramid_time += delta;

            angle.x += rotation_speed * delta;
            angle.y += rotation_speed * delta;
            angle.z += rotation_speed * delta;

            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            gl::UseProgram(program);

            if colourmode_loc != -1 {
                gl::Uniform1ui(colourmode_loc, colourmode);
            }

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

            let aspect = WIDTH as f32 / HEIGHT as f32;
            let proj = Mat4::perspective_rh(45.0f32.to_radians(), aspect, 0.1, 10.0);
            if proj_loc != -1 {
                gl::UniformMatrix4fv(proj_loc, 1, gl::FALSE, proj.to_cols_array().as_ptr());
            }

            if model_loc != -1 {
                // Cube 1
                let mut shape_iter = shapes.iter();

                // Cube 1
                let model1 = Mat4::IDENTITY
                    * Mat4::from_translation(trans)
                    * Mat4::from_translation(Vec3::new(-0.5, 0.0, 0.0))
                    * Mat4::from_scale(scale)
                    * Mat4::from_rotation_x(-angle.x)
                    * Mat4::from_rotation_y(angle.y)
                    * Mat4::from_rotation_z(angle.z);
                shape_iter.next().unwrap().draw(model1);

                // Cube 2
                let model2 = Mat4::IDENTITY
                    * Mat4::from_translation(Vec3::new(0.5, 0.0, 0.0))
                    * Mat4::from_scale(scale)
                    * Mat4::from_rotation_x(-angle.x)
                    * Mat4::from_rotation_y(angle.y)
                    * Mat4::from_rotation_z(angle.z);
                shape_iter.next().unwrap().draw(model2);

                // Pyramid
                let pyramid_y = pyramid_time.sin() * 0.5;
                let pyramid_model = Mat4::from_translation(Vec3::new(0.0, pyramid_y, -1.0))
                    * Mat4::from_rotation_y(pyramid_time);
                shape_iter.next().unwrap().draw(pyramid_model);

                // Sphere
                let sphere_model = Mat4::from_translation(Vec3::new(0.0, 0.0, -2.0))
                    * Mat4::from_scale(Vec3::splat(0.7));
                shape_iter.next().unwrap().draw(sphere_model);
            }

            glx::glXSwapBuffers(dpy, window);
            std::thread::sleep(FRAME_TIME.saturating_sub(now.elapsed()));
        }
    }
}
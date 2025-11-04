use glam::{Mat4, Vec4};
use gl::types::GLuint;

use crate::shader;

pub trait Shape {
    fn draw(&self, model: Mat4);
}

pub struct Cube {
    vao: GLuint,
    vertex_count: i32,
}

impl Cube {
    pub fn new() -> Self {
        unsafe {
            let positions: [Vec4; 36] = [
                // Front
                Vec4::new(-0.25, 0.25, -0.25, 1.0),
                Vec4::new(-0.25, -0.25, -0.25, 1.0),
                Vec4::new(0.25, -0.25, -0.25, 1.0),
                Vec4::new(0.25, -0.25, -0.25, 1.0),
                Vec4::new(0.25, 0.25, -0.25, 1.0),
                Vec4::new(-0.25, 0.25, -0.25, 1.0),
                // Right
                Vec4::new(0.25, -0.25, -0.25, 1.0),
                Vec4::new(0.25, -0.25, 0.25, 1.0),
                Vec4::new(0.25, 0.25, -0.25, 1.0),
                Vec4::new(0.25, -0.25, 0.25, 1.0),
                Vec4::new(0.25, 0.25, 0.25, 1.0),
                Vec4::new(0.25, 0.25, -0.25, 1.0),
                // Back
                Vec4::new(0.25, -0.25, 0.25, 1.0),
                Vec4::new(-0.25, -0.25, 0.25, 1.0),
                Vec4::new(0.25, 0.25, 0.25, 1.0),
                Vec4::new(-0.25, -0.25, 0.25, 1.0),
                Vec4::new(-0.25, 0.25, 0.25, 1.0),
                Vec4::new(0.25, 0.25, 0.25, 1.0),
                // Left
                Vec4::new(-0.25, -0.25, 0.25, 1.0),
                Vec4::new(-0.25, -0.25, -0.25, 1.0),
                Vec4::new(-0.25, 0.25, 0.25, 1.0),
                Vec4::new(-0.25, -0.25, -0.25, 1.0),
                Vec4::new(-0.25, 0.25, -0.25, 1.0),
                Vec4::new(-0.25, 0.25, 0.25, 1.0),
                // Bottom
                Vec4::new(-0.25, -0.25, 0.25, 1.0),
                Vec4::new(0.25, -0.25, 0.25, 1.0),
                Vec4::new(0.25, -0.25, -0.25, 1.0),
                Vec4::new(0.25, -0.25, -0.25, 1.0),
                Vec4::new(-0.25, -0.25, -0.25, 1.0),
                Vec4::new(-0.25, -0.25, 0.25, 1.0),
                // Top
                Vec4::new(-0.25, 0.25, -0.25, 1.0),
                Vec4::new(0.25, 0.25, -0.25, 1.0),
                Vec4::new(0.25, 0.25, 0.25, 1.0),
                Vec4::new(0.25, 0.25, 0.25, 1.0),
                Vec4::new(-0.25, 0.25, 0.25, 1.0),
                Vec4::new(-0.25, 0.25, -0.25, 1.0),
            ];
            let colours: [Vec4; 36] = [
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
            let mut vbo = [0u32; 2];
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
                std::mem::size_of_val(&colours) as isize,
                colours.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );
            gl::VertexAttribPointer(1, 4, gl::FLOAT, gl::FALSE, 0, std::ptr::null());
            gl::EnableVertexAttribArray(1);

            gl::BindVertexArray(0);

            Self {
                vao,
                vertex_count: 36,
            }
        }
    }
}

impl Shape for Cube {
    fn draw(&self, model: Mat4) {
        unsafe {
            let model_loc = gl::GetUniformLocation(shader::get_current_program(), b"model\0".as_ptr() as *const _);
            if model_loc != -1 {
                gl::UniformMatrix4fv(model_loc, 1, gl::FALSE, model.to_cols_array().as_ptr());
            }
            gl::BindVertexArray(self.vao);
            gl::DrawArrays(gl::TRIANGLES, 0, self.vertex_count);
        }
    }
}

impl Drop for Cube {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.vao);
        }
    }
}

pub struct Pyramid {
    vao: GLuint,
    vertex_count: i32,
}

impl Pyramid {
    pub fn new() -> Self {
        unsafe {
            let positions: [Vec4; 18] = [
                Vec4::new(-0.25, 0.0, -0.25, 1.0),
                Vec4::new(0.25, 0.0, -0.25, 1.0),
                Vec4::new(0.25, 0.0, 0.25, 1.0),
                Vec4::new(0.25, 0.0, 0.25, 1.0),
                Vec4::new(-0.25, 0.0, 0.25, 1.0),
                Vec4::new(-0.25, 0.0, -0.25, 1.0),
                Vec4::new(-0.25, 0.0, -0.25, 1.0),
                Vec4::new(0.25, 0.0, -0.25, 1.0),
                Vec4::new(0.0, 0.5, 0.0, 1.0),
                Vec4::new(0.25, 0.0, -0.25, 1.0),
                Vec4::new(0.25, 0.0, 0.25, 1.0),
                Vec4::new(0.0, 0.5, 0.0, 1.0),
                Vec4::new(0.25, 0.0, 0.25, 1.0),
                Vec4::new(-0.25, 0.0, 0.25, 1.0),
                Vec4::new(0.0, 0.5, 0.0, 1.0),
                Vec4::new(-0.25, 0.0, 0.25, 1.0),
                Vec4::new(-0.25, 0.0, -0.25, 1.0),
                Vec4::new(0.0, 0.5, 0.0, 1.0),
            ];
            let colours: [Vec4; 18] = [
                Vec4::new(0.5, 0.5, 0.5, 1.0),
                Vec4::new(0.5, 0.5, 0.5, 1.0),
                Vec4::new(0.5, 0.5, 0.5, 1.0),
                Vec4::new(0.5, 0.5, 0.5, 1.0),
                Vec4::new(0.5, 0.5, 0.5, 1.0),
                Vec4::new(0.5, 0.5, 0.5, 1.0),
                Vec4::new(1.0, 0.0, 0.0, 1.0),
                Vec4::new(1.0, 0.0, 0.0, 1.0),
                Vec4::new(1.0, 0.0, 0.0, 1.0),
                Vec4::new(0.0, 1.0, 0.0, 1.0),
                Vec4::new(0.0, 1.0, 0.0, 1.0),
                Vec4::new(0.0, 1.0, 0.0, 1.0),
                Vec4::new(0.0, 0.0, 1.0, 1.0),
                Vec4::new(0.0, 0.0, 1.0, 1.0),
                Vec4::new(0.0, 0.0, 1.0, 1.0),
                Vec4::new(1.0, 1.0, 0.0, 1.0),
                Vec4::new(1.0, 1.0, 0.0, 1.0),
                Vec4::new(1.0, 1.0, 0.0, 1.0),
            ];

            let mut vao = 0;
            let mut vbo = [0u32; 2];
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
                std::mem::size_of_val(&colours) as isize,
                colours.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );
            gl::VertexAttribPointer(1, 4, gl::FLOAT, gl::FALSE, 0, std::ptr::null());
            gl::EnableVertexAttribArray(1);

            gl::BindVertexArray(0);

            Self {
                vao,
                vertex_count: 18,
            }
        }
    }
}

impl Shape for Pyramid {
    fn draw(&self, model: Mat4) {
        unsafe {
            let model_loc = gl::GetUniformLocation(shader::get_current_program(), b"model\0".as_ptr() as *const _);
            if model_loc != -1 {
                gl::UniformMatrix4fv(model_loc, 1, gl::FALSE, model.to_cols_array().as_ptr());
            }
            gl::BindVertexArray(self.vao);
            gl::DrawArrays(gl::TRIANGLES, 0, self.vertex_count);
        }
    }
}

impl Drop for Pyramid {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.vao);
        }
    }
}

// For now, a simple sphere using same approach (non-indexed, low-res)
pub struct Sphere {
    vao: GLuint,
    vertex_count: i32,
}

impl Sphere {
    pub fn new() -> Self {
        // Generate a simple UV sphere with 10 latitudes and 10 longitudes
        let (positions, colours) = Self::generate_sphere(80, 80, Vec4::new(0.8, 0.2, 0.6, 1.0));
        let vertex_count = positions.len() as i32;

        unsafe {
            let mut vao = 0;
            let mut vbo = [0u32; 2];
            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(2, vbo.as_mut_ptr());

            gl::BindVertexArray(vao);

            gl::BindBuffer(gl::ARRAY_BUFFER, vbo[0]);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (positions.len() * std::mem::size_of::<Vec4>()) as isize,
                positions.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );
            gl::VertexAttribPointer(0, 4, gl::FLOAT, gl::FALSE, 0, std::ptr::null());
            gl::EnableVertexAttribArray(0);

            gl::BindBuffer(gl::ARRAY_BUFFER, vbo[1]);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (colours.len() * std::mem::size_of::<Vec4>()) as isize,
                colours.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );
            gl::VertexAttribPointer(1, 4, gl::FLOAT, gl::FALSE, 0, std::ptr::null());
            gl::EnableVertexAttribArray(1);

            gl::BindVertexArray(0);

            Self {
                vao,
                vertex_count,
            }
        }
    }

    fn generate_sphere(num_lats: usize, num_longs: usize, color: Vec4) -> (Vec<Vec4>, Vec<Vec4>) {
        let mut positions = Vec::new();
        let mut colours = Vec::new();

        // North pole
        positions.push(Vec4::new(0.0, 0.0, 1.0, 1.0));
        colours.push(color);

        let lat_step = std::f32::consts::PI / (num_lats as f32);
        let long_step = 2.0 * std::f32::consts::PI / (num_longs as f32);

        for i in 1..num_lats {
            let lat = i as f32 * lat_step;
            let sin_lat = lat.sin();
            let cos_lat = lat.cos();

            for j in 0..num_longs {
                let long = j as f32 * long_step;
                let sin_long = long.sin();
                let cos_long = long.cos();

                let x = cos_long * sin_lat;
                let y = sin_long * sin_lat;
                let z = cos_lat;

                positions.push(Vec4::new(x, y, z, 1.0));
                colours.push(color);
            }
        }

        // South pole
        positions.push(Vec4::new(0.0, 0.0, -1.0, 1.0));
        colours.push(color);

        // Now build triangle list (non-indexed)
        let mut final_positions = Vec::new();
        let mut final_colours = Vec::new();

        // Top cap
        for i in 0..num_longs {
            let i1 = 1 + i;
            let i2 = 1 + (i + 1) % num_longs;
            final_positions.push(positions[0]);
            final_positions.push(positions[i1]);
            final_positions.push(positions[i2]);
            final_colours.push(colours[0]);
            final_colours.push(colours[i1]);
            final_colours.push(colours[i2]);
        }

        // Middle bands
        for lat in 0..(num_lats - 2) {
            let base = 1 + lat * num_longs;
            let next = 1 + (lat + 1) * num_longs;
            for i in 0..num_longs {
                let i1 = base + i;
                let i2 = base + (i + 1) % num_longs;
                let i3 = next + i;
                let i4 = next + (i + 1) % num_longs;

                // First triangle
                final_positions.push(positions[i1]);
                final_positions.push(positions[i2]);
                final_positions.push(positions[i3]);
                final_colours.push(colours[i1]);
                final_colours.push(colours[i2]);
                final_colours.push(colours[i3]);

                // Second triangle
                final_positions.push(positions[i3]);
                final_positions.push(positions[i2]);
                final_positions.push(positions[i4]);
                final_colours.push(colours[i3]);
                final_colours.push(colours[i2]);
                final_colours.push(colours[i4]);
            }
        }

        // Bottom cap
        let south_pole_index = positions.len() - 1;
        let base = 1 + (num_lats - 2) * num_longs;
        for i in 0..num_longs {
            let i1 = base + i;
            let i2 = base + (i + 1) % num_longs;
            final_positions.push(positions[i1]);
            final_positions.push(positions[i2]);
            final_positions.push(positions[south_pole_index]);
            final_colours.push(colours[i1]);
            final_colours.push(colours[i2]);
            final_colours.push(colours[south_pole_index]);
        }

        (final_positions, final_colours)
    }
}

impl Shape for Sphere {
    fn draw(&self, model: Mat4) {
        unsafe {
            let model_loc = gl::GetUniformLocation(shader::get_current_program(), b"model\0".as_ptr() as *const _);
            if model_loc != -1 {
                gl::UniformMatrix4fv(model_loc, 1, gl::FALSE, model.to_cols_array().as_ptr());
            }
            gl::BindVertexArray(self.vao);
            gl::DrawArrays(gl::TRIANGLES, 0, self.vertex_count);
        }
    }
}

impl Drop for Sphere {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.vao);
        }
    }
}
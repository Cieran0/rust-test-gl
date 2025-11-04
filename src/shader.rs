use std::{ffi::CString, fs, ptr};
use std::sync::atomic::{AtomicU32, Ordering};

static CURRENT_PROGRAM: AtomicU32 = AtomicU32::new(0);

pub fn set_current_program(program: u32) {
    CURRENT_PROGRAM.store(program, Ordering::Relaxed);
}

pub fn get_current_program() -> u32 {
    CURRENT_PROGRAM.load(Ordering::Relaxed)
}

pub fn load_shader(path: &str, shader_type: u32) -> u32 {
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

pub fn create_program() -> u32 {
    unsafe {
        let vertex_shader = load_shader("lab2.vert", gl::VERTEX_SHADER);
        let fragment_shader = load_shader("lab2.frag", gl::FRAGMENT_SHADER);

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
    }
}
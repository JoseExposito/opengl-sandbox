use std::ffi::CString;

use gl;

#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub enum ShaderType {
    VertexShader = gl::VERTEX_SHADER,
    FragmentShader = gl::FRAGMENT_SHADER,
}

pub struct Shader {
    pub(crate) id: u32,
}

impl Shader {
    pub fn new(shader_type: ShaderType, source: &str) -> Self {
        let id: u32 = unsafe { gl::CreateShader(shader_type as u32) };
        if id == 0 {
            panic!("Error creating {:?} shader object", shader_type);
        }

        let c_source = CString::new(source).expect("Error creating CString from shader source");

        unsafe {
            gl::ShaderSource(id, 1, &c_source.as_ptr(), std::ptr::null());
            gl::CompileShader(id);
        }

        let mut success = 0;
        unsafe { gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success) };

        if success == 0 {
            let mut error_message: [u8; 512] = [0; 512];
            unsafe {
                gl::GetShaderInfoLog(id, 512, &mut 0, error_message.as_mut_ptr().cast());
            }
            panic!(
                "Error compiling {:?} shader:\n{}",
                &shader_type,
                String::from_utf8_lossy(&error_message)
            );
        }

        Self { id }
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe { gl::DeleteShader(self.id) };
    }
}

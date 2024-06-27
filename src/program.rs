use crate::shader::Shader;

use gl;

pub struct Program {
    id: u32,
}

impl Program {
    pub fn new(vertex_shader: &Shader, fragment_shader: &Shader) -> Self {
        let id = unsafe { gl::CreateProgram() };
        if id == 0 {
            panic!("Error creating shader program object");
        }

        unsafe {
            gl::AttachShader(id, vertex_shader.id);
            gl::AttachShader(id, fragment_shader.id);
            gl::LinkProgram(id);
        }

        let mut success = 0;
        unsafe { gl::GetProgramiv(id, gl::LINK_STATUS, &mut success) };

        if success == 0 {
            let mut error_message: [u8; 512] = [0; 512];
            unsafe {
                gl::GetProgramInfoLog(id, 512, &mut 0, error_message.as_mut_ptr().cast());
            }
            panic!(
                "Error linking program:\n{}",
                String::from_utf8_lossy(&error_message)
            );
        }

        Self { id }
    }

    pub fn bind(&self) {
        unsafe { gl::UseProgram(self.id) };
    }

    pub fn unbind(&self) {
        unsafe { gl::UseProgram(0) };
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe { gl::DeleteProgram(self.id) };
    }
}

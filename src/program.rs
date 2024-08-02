use crate::shader::Shader;
use crate::texture::Texture;

use gl;
use nalgebra_glm as glm;
use std::ffi::CString;

pub struct Program {
    id: u32,
    textures: Vec<Texture>,
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

        Self {
            id,
            textures: Vec::new(),
        }
    }

    pub fn bind(&self) {
        unsafe { gl::UseProgram(self.id) };
    }

    pub fn unbind(&self) {
        unsafe { gl::UseProgram(0) };
    }

    pub fn add_texture2d(&mut self, texture: Texture) {
        let c_uniform_name = CString::new(texture.get_uniform_name())
            .expect("Error creating CString from texture uniform name");

        self.bind();
        texture.bind();
        unsafe {
            let uniform_location = gl::GetUniformLocation(self.id, c_uniform_name.as_ptr());
            assert_ne!(uniform_location, -1);
            gl::Uniform1i(uniform_location, texture.get_slot() as i32);
        }
        texture.unbind();
        self.unbind();

        self.textures.push(texture);
    }

    pub fn bind_textures(&self) {
        self.textures.iter().for_each(|texture| texture.bind());
    }

    pub fn unbind_textures(&self) {
        self.textures.iter().for_each(|texture| texture.unbind());
    }

    pub fn set_uniform_4f(&self, uniform_name: &str, v0: f32, v1: f32, v2: f32, v3: f32) {
        let c_uniform_name =
            CString::new(uniform_name).expect("Error creating CString from uniform name");

        self.bind();
        unsafe {
            let uniform_location = gl::GetUniformLocation(self.id, c_uniform_name.as_ptr());
            assert_ne!(uniform_location, -1);
            gl::Uniform4f(uniform_location, v0, v1, v2, v3);
        }
        self.unbind();
    }

    pub fn set_uniform_mat4(&self, uniform_name: &str, mat4: &glm::Mat4) {
        let c_uniform_name =
            CString::new(uniform_name).expect("Error creating CString from uniform name");

        self.bind();
        unsafe {
            let uniform_location = gl::GetUniformLocation(self.id, c_uniform_name.as_ptr());
            assert_ne!(uniform_location, -1);
            gl::UniformMatrix4fv(uniform_location, 1, gl::FALSE, mat4.as_ptr());
        }
        self.unbind();
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe { gl::DeleteProgram(self.id) };
    }
}

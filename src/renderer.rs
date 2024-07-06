use gl;

use crate::{program::Program, vertex_array::VertexArray};

#[derive(Default)]
pub struct Renderer {}

impl Renderer {
    pub fn clear(&self) {
        unsafe { gl::Clear(gl::COLOR_BUFFER_BIT) };
    }

    pub fn draw(&self, vao: &VertexArray, program: &Program) {
        let num_indices = vao.get_num_indices_to_draw();

        program.bind();
        program.bind_textures();
        vao.bind();

        unsafe {
            gl::DrawElements(
                gl::TRIANGLES,
                num_indices as i32,
                gl::UNSIGNED_INT,
                std::ptr::null(),
            )
        };

        vao.unbind();
        program.unbind_textures();
        program.unbind();
    }
}

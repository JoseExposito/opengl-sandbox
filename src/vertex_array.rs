use gl;

use crate::element_buffer::ElementBuffer;
use crate::vertex_buffer::VertexBuffer;
use crate::vertex_buffer_layout::VertexBufferLayout;

pub struct VertexArray {
    id: u32,
}

impl VertexArray {
    pub fn new<T: Sized>(vertices: &[T], indices: &[u32], layouts: &VertexBufferLayout) -> Self {
        let mut id = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut id);
            assert_ne!(id, 0);
            gl::BindVertexArray(id);
        }

        let ebo = ElementBuffer::new(&indices);
        ebo.bind();

        let vbo = VertexBuffer::new(&vertices);
        vbo.bind();
        Self::add_layouts(&layouts);

        vbo.unbind();
        unsafe { gl::BindVertexArray(0) };

        Self { id }
    }

    fn add_layouts(layouts: &VertexBufferLayout) {
        let mut location = 0;
        let stride = layouts.get_stride();

        for layout in layouts.get_layouts() {
            let normalized = match layout.normalized {
                true => gl::TRUE,
                false => gl::FALSE,
            };

            unsafe {
                gl::VertexAttribPointer(
                    location,
                    layout.count as i32,
                    layout.layout_type as u32,
                    normalized,
                    stride as i32,
                    layout.offset as *const _,
                );
                gl::EnableVertexAttribArray(location);
            }

            location += 1;
        }
    }

    pub fn bind(&self) {
        unsafe { gl::BindVertexArray(self.id) };
    }

    pub fn unbind(&self) {
        unsafe { gl::BindVertexArray(0) };
    }
}

impl Drop for VertexArray {
    fn drop(&mut self) {
        unsafe { gl::DeleteVertexArrays(1, &mut self.id) };
    }
}

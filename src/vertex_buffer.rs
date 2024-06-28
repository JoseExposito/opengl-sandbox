use gl;

pub struct VertexBuffer {
    id: u32,
}

impl VertexBuffer {
    pub fn new<T: Sized>(vertices: &[T]) -> Self {
        assert_ne!(vertices.len(), 0);

        let mut id = 0;
        let size = vertices.len() * std::mem::size_of_val(&vertices[0]);

        unsafe {
            gl::GenBuffers(1, &mut id);
            assert_ne!(id, 0);
            gl::BindBuffer(gl::ARRAY_BUFFER, id);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                size as isize,
                vertices.as_ptr().cast(),
                gl::STATIC_DRAW,
            );
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }

        Self { id }
    }

    pub fn bind(&self) {
        unsafe { gl::BindBuffer(gl::ARRAY_BUFFER, self.id) };
    }

    pub fn unbind(&self) {
        unsafe { gl::BindBuffer(gl::ARRAY_BUFFER, 0) };
    }
}

impl Drop for VertexBuffer {
    fn drop(&mut self) {
        unsafe { gl::DeleteBuffers(1, &mut self.id) };
    }
}

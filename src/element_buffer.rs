use gl;

pub struct ElementBuffer {
    id: u32,
    // Useful to know how many indices are needed by glDrawElements()
    num_indices: u32,
}

impl ElementBuffer {
    pub fn new(indices: &[u32]) -> Self {
        assert_ne!(indices.len(), 0);

        let mut id = 0;
        let num_indices = indices.len() as u32;
        let size = indices.len() * std::mem::size_of_val(&indices[0]);

        unsafe {
            gl::GenBuffers(1, &mut id);
            assert_ne!(id, 0);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, id);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                size as isize,
                indices.as_ptr().cast(),
                gl::STATIC_DRAW,
            );
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
        }

        Self { id, num_indices }
    }

    pub fn get_num_indices(self) -> u32 {
        self.num_indices
    }

    pub fn bind(&self) {
        unsafe { gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.id) };
    }

    pub fn unbind(&self) {
        unsafe { gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0) };
    }
}

impl Drop for ElementBuffer {
    fn drop(&mut self) {
        unsafe { gl::DeleteBuffers(1, &mut self.id) };
    }
}

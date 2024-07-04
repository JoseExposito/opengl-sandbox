use gl;
use image;

pub struct Texture {
    id: u32,
}

impl Texture {
    pub fn new(path: &str) -> Self {
        let mut id = 0;

        let img = image::open(path).unwrap().into_rgb8();

        unsafe {
            gl::GenTextures(1, &mut id);
            assert_ne!(id, 0);
            gl::BindTexture(gl::TEXTURE_2D, id);

            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_WRAP_S,
                gl::CLAMP_TO_BORDER as i32,
            );
            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_WRAP_T,
                gl::CLAMP_TO_BORDER as i32,
            );
            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_MIN_FILTER,
                gl::LINEAR_MIPMAP_LINEAR as i32,
            );
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA8 as i32,
                img.width() as i32,
                img.height() as i32,
                0,
                gl::RGB,           // Source image format
                gl::UNSIGNED_BYTE, // Source image data type
                img.as_ptr() as *const _,
            );
            gl::GenerateMipmap(gl::TEXTURE_2D);

            gl::BindTexture(gl::TEXTURE_2D, 0);
        }

        Self { id }
    }

    pub fn bind(&self, slot: u32) {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0 + slot);
            gl::BindTexture(gl::TEXTURE_2D, self.id);
        }
    }

    pub fn unbind(&self) {
        unsafe { gl::BindTexture(gl::TEXTURE_2D, 0) };
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe { gl::DeleteTextures(1, &mut self.id) };
    }
}

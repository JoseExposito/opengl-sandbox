use gl;
use image;
use image::imageops;

pub struct Texture {
    id: u32,
    uniform_name: String,
    slot: u32,
}

impl Texture {
    pub fn new(uniform_name: &str, path: &str, slot: u32) -> Self {
        let mut id = 0;

        let valid_slot_range = 0..31;
        assert!(valid_slot_range.contains(&slot));

        let mut img = image::open(path).unwrap();
        imageops::flip_vertical_in_place(&mut img);

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
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
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
                img.into_rgb8().into_raw().as_ptr() as *const _,
            );
            gl::GenerateMipmap(gl::TEXTURE_2D);

            gl::BindTexture(gl::TEXTURE_2D, 0);
        }

        Self {
            id,
            uniform_name: String::from(uniform_name),
            slot,
        }
    }

    pub fn get_uniform_name(&self) -> &str {
        self.uniform_name.as_str()
    }

    pub fn get_slot(&self) -> u32 {
        self.slot
    }
    pub fn bind(&self) {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0 + self.slot);
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

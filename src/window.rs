use gl;
use glfw::{fail_on_errors, Context};

pub struct Window {
    pub glfw: glfw::Glfw,
    pub window: glfw::PWindow,
}

impl Window {
    pub fn new(width: u32, height: u32, title: &str) -> Self {
        let mut glfw = glfw::init(glfw::fail_on_errors!()).unwrap();
        glfw.window_hint(glfw::WindowHint::ContextVersionMajor(3));
        glfw.window_hint(glfw::WindowHint::ContextVersionMinor(3));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(
            glfw::OpenGlProfileHint::Core,
        ));

        let (mut window, _) = glfw
            .create_window(width, height, title, glfw::WindowMode::Windowed)
            .expect("Failed to create window");

        window.make_current();
        window.set_framebuffer_size_callback(|_, width, height| unsafe {
            gl::Viewport(0, 0, width, height)
        });

        gl::load_with(|ptr| window.get_proc_address(ptr) as *const _);

        Self { glfw, window }
    }

    pub fn get_time(&self) -> f32 {
        self.glfw.get_time() as f32
    }
}

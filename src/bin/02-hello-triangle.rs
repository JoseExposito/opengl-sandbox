use gl;
use glfw::Context;
use opengl_sandbox::window;

fn main() {
    let mut w = window::Window::new(800, 600, "Hello Triangle!");

    while !w.window.should_close() {
        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        w.window.swap_buffers();
        w.glfw.poll_events();
    }
}

use gl;
use glfw::{fail_on_errors, Context};

const WIN_WIDTH: u32 = 800;
const WIN_HEIGHT: u32 = 600;
const WIN_TITLE: &str = "Hello Window!";

fn process_input(window: &mut glfw::PWindow) {
    if window.get_key(glfw::Key::Escape) == glfw::Action::Press {
        window.set_should_close(true)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut glfw = glfw::init(glfw::fail_on_errors!())?;
    glfw.window_hint(glfw::WindowHint::ContextVersionMajor(3));
    glfw.window_hint(glfw::WindowHint::ContextVersionMinor(3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(
        glfw::OpenGlProfileHint::Core,
    ));

    let (mut window, _) = glfw
        .create_window(WIN_WIDTH, WIN_HEIGHT, WIN_TITLE, glfw::WindowMode::Windowed)
        .expect("Failed to create window");

    window.make_current();
    window.set_framebuffer_size_callback(|_, width, height| unsafe {
        gl::Viewport(0, 0, width, height)
    });

    gl::load_with(|ptr| window.get_proc_address(ptr) as *const _);

    while !window.should_close() {
        process_input(&mut window);

        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        window.swap_buffers();
        glfw.poll_events();
    }

    Ok(())
}

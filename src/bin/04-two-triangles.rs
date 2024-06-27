use gl;
use glfw::Context;
use opengl_sandbox::program::Program;
use opengl_sandbox::shader::{Shader, ShaderType};
use opengl_sandbox::window::Window;

const VERTEX_SHADER_SRC: &str = "#version 330 core
layout (location = 0) in vec2 aPos;

void main()
{
    gl_Position = vec4(aPos.xy, 0.0, 1.0);
}";

const FRAGMENT_SHADER_SRC: &str = "#version 330 core
out vec4 FragColor;

void main()
{
    FragColor = vec4(1.0f, 0.5f, 0.2f, 1.0f);
}";

fn main() {
    let mut w = Window::new(800, 600, "Two triangles");

    let vertex_shader = Shader::new(ShaderType::VertexShader, VERTEX_SHADER_SRC);
    let fragment_shader = Shader::new(ShaderType::FragmentShader, FRAGMENT_SHADER_SRC);
    let program = Program::new(&vertex_shader, &fragment_shader);

    let vertices = [
        // Triangle 1
        0.2f32, -0.5, // Bottom left
        0.5, -0.5, // Bottom right
        0.5, 0.5, // Top right
        // Triangle 2
        -0.5, 0.5, // Top left
        -0.2, 0.5, // Top right
        -0.5, -0.5, // Bottom left
    ];

    let indices = [
        0u32, 1, 2, // Triangle 1
        3, 4, 5, // Triangle 2
    ];

    let mut vao = 0;
    let mut vbo = 0;
    let mut ebo = 0;

    unsafe {
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);

        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            std::mem::size_of_val(&vertices) as isize,
            vertices.as_ptr().cast(),
            gl::STATIC_DRAW,
        );

        gl::GenBuffers(1, &mut ebo);
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
        gl::BufferData(
            gl::ELEMENT_ARRAY_BUFFER,
            std::mem::size_of_val(&indices) as isize,
            indices.as_ptr().cast(),
            gl::STATIC_DRAW,
        );

        let location = 0;
        gl::VertexAttribPointer(
            location,
            2,
            gl::FLOAT,
            gl::FALSE,
            2 * std::mem::size_of::<f32>() as i32,
            0 as *const _,
        );
        gl::EnableVertexAttribArray(location);

        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(vao);
    }

    while !w.window.should_close() {
        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        program.bind();
        unsafe {
            gl::BindVertexArray(vao);
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null());
            gl::BindVertexArray(0);
        }
        program.unbind();

        w.window.swap_buffers();
        w.glfw.poll_events();
    }

    unsafe {
        gl::DeleteBuffers(1, &mut ebo);
        gl::DeleteBuffers(1, &mut vbo);
        gl::DeleteVertexArrays(1, &mut vao);
    }
}

use gl;
use glfw::Context;
use opengl_sandbox::program::Program;
use opengl_sandbox::shader::{Shader, ShaderType};
use opengl_sandbox::window::Window;

const VERTEX_SHADER_SRC: &str = "#version 330 core
layout (location = 0) in vec3 aPos;

void main()
{
    gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);
}";

const FRAGMENT_SHADER_SRC: &str = "#version 330 core
out vec4 FragColor;

void main()
{
    FragColor = vec4(1.0f, 0.5f, 0.2f, 1.0f);
}";

fn main() {
    let mut w = Window::new(800, 600, "Hello Rectangle!");

    let vertex_shader = Shader::new(ShaderType::VertexShader, VERTEX_SHADER_SRC);
    let fragment_shader = Shader::new(ShaderType::FragmentShader, FRAGMENT_SHADER_SRC);
    let program = Program::new(&vertex_shader, &fragment_shader);

    // Create the buffers
    let vertices = [
        0.5f32, 0.5, 0.0, // top right
        0.5, -0.5, 0.0, // bottom right
        -0.5, -0.5, 0.0, // bottom left
        -0.5, 0.5, 0.0, // top left
    ];

    let indices = [
        0u32, 1, 3, // First triangle
        1, 2, 3, // Second triangle
    ];

    let mut vao = 0;
    let mut vbo = 0;
    let mut ebo = 0;

    unsafe {
        gl::GenVertexArrays(1, &mut vao);
        gl::GenBuffers(1, &mut vbo);
        gl::GenBuffers(1, &mut ebo);

        // Bind the Vertex Array Object
        gl::BindVertexArray(vao);

        // Copy the vertices to the Vertex Buffer Object
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            std::mem::size_of_val(&vertices) as isize,
            vertices.as_ptr().cast(),
            gl::STATIC_DRAW,
        );

        // Copy the indices to the Element Buffer Object
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
        gl::BufferData(
            gl::ELEMENT_ARRAY_BUFFER,
            std::mem::size_of_val(&indices) as isize,
            indices.as_ptr().cast(),
            gl::STATIC_DRAW,
        );

        // Set the vertex attributes pointers
        let location = 0; // layout (location = 0)
        gl::VertexAttribPointer(
            location,
            3, // in vec3 aPos;
            gl::FLOAT,
            gl::FALSE,
            3 * std::mem::size_of::<f32>() as i32,
            0 as *const _,
        );
        gl::EnableVertexAttribArray(location);

        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);

        // Do NOT unbind the EBO while a VAO is active as the bound element buffer object IS stored in the VAO
        // gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
    }

    // Draw in wireframe mode to see both triangles
    unsafe { gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE) };

    while !w.window.should_close() {
        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        program.use_program();

        unsafe {
            gl::BindVertexArray(vao);
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null());
            gl::BindVertexArray(0);
        }

        Program::use_no_program();
        w.window.swap_buffers();
        w.glfw.poll_events();
    }

    unsafe {
        gl::DeleteVertexArrays(1, &mut vao);
        gl::DeleteBuffers(1, &mut vbo);
        gl::DeleteBuffers(1, &mut ebo);
    }
}

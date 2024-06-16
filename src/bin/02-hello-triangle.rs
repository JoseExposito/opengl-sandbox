use std::ffi::CString;

use gl;
use glfw::Context;
use opengl_sandbox::window;

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
    let mut w = window::Window::new(800, 600, "Hello Triangle!");

    // Vertex shader
    let vertex_shader = unsafe { gl::CreateShader(gl::VERTEX_SHADER) };
    let src = CString::new(VERTEX_SHADER_SRC).expect("CString::new failed");
    unsafe {
        gl::ShaderSource(vertex_shader, 1, &src.as_ptr(), std::ptr::null());
        gl::CompileShader(vertex_shader);
    }

    let mut success = 0;
    unsafe {
        gl::GetShaderiv(vertex_shader, gl::COMPILE_STATUS, &mut success);
    }

    if success == 0 {
        let mut error_message: [u8; 512] = [0; 512];
        unsafe {
            gl::GetShaderInfoLog(
                vertex_shader,
                512,
                &mut 0,
                error_message.as_mut_ptr().cast(),
            );
        }
        panic!(
            "Error compiling vertex shader:\n{}",
            String::from_utf8_lossy(&error_message)
        );
    }

    // Fragment shader
    let fragment_shader = unsafe { gl::CreateShader(gl::FRAGMENT_SHADER) };
    let src = CString::new(FRAGMENT_SHADER_SRC).unwrap();
    unsafe {
        gl::ShaderSource(fragment_shader, 1, &src.as_ptr(), std::ptr::null());
        gl::CompileShader(fragment_shader);
    }

    let mut success = 0;
    unsafe {
        gl::GetShaderiv(fragment_shader, gl::COMPILE_STATUS, &mut success);
    }

    if success == 0 {
        let mut error_message: [u8; 512] = [0; 512];
        unsafe {
            gl::GetShaderInfoLog(
                fragment_shader,
                512,
                &mut 0,
                error_message.as_mut_ptr().cast(),
            );
        }
        panic!(
            "Error compiling fragment shader:\n{}",
            String::from_utf8_lossy(&error_message)
        );
    }

    // Link shaders to the program
    let shader_program = unsafe { gl::CreateProgram() };
    unsafe {
        gl::AttachShader(shader_program, vertex_shader);
        gl::AttachShader(shader_program, fragment_shader);
        gl::LinkProgram(shader_program);
    }

    let mut success = 0;
    unsafe {
        gl::GetProgramiv(shader_program, gl::LINK_STATUS, &mut success);
    }

    if success == 0 {
        let mut error_message: [u8; 512] = [0; 512];
        unsafe {
            gl::GetProgramInfoLog(
                shader_program,
                512,
                &mut 0,
                error_message.as_mut_ptr().cast(),
            );
            panic!(
                "Error linking program:\n{}",
                String::from_utf8_lossy(&error_message)
            );
        }
    }

    unsafe {
        gl::UseProgram(shader_program);
        gl::DeleteShader(vertex_shader);
        gl::DeleteShader(fragment_shader);
    }

    // Create the buffers
    let vertices = [
        -0.5f32, -0.5, 0.0, //
        0.5, -0.5, 0.0, //
        0.0, 0.5, 0.0, //
    ];

    let mut vao = 0;
    let mut vbo = 0;

    unsafe {
        gl::GenVertexArrays(1, &mut vao);
        gl::GenBuffers(1, &mut vbo);

        gl::BindVertexArray(vao);

        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            std::mem::size_of_val(&vertices) as isize,
            vertices.as_ptr().cast(),
            gl::STATIC_DRAW,
        );

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
    }

    while !w.window.should_close() {
        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            gl::UseProgram(shader_program);
            gl::BindVertexArray(vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
            gl::BindVertexArray(0);
        }

        w.window.swap_buffers();
        w.glfw.poll_events();
    }

    unsafe {
        gl::DeleteVertexArrays(1, &mut vao);
        gl::DeleteBuffers(1, &mut vbo);
        gl::DeleteProgram(shader_program);
    }
}

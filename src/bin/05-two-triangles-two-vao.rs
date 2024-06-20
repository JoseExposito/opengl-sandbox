use std::ffi::CString;

use gl;
use glfw::Context;
use opengl_sandbox::window::Window;

const VERTEX_SHADER_SRC: &str = "#version 330 core
layout (location = 0) in vec3 aPos;

void main() {
    gl_Position = vec4(aPos.xyz, 1.0);
}";

const FRAGMENT_SHADER_SRC_1: &str = "#version 330 core
out vec4 FragColor;

void main() {
    FragColor = vec4(1.0, 0.2, 0.2, 1.0);
}";

const FRAGMENT_SHADER_SRC_2: &str = "#version 330 core
out vec4 FragColor;

void main() {
    FragColor = vec4(0.9, 0.9, 0.9, 1.0);
}";

fn main() {
    let mut w = Window::new(800, 600, "2 triangles with 2 VAOs and VBOs");

    let vertex_shader1 = unsafe { gl::CreateShader(gl::VERTEX_SHADER) };
    let vertex_shader2 = unsafe { gl::CreateShader(gl::VERTEX_SHADER) };
    let fragment_shader1 = unsafe { gl::CreateShader(gl::FRAGMENT_SHADER) };
    let fragment_shader2 = unsafe { gl::CreateShader(gl::FRAGMENT_SHADER) };
    let program1 = unsafe { gl::CreateProgram() };
    let program2 = unsafe { gl::CreateProgram() };

    unsafe {
        let vertex_shader_src = CString::new(VERTEX_SHADER_SRC).unwrap();
        gl::ShaderSource(
            vertex_shader1,
            1,
            &vertex_shader_src.as_ptr(),
            std::ptr::null(),
        );
        gl::CompileShader(vertex_shader1);

        gl::ShaderSource(
            vertex_shader2,
            1,
            &vertex_shader_src.as_ptr(),
            std::ptr::null(),
        );
        gl::CompileShader(vertex_shader2);

        let fragment_shader1_src = CString::new(FRAGMENT_SHADER_SRC_1).unwrap();
        gl::ShaderSource(
            fragment_shader1,
            1,
            &fragment_shader1_src.as_ptr(),
            std::ptr::null(),
        );
        gl::CompileShader(fragment_shader1);

        let fragment_shader2_src = CString::new(FRAGMENT_SHADER_SRC_2).unwrap();
        gl::ShaderSource(
            fragment_shader2,
            1,
            &fragment_shader2_src.as_ptr(),
            std::ptr::null(),
        );
        gl::CompileShader(fragment_shader2);

        gl::AttachShader(program1, vertex_shader1);
        gl::AttachShader(program1, fragment_shader1);
        gl::LinkProgram(program1);

        gl::AttachShader(program2, vertex_shader2);
        gl::AttachShader(program2, fragment_shader2);
        gl::LinkProgram(program2);
    }

    let triangle1 = [
        -0.5f32, 0.5, 0.0, //
        0.5, 0.5, 0.0, //
        -0.5, -1.0, 0.0, //
    ];

    let triangle2 = [
        0.0f32, 0.3, 0.0, //
        0.3, -0.3, 0.0, //
        -0.3, -0.3, 0.0, //
    ];

    let indices = [0u32, 1, 2];

    let mut vao1 = 0;
    let mut vbo1 = 0;
    let mut ebo1 = 0;

    let mut vao2 = 0;
    let mut vbo2 = 0;
    let mut ebo2 = 0;

    unsafe {
        gl::GenVertexArrays(1, &mut vao1);
        gl::BindVertexArray(vao1);

        gl::GenBuffers(1, &mut vbo1);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo1);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            std::mem::size_of_val(&triangle1) as isize,
            triangle1.as_ptr().cast(),
            gl::STATIC_DRAW,
        );

        gl::GenBuffers(1, &mut ebo1);
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo1);
        gl::BufferData(
            gl::ELEMENT_ARRAY_BUFFER,
            std::mem::size_of_val(&indices) as isize,
            indices.as_ptr().cast(),
            gl::STATIC_DRAW,
        );

        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            3 * std::mem::size_of::<f32>() as i32,
            0 as *const _,
        );
        gl::EnableVertexAttribArray(0);

        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);
    }

    unsafe {
        gl::GenVertexArrays(1, &mut vao2);
        gl::BindVertexArray(vao2);

        gl::GenBuffers(1, &mut vbo2);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo2);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            std::mem::size_of_val(&triangle2) as isize,
            triangle2.as_ptr().cast(),
            gl::STATIC_DRAW,
        );

        gl::GenBuffers(1, &mut ebo2);
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo2);
        gl::BufferData(
            gl::ELEMENT_ARRAY_BUFFER,
            std::mem::size_of_val(&indices) as isize,
            indices.as_ptr().cast(),
            gl::STATIC_DRAW,
        );

        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            3 * std::mem::size_of::<f32>() as i32,
            0 as *const _,
        );
        gl::EnableVertexAttribArray(0);

        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);
    }

    while !w.window.should_close() {
        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            gl::UseProgram(program1);
            gl::BindVertexArray(vao1);
            gl::DrawElements(gl::TRIANGLES, 3, gl::UNSIGNED_INT, std::ptr::null());
            gl::BindVertexArray(0);
            gl::UseProgram(0);

            gl::UseProgram(program2);
            gl::BindVertexArray(vao2);
            gl::DrawElements(gl::TRIANGLES, 3, gl::UNSIGNED_INT, std::ptr::null());
            gl::BindVertexArray(0);
            gl::UseProgram(0);
        }

        w.window.swap_buffers();
        w.glfw.poll_events();
    }
}

use glfw::Context;
use opengl_sandbox::{
    program::Program,
    renderer::Renderer,
    shader::{Shader, ShaderType},
    vertex_array::VertexArray,
    vertex_buffer_layout::{VertexBufferLayout, VertexBufferLayoutType},
    window::Window,
};

const VERTEX_SHADER_SRC: &str = "#version 330 core
layout (location = 0) in vec3 aPos;
layout (location = 1) in vec3 aColor;
out vec3 vertexColor;

void main() {
    gl_Position = vec4(aPos.xyz, 1.0);
    vertexColor = aColor;
}";

const FRAGMENT_SHADER_SRC: &str = "#version 330 core
out vec4 FragColor;
in vec3 vertexColor;

void main() {
    FragColor = vec4(vertexColor, 1.0);
}";

fn main() {
    let renderer = Renderer::default();
    let mut w = Window::new(800, 600, "VAO with 2 layouts");

    let vertex_shader = Shader::new(ShaderType::VertexShader, VERTEX_SHADER_SRC);
    let fragment_shader = Shader::new(ShaderType::FragmentShader, FRAGMENT_SHADER_SRC);
    let program = Program::new(&vertex_shader, &fragment_shader);

    let vertices = [
        -0.5f32, -0.5, 0.0, // bottom left vertex
        1.0, 0.0, 0.0, // color
        0.5, -0.5, 0.0, // bottom right vertex
        0.0, 1.0, 0.0, // color
        0.0, 0.5, 0.0, // top vertex
        0.0, 0.0, 1.0, // color
    ];
    let indices = [0u32, 1, 2];

    let mut layouts = VertexBufferLayout::new(VertexBufferLayoutType::F32, 3, false);
    layouts.add(VertexBufferLayoutType::F32, 3, false);
    let vao = VertexArray::new(&vertices, &indices, &layouts);

    while !w.window.should_close() {
        renderer.clear();
        renderer.draw(&vao, &program);

        w.window.swap_buffers();
        w.glfw.poll_events();
    }
}

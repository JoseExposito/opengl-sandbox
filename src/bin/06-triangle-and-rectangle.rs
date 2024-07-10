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
    let mut w = Window::new(800, 600, "Hello Rectangle!");
    let renderer = Renderer::default();

    let vertex_shader = Shader::new(ShaderType::VertexShader, VERTEX_SHADER_SRC);
    let fragment_shader1 = Shader::new(ShaderType::FragmentShader, FRAGMENT_SHADER_SRC_1);
    let program1 = Program::new(&vertex_shader, &fragment_shader1);

    let fragment_shader2 = Shader::new(ShaderType::FragmentShader, FRAGMENT_SHADER_SRC_2);
    let program2 = Program::new(&vertex_shader, &fragment_shader2);

    let triangle = [
        -1.0f32, 1.0, 0.0, // top left
        1.0, 1.0, 0.0, // top right
        0.0, -1.0, 0.0, // bottom
    ];
    let triangle_indices = [0u32, 1, 2];

    let rectangle = [
        0.5f32, 0.5, 0.0, // top right
        0.5, -0.5, 0.0, // bottom right
        -0.5, -0.5, 0.0, // bottom left
        -0.5, 0.5, 0.0, // top left
    ];

    let rectangle_indices = [
        0u32, 1, 3, // First triangle
        1, 2, 3, // Second triangle
    ];

    let triangle_layouts = VertexBufferLayout::new(VertexBufferLayoutType::F32, 3, false);
    let vao1 = VertexArray::new(&triangle, &triangle_indices, &triangle_layouts);

    let rectangle_layouts = VertexBufferLayout::new(VertexBufferLayoutType::F32, 3, false);
    let vao2 = VertexArray::new(&rectangle, &rectangle_indices, &rectangle_layouts);

    while !w.window.should_close() {
        renderer.clear();
        renderer.draw(&vao1, &program1);
        renderer.draw(&vao2, &program2);

        w.window.swap_buffers();
        w.glfw.poll_events();
    }
}

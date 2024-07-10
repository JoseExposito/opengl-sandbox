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
layout (location = 1) in vec4 aColor;

out vec4 vColor;

void main() {
    gl_Position = vec4(aPos, 1.0);
    vColor = aColor;
}";

const FRAGMENT_SHADER_SRC: &str = "#version 330 core
out vec4 FragColor;

in vec4 vColor;

void main() {
    FragColor = vColor;
}";

fn main() {
    let mut w = Window::new(800, 600, "Blending");
    let renderer = Renderer::default();

    let vertex_shader = Shader::new(ShaderType::VertexShader, VERTEX_SHADER_SRC);
    let fragment_shader = Shader::new(ShaderType::FragmentShader, FRAGMENT_SHADER_SRC);
    let program = Program::new(&vertex_shader, &fragment_shader);

    #[rustfmt::skip]
    let rectangle1_vertices = [
        // positions      // color
        -1.0,  1.0, 0.0,  1.0, 0.0, 0.0, 1.0,    // top left
         0.5,  1.0, 0.0,  1.0, 0.0, 0.0, 1.0,    // top right
         0.5, -0.5, 0.0,  1.0, 0.0, 0.0, 1.0,    // bottom right
        -1.0, -0.5, 0.0,  1.0, 0.0, 0.0, 1.0f32, // bottom left
    ];

    #[rustfmt::skip]
    let rectangle1_indices = [
        0, 1, 3,    // First triangle
        1, 2, 3u32, // Second triangle
    ];

    #[rustfmt::skip]
    let rectangle2_vertices = [
        // positions      // color
        -0.5,  0.5, 0.0,  0.0, 0.0, 1.0, 0.6,    // top left
         1.0,  0.5, 0.0,  0.0, 0.0, 1.0, 0.6,    // top right
         1.0, -1.0, 0.0,  0.0, 0.0, 1.0, 0.6,    // bottom right
        -0.5, -1.0, 0.0,  0.0, 0.0, 1.0, 0.6f32, // bottom left
    ];

    #[rustfmt::skip]
    let rectangle2_indices = [
        0, 1, 3,    // First triangle
        1, 2, 3u32, // Second triangle
    ];

    let mut layouts = VertexBufferLayout::new(VertexBufferLayoutType::F32, 3, false);
    layouts.add(VertexBufferLayoutType::F32, 4, false);
    let vao1 = VertexArray::new(&rectangle1_vertices, &rectangle1_indices, &layouts);
    let vao2 = VertexArray::new(&rectangle2_vertices, &rectangle2_indices, &layouts);

    while !w.window.should_close() {
        renderer.clear();
        renderer.draw(&vao1, &program);
        renderer.draw(&vao2, &program);

        w.window.swap_buffers();
        w.glfw.poll_events();
    }
}

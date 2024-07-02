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

const FRAGMENT_SHADER_SRC: &str = "#version 330 core
out vec4 FragColor;
uniform vec4 u_GreenColor;

void main() {
    FragColor = u_GreenColor;
}";

fn main() {
    let renderer = Renderer::default();
    let mut w = Window::new(800, 600, "Hello Uniform!");

    let vertex_shader = Shader::new(ShaderType::VertexShader, VERTEX_SHADER_SRC);
    let fragment_shader = Shader::new(ShaderType::FragmentShader, FRAGMENT_SHADER_SRC);
    let program = Program::new(&vertex_shader, &fragment_shader);

    let vertices = [
        -0.5f32, -0.5, 0.0, //
        0.5, -0.5, 0.0, //
        0.0, 0.5, 0.0, //
    ];
    let indices = [0u32, 1, 2];

    let layouts = VertexBufferLayout::new(VertexBufferLayoutType::F32, 3, false);
    let vao = VertexArray::new(&vertices, &indices, &layouts);

    while !w.window.should_close() {
        let green_color = (f32::sin(w.get_time()) / 2.0) + 0.5;
        program.set_uniform_4f("u_GreenColor", 0.0, green_color, 0.0, 1.0);

        renderer.clear();
        renderer.draw(&vao, &program);

        w.window.swap_buffers();
        w.glfw.poll_events();
    }
}

use glfw::Context;
use opengl_sandbox::{
    program::Program,
    renderer::Renderer,
    shader::{Shader, ShaderType},
    texture::Texture,
    vertex_array::VertexArray,
    vertex_buffer_layout::{VertexBufferLayout, VertexBufferLayoutType},
    window::Window,
};

const VERTEX_SHADER_SRC: &str = "#version 330 core
layout (location = 0) in vec3 aPos;
layout (location = 1) in vec3 aColor;
layout (location = 2) in vec2 aTexCoord;

out vec3 vVertexColor;
out vec2 vTexCoord;

void main() {
    gl_Position = vec4(aPos, 1.0);
    vVertexColor = aColor;
    vTexCoord = aTexCoord;
}";

const FRAGMENT_SHADER_SRC: &str = "#version 330 core
out vec4 FragColor;

in vec3 vVertexColor;
in vec2 vTexCoord;

uniform sampler2D uTex;

void main() {
    FragColor = texture(uTex, vTexCoord) * vec4(vVertexColor, 1.0);
}";

fn main() {
    let renderer = Renderer::default();
    let mut w = Window::new(800, 600, "Hello texture!");

    let vertex_shader = Shader::new(ShaderType::VertexShader, VERTEX_SHADER_SRC);
    let fragment_shader = Shader::new(ShaderType::FragmentShader, FRAGMENT_SHADER_SRC);
    let mut program = Program::new(&vertex_shader, &fragment_shader);

    let texture = Texture::new("uTex", "res/textures/wall.jpg", 0);
    program.add_texture2d(texture);

    let vertices = [
        // positions          // colors           // texture coords
        0.0, 0.5, 0.0, /*  */ 1.0, 0.0, 0.0, /**/ 0.5, 1.0, //    top
        -0.5, -0.5, 0.0, /**/ 0.0, 1.0, 0.0, /**/ 0.0, 0.0, //    bottom left
        0.5, -0.5, 0.0, /* */ 0.0, 0.0, 1.0, /**/ 1.0, 0.0f32, // bottom right
    ];
    let indices = [0u32, 1, 2];

    let mut layouts = VertexBufferLayout::new(VertexBufferLayoutType::F32, 3, false);
    layouts.add(VertexBufferLayoutType::F32, 3, false);
    layouts.add(VertexBufferLayoutType::F32, 2, false);
    let vao = VertexArray::new(&vertices, &indices, &layouts);

    while !w.window.should_close() {
        renderer.clear();
        renderer.draw(&vao, &program);

        w.window.swap_buffers();
        w.glfw.poll_events();
    }
}

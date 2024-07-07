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
layout (location = 1) in vec2 aTexCoord;

out vec2 vTexCoord;

void main() {
    gl_Position = vec4(aPos, 1.0);
    vTexCoord = aTexCoord;
}";

const FRAGMENT_SHADER_SRC: &str = "#version 330 core
out vec4 FragColor;

in vec2 vTexCoord;

uniform sampler2D uTex1;
uniform sampler2D uTex2;

void main() {
    FragColor = mix(texture(uTex1, vTexCoord), texture(uTex2, vTexCoord), 0.5);
}";

fn main() {
    let renderer = Renderer::default();
    let mut w = Window::new(800, 600, "Two textures");

    let vertex_shader = Shader::new(ShaderType::VertexShader, VERTEX_SHADER_SRC);
    let fragment_shader = Shader::new(ShaderType::FragmentShader, FRAGMENT_SHADER_SRC);
    let mut program = Program::new(&vertex_shader, &fragment_shader);

    let wall_texture = Texture::new("uTex1", "res/textures/wall.jpg", 0);
    let ferris_texture = Texture::new("uTex2", "res/textures/ferris.png", 1);
    program.add_texture2d(wall_texture);
    program.add_texture2d(ferris_texture);

    // Note the the coords are inverted because the image is loaded starting at the top-left corner
    #[rustfmt::skip]
    let vertices = [
        // positions      // texture coords
        -0.5,  0.5, 0.0,  0.0, 0.0,    // top left
         0.5,  0.5, 0.0,  1.0, 0.0,    // top right
         0.5, -0.5, 0.0,  1.0, 1.0,    // bottom right
        -0.5, -0.5, 0.0,  0.0, 1.0f32, // bottom left
    ];

    let indices = [
        0, 1, 3, // First triangle
        1, 2, 3u32, // Second triangle
    ];

    let mut layouts = VertexBufferLayout::new(VertexBufferLayoutType::F32, 3, false);
    layouts.add(VertexBufferLayoutType::F32, 2, false);
    let vao = VertexArray::new(&vertices, &indices, &layouts);

    while !w.window.should_close() {
        renderer.clear();
        renderer.draw(&vao, &program);

        w.window.swap_buffers();
        w.glfw.poll_events();
    }
}

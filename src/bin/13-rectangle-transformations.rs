use glfw::Context;
use nalgebra_glm as glm;
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

uniform mat4 uTransform;

void main() {
    gl_Position = uTransform * vec4(aPos, 1.0);
    vTexCoord = aTexCoord;
}";

const FRAGMENT_SHADER_SRC: &str = "#version 330 core
out vec4 FragColor;

in vec2 vTexCoord;

uniform sampler2D uTex1;

void main() {
    FragColor = texture(uTex1, vTexCoord);
}";

fn main() {
    let mut w = Window::new(800, 600, "Rectangle transformations");
    let renderer = Renderer::default();

    let vertex_shader = Shader::new(ShaderType::VertexShader, VERTEX_SHADER_SRC);
    let fragment_shader = Shader::new(ShaderType::FragmentShader, FRAGMENT_SHADER_SRC);
    let mut program = Program::new(&vertex_shader, &fragment_shader);

    let ferris_texture = Texture::new("uTex1", "res/textures/ferris.png", 0);
    program.add_texture2d(ferris_texture);

    #[rustfmt::skip]
    let vertices = [
        // positions      // texture coords
        -0.5,  0.5, 0.0,  0.0, 1.0,    // top left
         0.5,  0.5, 0.0,  1.0, 1.0,    // top right
         0.5, -0.5, 0.0,  1.0, 0.0,    // bottom right
        -0.5, -0.5, 0.0,  0.0, 0.0f32, // bottom left
    ];

    let indices = [
        0, 1, 3, // First triangle
        1, 2, 3u32, // Second triangle
    ];

    let mut layouts = VertexBufferLayout::new(VertexBufferLayoutType::F32, 3, false);
    layouts.add(VertexBufferLayoutType::F32, 2, false);
    let vao = VertexArray::new(&vertices, &indices, &layouts);

    // Transformations are backwards. First we scale to half of the size and then rotate 90º around the Z axis
    let trans = &glm::Mat4::identity();
    let trans = glm::rotate(&trans, f32::to_radians(90.0), &glm::vec3(0.0, 0.0, 1.0));
    let trans = glm::scale(&trans, &glm::vec3(0.5, 0.5, 0.5));
    program.set_uniform_mat4("uTransform", &trans);

    while !w.window.should_close() {
        renderer.clear();
        renderer.draw(&vao, &program);

        w.window.swap_buffers();
        w.glfw.poll_events();
    }
}

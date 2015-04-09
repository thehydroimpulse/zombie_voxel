#![feature(plugin, custom_attribute)]
#![plugin(gfx_macros)]

extern crate glfw_window;
extern crate piston;
extern crate shader_version;
extern crate camera_controllers;
extern crate gfx;
extern crate gfx_device_gl;
extern crate vecmath;

use gfx_device_gl::Resources;
use glfw_window::GlfwWindow;
use piston::window::{Window, WindowSettings, Size, AdvancedWindow, OpenGLWindow};
use std::rc::Rc;
use std::cell::RefCell;
use gfx::traits::*;
use camera_controllers::{
    FirstPersonSettings,
    FirstPerson,
    CameraPerspective,
    model_view_projection
};

#[vertex_format]
#[derive(Copy, Clone)]
struct Vertex {
    #[as_float]
    a_pos: [i8; 3],
    #[as_float]
    a_tex_coord: [u8; 2]
}

impl Vertex {
    fn new(pos: [i8; 3], tc: [u8; 2]) -> Vertex {
        Vertex {
            a_pos: pos,
            a_tex_coord: tc
        }
    }
}

#[shader_param]
struct Params<R: gfx::Resources> {
    u_model_view_proj: [[f32; 4]; 4],
    t_color: gfx::shade::TextureParam<R>
}

const VERTEX_SRC: [&'static [u8]; 2] = [ b"
    #version 120
    attribute vec3 a_pos;
    attribute vec2 a_tex_coord;
    varying vec2 v_TexCoord;
    uniform mat4 u_model_view_proj;
    void main() {
        v_TexCoord = a_tex_coord;
        gl_Position = u_model_view_proj * vec4(a_pos, 1.0);
    }
", b"
    #version 150 core
    in vec3 a_pos;
    in vec2 a_tex_coord;
    out vec2 v_TexCoord;
    uniform mat4 u_model_view_proj;
    void main() {
        v_TexCoord = a_tex_coord;
        gl_Position = u_model_view_proj * vec4(a_pos, 1.0);
    }
"];

const FRAGMENT_SRC: [&'static [u8]; 2] = [ b"
    #version 120
    varying vec2 v_TexCoord;
    uniform sampler2D t_color;
    void main() {
        vec4 tex = texture2D(t_color, v_TexCoord);
        float blend = dot(v_TexCoord-vec2(0.5,0.5), v_TexCoord-vec2(0.5,0.5));
        gl_FragColor = mix(tex, vec4(0.0,0.0,0.0,0.0), blend*1.0);
    }
", b"
    #version 150 core
    in vec2 v_TexCoord;
    out vec4 o_Color;
    uniform sampler2D t_color;
    void main() {
        vec4 tex = texture(t_color, v_TexCoord);
        float blend = dot(v_TexCoord-vec2(0.5,0.5), v_TexCoord-vec2(0.5,0.5));
        o_Color = mix(tex, vec4(0.0,0.0,0.0,0.0), blend*1.0);
    }
"];

fn main() {
    let (w, h) = (1280, 720);
    let version = shader_version::OpenGL::_4_1;
    let size = Size { width: w, height: h };

    let settings = WindowSettings::new("Zombie".to_string(), size)
        .exit_on_esc(true);

    let mut window = GlfwWindow::new(version, settings);

    let vertex = gfx::ShaderSource {
        glsl_120: Some(VERTEX_SRC[0]),
        glsl_150: Some(VERTEX_SRC[1]),
        .. gfx::ShaderSource::empty()
    };

    let fragment = gfx::ShaderSource {
        glsl_120: Some(FRAGMENT_SRC[0]),
        glsl_150: Some(FRAGMENT_SRC[1]),
        .. gfx::ShaderSource::empty()
    };

    let (device, mut factory) = gfx_device_gl::create(|s| window.get_proc_address(s));
    let size = window.size();
    let frame = gfx::Frame::<Resources>::new(size.width as u16, size.height as u16);
    let state = gfx::DrawState::new().depth(gfx::state::Comparison::LessEqual, true);

    let vertex_data = vec![
        //top (0, 0, 1)
        Vertex::new([-1, -1,  1], [0, 0]),
        Vertex::new([ 1, -1,  1], [1, 0]),
        Vertex::new([ 1,  1,  1], [1, 1]),
        Vertex::new([-1,  1,  1], [0, 1]),
        //bottom (0, 0, -1)
        Vertex::new([ 1,  1, -1], [0, 0]),
        Vertex::new([-1,  1, -1], [1, 0]),
        Vertex::new([-1, -1, -1], [1, 1]),
        Vertex::new([ 1, -1, -1], [0, 1]),
        //right (1, 0, 0)
        Vertex::new([ 1, -1, -1], [0, 0]),
        Vertex::new([ 1,  1, -1], [1, 0]),
        Vertex::new([ 1,  1,  1], [1, 1]),
        Vertex::new([ 1, -1,  1], [0, 1]),
        //left (-1, 0, 0)
        Vertex::new([-1,  1,  1], [0, 0]),
        Vertex::new([-1, -1,  1], [1, 0]),
        Vertex::new([-1, -1, -1], [1, 1]),
        Vertex::new([-1,  1, -1], [0, 1]),
        //front (0, 1, 0)
        Vertex::new([-1,  1, -1], [0, 0]),
        Vertex::new([ 1,  1, -1], [1, 0]),
        Vertex::new([ 1,  1,  1], [1, 1]),
        Vertex::new([-1,  1,  1], [0, 1]),
        //back (0, -1, 0)
        Vertex::new([ 1, -1,  1], [0, 0]),
        Vertex::new([-1, -1,  1], [1, 0]),
        Vertex::new([-1, -1, -1], [1, 1]),
        Vertex::new([ 1, -1, -1], [0, 1]),
    ];

    let mesh = factory.create_mesh(&vertex_data);

    let index_data: &[u8] = &[
         0,  1,  2,  2,  3,  0, // top
         4,  6,  5,  6,  4,  7, // bottom
         8,  9, 10, 10, 11,  8, // right
        12, 14, 13, 14, 12, 16, // left
        16, 18, 17, 18, 16, 19, // front
        20, 21, 22, 22, 23, 20, // back
    ];

    let slice = factory.create_buffer_index(index_data)
                      .to_slice(gfx::PrimitiveType::TriangleList);

    let tinfo = gfx::tex::TextureInfo {
        width: 1,
        height: 1,
        depth: 1,
        levels: 1,
        kind: gfx::tex::TextureKind::Texture2D,
        format: gfx::tex::RGBA8,
    };
    let img_info = tinfo.to_image_info();
    let texture = factory.create_texture(tinfo).unwrap();
    factory.update_texture(
        &texture,
        &img_info,
        &[0x20u8, 0xA0, 0xC0, 0x00],
        Some(gfx::tex::TextureKind::Texture2D)
    ).unwrap();

    let sampler = factory.create_sampler(
        gfx::tex::SamplerInfo::new(
            gfx::tex::FilterMethod::Bilinear,
            gfx::tex::WrapMode::Clamp
        )
    );

    let shader_model = device.get_capabilities().shader_model;

    let program = factory.link_program(
        vertex.choose(shader_model).unwrap(),
        fragment.choose(shader_model).unwrap()
    ).unwrap();

    let mut graphics = (device, factory).into_graphics();

    let data = Params {
        u_model_view_proj: vecmath::mat4_id(),
        t_color: (texture, Some(sampler)),
    };

    let mut batch = graphics.make_batch(&program, data, &mesh, slice, &state).unwrap();

    let model = vecmath::mat4_id();
    let projection = CameraPerspective {
        fov: 90.0,
        near_clip: 0.1,
        far_clip: 1000.0,
        aspect_ratio: 1.0
    }.projection();
    let mut first_person = FirstPerson::new(
        [0.10, 0.5, 4.0],
        FirstPersonSettings::keyboard_wasd()
    );

    let window = Rc::new(RefCell::new(window));

    for e in piston::events(window) {
        use piston::event::*;

        first_person.event(&e);

        if let Some(args) = e.render_args() {
            graphics.clear(
                gfx::ClearData {
                    color: [0.9, 0.5, 0.3, 1.0],
                    depth: 1.0,
                    stencil: 0,
                },
                gfx::COLOR | gfx::DEPTH,
                &frame
            );
            batch.params.u_model_view_proj = model_view_projection(
                model,
                first_person.camera(args.ext_dt).orthogonal(),
                projection
            );
            graphics.draw(&batch, &frame).unwrap();
            graphics.end_frame();
        }

        if let Some(_) = e.after_render_args() {
            graphics.cleanup();
        }
    }
}

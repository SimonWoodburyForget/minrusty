use crate::components::*;
use crate::resources::*;

use std::mem;

use glow::*;
use sdl2::{event::Event, video::Window, EventPump, Sdl};
use specs::prelude::*;

// TODO: figure out whether implement or don't implement this.
// /// Window system for SDL window... processing.
// pub struct WindowSystem;
// impl WindowSystem {
//     /// init sdl, create window and `insert` it in world
//     pub fn new(world: &mut World) -> Self {
//         // init sdl?
//         let sdl = sdl2::init().unwrap();
//         let video = sdl.video().unwrap();
//         let gl_attr = video.gl_attr();
//         gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
//         gl_attr.set_context_version(3, 0);
//         // init window?
//         let window = video
//             .window("Hello Triangele!", 1024, 769)
//             .opengl()
//             .resizable()
//             .build()
//             .unwrap();
//         world.insert(window); // ?
//         Self
//     }
// }

/// Simple shader program.
pub struct Program {
    /// Shader Program
    pg: u32,
}

impl Program {
    /// Create simple shader program, out of vertex and fragment glsl source.
    pub fn new(gl: &Context, v_source: &str, f_source: &str) -> Result<Self, String> {
        let pg = unsafe { gl.create_program() }?;
        let shader_sources = [
            (glow::VERTEX_SHADER, v_source),
            (glow::FRAGMENT_SHADER, f_source),
        ];

        let shaders: Vec<_> = shader_sources
            .iter()
            .map(|(shader_type, shader_source)| unsafe {
                let shader = gl
                    .create_shader(*shader_type)
                    .expect("Cannot create shader");
                gl.shader_source(shader, shader_source);
                gl.compile_shader(shader);
                if !gl.get_shader_compile_status(shader) {
                    panic!(gl.get_shader_info_log(shader));
                }
                gl.attach_shader(pg, shader);
                shader
            })
            .collect();

        unsafe {
            gl.link_program(pg);
            if !gl.get_program_link_status(pg) {
                return Err(gl.get_program_info_log(pg));
            }

            for shader in shaders {
                gl.detach_shader(pg, shader);
                gl.delete_shader(shader);
            }

            gl.use_program(Some(pg));
            gl.clear_color(0.1, 0.2, 0.3, 1.0);
        }

        Ok(Program { pg })
    }

    pub unsafe fn use_program(&self, gl: &Context) {
        gl.use_program(Some(self.pg));
    }

    /// TODO
    pub unsafe fn delete(&self, gl: &Context) {
        unimplemented!();
    }
}

pub struct VertexArray {
    vao: u32,
    vbo: u32,
    ebo: u32,
}

impl VertexArray {
    /// Initializes vertex and index buffers from an OpenGL context.
    pub fn new(gl: &Context, vertices: &[f32], indices: &[u32]) -> Result<Self, String> {
        let Self { vao, vbo, ebo } = unsafe { Self::create(&gl) }?;
        unsafe {
            gl.bind_vertex_array(Some(vao));

            gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));
            gl.buffer_data_u8_slice(
                glow::ARRAY_BUFFER,
                &vertices.align_to::<u8>().1,
                glow::STATIC_DRAW,
            );

            gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(ebo));
            gl.buffer_data_u8_slice(
                glow::ELEMENT_ARRAY_BUFFER,
                &indices.align_to::<u8>().1,
                glow::STATIC_DRAW,
            );

            let stride = 3 * mem::size_of::<f32>() as i32;
            gl.vertex_attrib_pointer_f32(0, 3, glow::FLOAT, false, stride, 0);
            gl.enable_vertex_attrib_array(0);

            // gl.bind_buffer(glow::ARRAY_BUFFER, None);
            gl.bind_vertex_array(None);
        };

        Ok(Self { vao, vbo, ebo })
    }

    pub unsafe fn create(gl: &Context) -> Result<Self, String> {
        Ok(Self {
            vao: gl.create_vertex_array()?,
            vbo: gl.create_buffer()?,
            ebo: gl.create_buffer()?,
        })
    }

    pub unsafe fn bind(&self, gl: &Context) {
        gl.bind_vertex_array(Some(self.vao));
        // gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(self.ebo));
        // gl.bind_buffer(glow::ARRAY_BUFFER, Some(self.vbo));
    }

    pub unsafe fn delete(&self, gl: &Context) {
        gl.delete_vertex_array(self.vao);
        gl.delete_buffer(self.vbo);
        gl.delete_buffer(self.ebo);
    }
}

/// A ..Square renderer
pub struct Square {
    va: VertexArray,
    pg: Program,
}

impl Square {
    pub fn new(gl: &Context) -> Result<Self, String> {
        #[rustfmt::skip]
        let vertices: [f32; 12] = [
             0.5,  0.5, 0.0,  // top right
             0.5, -0.5, 0.0,  // bottom right
            -0.5,  0.5, 0.0,  // top left
            -0.5, -0.5, 0.0,  // bottom left
        ];

        #[rustfmt::skip]
        let indices: [u32; 6] = [
            0, 1, 2, // top right triangle
            2, 3, 1, // buttom left triangle
        ];

        Ok(Self {
            va: VertexArray::new(&gl, &vertices, &indices)?,
            pg: Program::new(
                &gl,
                include_str!("shaders/vss.glsl"),
                include_str!("shaders/fss.glsl"),
            )?,
        })
    }

    pub fn draw(&self, gl: &Context) {
        let Self { va, pg } = self;
        unsafe {
            gl.clear(glow::COLOR_BUFFER_BIT);

            pg.use_program(&gl);
            va.bind(&gl);
            // gl.draw_arrays(glow::TRIANGLES, 0, 6);
            gl.draw_elements(glow::TRIANGLES, 6, glow::UNSIGNED_INT, 0);
            gl.bind_vertex_array(None);
        }
    }
}

/// Render system for OpenGL graphics processing.
pub struct RenderSystem {
    gl: Context,
    square: Square,
}

impl RenderSystem {
    pub fn new(gl: Context, shader_version: &str) -> Result<Self, String> {
        Ok(Self {
            square: Square::new(&gl)?,
            gl: gl,
        })
    }
}

impl<'a> System<'a> for RenderSystem {
    type SystemData = ();

    fn run(&mut self, (): Self::SystemData) {
        let RenderSystem { gl, square } = self;
        square.draw(&gl);
    }
}

/// Event system for simple SDL event processing.
pub struct EventSystem(pub EventPump);
impl<'a> System<'a> for EventSystem {
    type SystemData = (Write<'a, Quit>);

    fn run(&mut self, (mut quit): Self::SystemData) {
        for event in self.0.poll_iter() {
            match event {
                Event::Quit { .. } => *quit = Quit(true),
                _ => (),
            }
        }
        // event_handler.drain_vec_write(events);
    }
}

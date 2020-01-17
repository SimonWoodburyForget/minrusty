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
}

pub struct VertexArray {
    vao: u32,
}

impl VertexArray {
    pub fn new(gl: &Context) -> Result<Self, String> {
        #[rustfmt::skip]
        let triangle: [f32; 9] = [
            -0.5, -0.5, 0.0,
             0.5, -0.5, 0.0,
             0.0,  0.5, 0.0
        ];

        unsafe {
            let vao = gl.create_vertex_array()?;
            let vbo = gl.create_buffer()?;

            gl.bind_vertex_array(Some(vao));
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));

            gl.buffer_data_u8_slice(
                glow::ARRAY_BUFFER,
                &triangle.align_to::<u8>().1,
                glow::STATIC_DRAW,
            );

            gl.enable_vertex_attrib_array(0);
            let stride = 3 * mem::size_of::<f32>() as i32;
            gl.vertex_attrib_pointer_f32(0, 3, glow::FLOAT, false, stride, 0);

            Ok(VertexArray { vao })
        }
    }

    pub unsafe fn bind_vertex_array(&self, gl: &Context) {
        gl.bind_vertex_array(Some(self.vao));
    }
}

/// Render system for OpenGL graphics processing.
pub struct RenderSystem {
    gl: Context,
    va: VertexArray,
    pg: Program,
}

impl RenderSystem {
    pub fn new(gl: Context, shader_version: &str) -> Result<Self, String> {
        let vertex_array = unsafe {
            let va = gl.create_vertex_array()?;
            gl.bind_vertex_array(Some(va));
            va
        };

        let vertex_array = VertexArray::new(&gl)?;
        let program = Program::new(
            &gl,
            include_str!("shaders/vss.glsl"),
            include_str!("shaders/fss.glsl"),
        )?;

        Ok(Self {
            gl: gl,
            va: vertex_array,
            pg: program,
        })
    }
}

impl<'a> System<'a> for RenderSystem {
    type SystemData = ();

    fn run(&mut self, (): Self::SystemData) {
        let RenderSystem { gl, pg, va } = self;
        unsafe {
            gl.clear(glow::COLOR_BUFFER_BIT);

            pg.use_program(&gl);
            va.bind_vertex_array(&gl);
            gl.draw_arrays(glow::TRIANGLES, 0, 3);
        }
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

use crate::components::*;
use crate::resources::*;
use glow::*;

use specs::{
    prelude::*,
    // shrev::EventChannel
};

use sdl2::{
    Sdl,
    EventPump,
    event::Event,
    video::Window,
};

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

/// Render system for OpenGL graphics processing.
pub struct RenderSystem {
    gl: Context,
    va: u32,
    pg: u32,
}

impl RenderSystem {
    /// init gl shaders/programs/arrays/... 
    pub fn new(gl: Context, shader_version: &str) -> Self {
        let (vertex_array, program) = unsafe {
            let va = gl.create_vertex_array().expect("Cannot create vertex array");
            gl.bind_vertex_array(Some(va));
            let pg = gl.create_program().expect("Cannot create program");
            (va, pg)
        };

        let shader_sources = [
            (glow::VERTEX_SHADER, include_str!("shaders/vss.glsl")),
            (glow::FRAGMENT_SHADER, include_str!("shaders/fss.glsl")),
        ];
        
        let mut shaders = Vec::with_capacity(shader_sources.len());
        
        for (shader_type, shader_source) in shader_sources.iter() {
            unsafe {
                let shader = gl.create_shader(*shader_type).expect("Cannot create shader");
                gl.shader_source(shader, &format!("{}\n{}", shader_version, shader_source));
                gl.compile_shader(shader);
                if !gl.get_shader_compile_status(shader) {
                    println!("{}", gl.get_shader_info_log(shader));
                    panic!(gl.get_shader_info_log(shader));
                }
                gl.attach_shader(program, shader);
                shaders.push(shader);
            }
        }

        unsafe {
            gl.link_program(program);
            if !gl.get_program_link_status(program) {
                panic!(gl.get_program_info_log(program));
            }
            
            for shader in shaders {
                gl.detach_shader(program, shader);
                gl.delete_shader(shader);
            }
            
            gl.use_program(Some(program));
            gl.clear_color(0.1, 0.2, 0.3, 1.0);
        }

        Self {
            gl: gl,
            va: vertex_array,
            pg: program,
        }
    }
}

impl<'a> System<'a> for RenderSystem {
    type SystemData = ();

    fn run(&mut self, (): Self::SystemData) {
        let RenderSystem { gl, pg, va } = self;
        unsafe {
            gl.clear(glow::COLOR_BUFFER_BIT);
            gl.draw_arrays(glow::TRIANGLES, 0, 3);
        }
    }
}

/// Event system for simple SDL event processing.
pub struct EventSystem(pub EventPump);
impl<'a> System<'a> for EventSystem {
    type SystemData = (
        // Write<'a, EventChannel<Event>>
        Write<'a, Quit>
    );
    
    fn run(&mut self, (mut quit): Self::SystemData) {
        for event in self.0.poll_iter() {
            match event {
                Event::Quit { .. } => *quit = Quit(true),
                _ => ()
            }
        }
        // event_handler.drain_vec_write(events);
    }
}


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
};

pub struct RenderSystem {
    gl: Context,
}

impl RenderSystem {
    pub fn new(gl: Context) -> Self {
        Self {
            gl: gl
        }
    }
}

impl<'a> System<'a> for RenderSystem {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Size>,
        Read<'a, Quit>
    );

    fn run(&mut self, (_, _, _, quit): Self::SystemData) {
        // .. TODO

        // unsafe {
        //     gl.clear(glow::COLOR_BUFFER_BIT);
        //     gl.draw_arrays(glow::TRIANGLES, 0, 3);
        
        //     if let Quit(true) = quit {
        //         gl.delete_program(program);
        //         gl.delete_vertex_array(vertex_array);
        //     }
        // }
    }
}

pub struct EventSystem(EventPump);
impl EventSystem {
    pub fn new(sdl: &Sdl) -> Self {
        Self(sdl.event_pump().unwrap())
    }
}

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


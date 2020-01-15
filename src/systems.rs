use crate::components::*;
// use crate::resources::*;

use specs::{
    prelude::*,
    shrev::EventChannel
};

use sdl2::{
    Sdl,
    EventPump,
    event::Event,
};

pub struct RenderSystem;
impl<'a> System<'a> for RenderSystem {
    type SystemData = (Entities<'a>, ReadStorage<'a, Position>, ReadStorage<'a, Size>);
    fn run(&mut self, data: Self::SystemData) {
        // .. TODO
    }
}

pub struct EventSystem(EventPump);
impl EventSystem {
    pub fn new(sdl: Sdl) -> Self {
        Self(sdl.event_pump().unwrap())
    }
}

impl<'a> System<'a> for EventSystem {
    type SystemData = (
        // Write<'a, EventChannel<Event>>
        // Write<'a, GameState>
    );
    
    fn run(&mut self, data: Self::SystemData) {
        for event in self.0.poll_iter() {
            match event {
                Event::Quit { .. } => (),
                _ => ()
            }
        }
        // event_handler.drain_vec_write(events);
    }
}


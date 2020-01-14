use crate::components::*;

use specs::prelude::*;
use glutin::window::Window;

pub struct RenderSystem;
impl<'a> System<'a> for RenderSystem {
    type SystemData = (Entities<'a>, ReadStorage<'a, Position>, ReadStorage<'a, Size>);
    fn run(&mut self, data: Self::SystemData) {
        // .. TODO
    }
}

#[derive(Default)]
pub struct EventSystem {
    reader: Option<ReaderId<Event>>,
};

impl<'a> System<'a> for EventSystem {
    type SystemData = (Read<'a, EventChannel>);
    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);
        self.reader = Some(
            world.fetch_mut::<EventChannel<Event>>().register_reader()
        );
    }
    
    fn run(&mut self, data: Self::SystemData) {
        // .. TODO
    }
}


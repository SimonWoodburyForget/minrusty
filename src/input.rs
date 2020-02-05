use crate::components::*;

use shrev::*;
use specs::prelude::*;
use winit::event::KeyboardInput;

enum Event {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Default)]
pub struct InputSystem(pub Option<ReaderId<KeyboardInput>>);

impl<'a> System<'a> for InputSystem {
    type SystemData = Read<'a, EventChannel<KeyboardInput>>;

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);
        self.0 = Some(
            world
                .fetch_mut::<EventChannel<KeyboardInput>>()
                .register_reader(),
        );
    }

    fn run(&mut self, channel: Self::SystemData) {
        for event in channel.read(&mut self.0.as_mut().unwrap()) {
            crate::log(&format!("{:?}", event));
        }
    }
}

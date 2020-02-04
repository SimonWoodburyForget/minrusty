use winit::event::KeyboardInput;

use shrev::*;
use specs::prelude::*;

enum Event {}

#[derive(Default)]
pub struct InputSystem(pub Option<ReaderId<KeyboardInput>>);

impl<'a> System<'a> for InputSystem {
    type SystemData = Read<'a, EventChannel<KeyboardInput>>;

    fn run(&mut self, channel: Self::SystemData) {
        for event in channel.read(&mut self.0.as_mut().unwrap()) {
            crate::log(&format!("{:?}", event));
        }
    }

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);
        self.0 = Some(
            world
                .fetch_mut::<EventChannel<KeyboardInput>>()
                .register_reader(),
        );
    }
}

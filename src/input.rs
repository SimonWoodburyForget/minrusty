use shrev::*;
use specs::prelude::*;
use vek::*;

use winit::event::{ElementState as ES, KeyboardInput, VirtualKeyCode as VKC};

/// Represents the direction the player wants to go.
#[derive(Default, Debug, Clone, Copy)]
pub struct InputState {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
}

impl From<InputState> for Vec2<f32> {
    /// Converts InputState into a unit vector, (Vec2 with a lenght of 1) if a button
    /// is pressed otherwise it returns zero vector; which can be used as velocity.
    fn from(state: InputState) -> Self {
        #[rustfmt::skip]
        let InputState { up, down, left, right } = state;
        let to_float = |x| if x { 1.0 } else { 0.0 };

        Vec2::new(
            to_float(up) - to_float(down),
            to_float(left) - to_float(right),
        )
        .try_normalized()
        .unwrap_or(Vec2::zero())
    }
}

#[derive(Default)]
pub struct InputSystem(pub Option<ReaderId<KeyboardInput>>);

impl<'a> System<'a> for InputSystem {
    type SystemData = (Read<'a, EventChannel<KeyboardInput>>, Write<'a, InputState>);

    fn run(&mut self, (channel, mut state): Self::SystemData) {
        for event in channel.read(&mut self.0.as_mut().unwrap()) {
            let p = event.state == ES::Pressed;
            if let Some(key) = event.virtual_keycode {
                match key {
                    VKC::Up => state.up = p,
                    VKC::Down => state.down = p,
                    VKC::Left => state.left = p,
                    VKC::Right => state.right = p,
                    _ => (),
                }
            }
        }

        crate::log(&format!("{:?}", Vec2::<f32>::from(*state)));
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
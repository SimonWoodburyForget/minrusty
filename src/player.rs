use crate::components::*;
use crate::input::*;

use specs::prelude::*;
use vek::*;

pub struct PlayerSystem;

impl<'a> System<'a> for PlayerSystem {
    type SystemData = (
        WriteStorage<'a, Force>,
        ReadStorage<'a, Control>,
        Read<'a, InputState>,
        // ..
    );

    fn run(&mut self, (mut forces, control, input): Self::SystemData) {
        let input_vec = Vec2::from(*input) * 0.1;
        for (f, _) in (&mut forces, &control).join() {
            f.0 += input_vec;
        }
    }
}

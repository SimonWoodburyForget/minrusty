use crate::components::*;
use crate::input::*;

use specs::prelude::*;
use vek::*;

pub struct PlayerSystem;

impl<'a> System<'a> for PlayerSystem {
    type SystemData = (
        WriteStorage<'a, Velocity>,
        ReadStorage<'a, Control>,
        Read<'a, InputState>,
        // ..
    );

    fn run(&mut self, (mut velocities, control, input): Self::SystemData) {
        let input_vec = Vec2::from(*input) * 0.1;
        for (vel, _) in (&mut velocities, &control).join() {
            vel.0 += input_vec;
        }
    }
}

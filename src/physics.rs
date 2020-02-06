use crate::components::*;
use crate::state::*;
use crate::units::*;

use specs::prelude::*;

pub struct PhysicSystem;

impl<'a> System<'a> for PhysicSystem {
    type SystemData = (
        WriteStorage<'a, Position>,
        ReadStorage<'a, Velocity>,
        Read<'a, DeltaTime>,
        // ..
    );

    fn run(&mut self, (mut positions, velocities, dt): Self::SystemData) {
        let seconds: f32 = Seconds::from(dt.0).0;
        for (pos, vel) in (&mut positions, &velocities).join() {
            pos.0 += vel.0 * seconds;
        }
    }
}

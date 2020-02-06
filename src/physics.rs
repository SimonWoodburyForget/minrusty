use crate::components::*;
use crate::state::*;
use crate::units::*;

use specs::prelude::*;
use vek::*;

pub struct PhysicSystem;

impl<'a> System<'a> for PhysicSystem {
    type SystemData = (
        WriteStorage<'a, Position>,
        WriteStorage<'a, Velocity>,
        ReadStorage<'a, Force>,
        Read<'a, DeltaTime>,
    );

    fn run(&mut self, (mut positions, mut velocities, forces, dt): Self::SystemData) {
        let seconds: f32 = Seconds::from(dt.0).0;
        for (pos, vel, force) in (&mut positions, &mut velocities, &forces).join() {
            // FIXME: at the moment, momentum is pretty laggy, no idea why.
            pos.0 += vel.0 * seconds;
            vel.0 *= 0.1;
            vel.0 += force.0 * 50.0 * seconds;

            // if v.0.reduce_partial_min() < 0.001 {
            //     v.0 = Vec2::zero();
            // }
        }
    }
}

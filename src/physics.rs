use crate::components::*;
use crate::state::*;

use specs::prelude::*;

pub struct PhysicSystem;

impl<'a> System<'a> for PhysicSystem {
    type SystemData = (
        WriteStorage<'a, Position>,
        WriteStorage<'a, Velocity>,
        ReadStorage<'a, Force>,
        ReadStorage<'a, Coordinate>,
        Read<'a, DeltaTime>,
        // WriteStorage<'a, Items>,
    );

    fn run(&mut self, _: Self::SystemData) {}
}

use specs::prelude::*;

use crate::components::*;
use crate::game::resources::*;
use crate::map::{Cell, Map};

struct BuilderSystem;

impl<'a> System<'a> for BuilderSystem {
    type SystemData = (
        // Read<'a, T>
        Read<'a, CursorState>,
        Read<'a, ScreenSize>,
        Read<'a, UniversePosition>,
        Read<'a, Map<Cell>>,
        ReadStorage<'a, Color>,
    );

    fn run(&mut self, _: Self::SystemData) {}
}

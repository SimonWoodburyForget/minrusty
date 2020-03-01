use specs::prelude::*;
use vek::*;

use crate::components::*;
use crate::game::{resources::Frame, Scene};
use crate::map::{Cell, Map};

#[derive(Default)]
pub struct BuilderSystem;

impl<'a> System<'a> for BuilderSystem {
    type SystemData = (
        // ..
        Entities<'a>,
        Read<'a, Frame>,
        Read<'a, Map<Cell>>,
        Read<'a, Scene>,
        WriteStorage<'a, Color>,
    );

    fn run(
        &mut self,
        (
            _entities,
            _frame,
            map,
            scene,
            mut colors,
            // ..
        ): Self::SystemData,
    ) {
        let cursor = scene.coordinate_cursor();
        if let Some(cell) = map.get(cursor) {
            if let Some(entity) = cell.tile {
                if let Some(ref mut color) = colors.get_mut(entity) {
                    color.0 = Rgba::new(0.0, 0.0, 0.0, 0.0);
                    // TODO: drop block here?
                }
            }
        }
    }
}

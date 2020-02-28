use specs::prelude::*;
use vek::*;

use crate::components::*;
use crate::game::resources::*;
use crate::map::{Cell, Map};

#[derive(Default)]
pub struct BuilderSystem;

impl<'a> System<'a> for BuilderSystem {
    type SystemData = (
        // ..
        Entities<'a>,
        Read<'a, Frame>,
        Read<'a, CursorState>,
        Read<'a, ScreenSize>,
        Read<'a, UniversePosition>,
        Read<'a, Map<Cell>>,
        WriteStorage<'a, Color>,
    );

    fn run(
        &mut self,
        (
            _entities,
            _frame,
            _cursor_state,
            _screen_size,
            _universe_position,
            _map,
            mut _colors,
            // ..
        ): Self::SystemData,
    ) {
        // if let Some(cell) = map.get(Vec2::new(xr, yr)) {
        //     if let Some(entity) = cell.tile {
        //         if let Some(ref mut color) = colors.get_mut(entity) {
        //             color.0 = Rgba::new(0.0, 0.0, 0.0, 0.0);
        //         }
        //     }
        // }

        // get(entity);
    }
}

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
            cursor_state,
            screen_size,
            _universe_position,
            map,
            mut colors,
            // ..
        ): Self::SystemData,
    ) {
        let [h, w] = screen_size.0.into_array();
        let hw = Vec2::new(h as f32, w as f32);

        if let Some(entity) = map[Vec2::new(1, 1)].tile {
            if let Some(ref mut color) = colors.get_mut(entity) {
                color.0 = Rgba::new(0.0, 0.0, 0.0, 0.0);
            }
        }

        // get(entity);
    }
}

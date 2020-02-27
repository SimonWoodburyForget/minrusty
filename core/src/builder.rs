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
            frame,
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

        let [x, y] = cursor_state.0.into_array();
        let xy = Vec2::new(x as f32, y as f32);

        let normal_space_cursor = ((xy / hw) + Vec2::new(-0.5, -0.5)) * 2.0;

        if frame.0 % 100 == 1 {
            let [x, y] = normal_space_cursor.into_array();
            println!("curs {:>6} ({:>11}, {:>11})", frame.0, x, y);
        }

        if let Some(entity) = map[Vec2::new(1, 1)].tile {
            if let Some(ref mut color) = colors.get_mut(entity) {
                color.0 = Rgba::new(0.0, 0.0, 0.0, 0.0);
            }
        }

        // get(entity);
    }
}

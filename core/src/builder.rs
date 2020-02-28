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
            universe_position,
            map,
            mut colors,
            // ..
        ): Self::SystemData,
    ) {
        fn normalize(screen_size: Vec2<f32>, cursor_position: Vec2<f32>) -> Vec2<f32> {
            ((cursor_position / screen_size) - Vec2::new(0.5, 0.5)) * 2.0
        }

        fn convert(mut vector: Vec2<i32>) -> Vec2<f32> {
            vector.y = -vector.y;
            vector.numcast().unwrap()
        }

        let hw = convert(screen_size.0);
        let xy = convert(cursor_state.0);

        // let normed = normalize(hw, xy);
        // debug_assert!(normed.partial_cmple(&Vec2::new(1., 1.)).reduce_bitand());
        // debug_assert!(normed.partial_cmpge(&Vec2::new(-1., -1.)).reduce_bitand());

        // let scale: Mat4<f32> = Mat4::scaling_3d(Vec3::new(100., 100., 1.));
        // let frustum = Mat4::frustum_lh_no(FrustumPlanes {
        //     left: hw.x,
        //     right: 0.0,
        //     bottom: hw.y,
        //     top: 0.0,
        //     near: -10.,
        //     far: 10.,
        // });
        // dbg!(frustum * scale * Vec4::new(normed.x, normed.y, 0.0, 1.0));

        // let [x, y] = normal_space_cursor.into_array();
        // let y = -y;

        // // let v = Vec2::new(0., 0.);
        // // v.translate_2d(Vec2::new(x, y));
        // let scale: Mat4<f32> = Mat4::scaling_3d(Vec3::new(100., 100., 1.0));
        // #[rustfmt::skip]
        // let frustum = {
        //     FrustumPlanes::<f32> {
        //         left: hw.x, right: 0.,
        //         bottom: hw.y, top: 0.,
        //         near: -10., far: 10.,
        //     }
        // };
        // // let [ox, oy] = universe_position.0.into_array();
        // // #[rustfmt::skip]
        // // let movit = Mat4::new(
        // //     1.0, 0.0, 0.0,   x,
        // //     0.0, 1.0, 0.0,   y,
        // //     0.0, 0.0, 1.0, 0.0,
        // //     0.0, 0.0, 0.0, 1.0,
        // // );
        // let transform = Mat4::frustum_rh_no(frustum) * scale;

        // // movit *
        // // ortho *

        // let coordinate = (transform).inverted() * Vec4::new(x, y, 0.0, 1.0);

        // let [x, y, _, _] = coordinate.into_array();
        // // let v = Vec2::new(x, y) - Vec2::new(1., 1.);
        // // let [x, y] = v.into_array();
        // let (xr, yr) = (x.round() as _, y.round() as _);

        // if frame.0 % 10 == 0 {
        //     println!(
        //         "curs {:>6} ({:>11}, {:>11}); round ({:>3}, {:>3}); floor ({:>3}, {:>3})",
        //         frame.0,
        //         x,
        //         y,
        //         x.round(),
        //         y.round(),
        //         x.floor(),
        //         y.floor(),
        //     );
        // }

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

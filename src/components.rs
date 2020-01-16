use specs::{Component, VecStorage};
use vek::Vec3;

pub struct Position(Vec3<f32>);
impl Component for Position {
    type Storage = VecStorage<Self>;
}

pub struct Size(f32);
impl Component for Size {
    type Storage = VecStorage<Self>;
}

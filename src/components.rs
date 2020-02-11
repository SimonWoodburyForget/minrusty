use specs::prelude::*;
use specs::Component;
use vek::*;

#[derive(Component, Clone, Debug)]
pub struct Tile;

#[derive(Component, Clone, Debug)]
pub struct Camera;

/// Marks entity to be affected by player inputs.
#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct Control;

#[derive(Component)]
pub struct Force(pub Vec2<f32>);

#[derive(Component, Default)]
pub struct Velocity(pub Vec2<f32>);

/// Position in 3D space.
#[derive(Component, Clone, Debug)]
pub struct Position(pub Vec3<f32>);

/// Position in tile buffers.
#[derive(Component, Clone, Debug)]
pub struct Coordinate(pub Vec2<usize>);

/// Size of a tile.
#[derive(Component, Clone, Debug)]
pub struct Size(pub f32);

#[derive(Component, Clone, Debug)]
pub struct Color(pub Vec4<f32>);

/// An internal name used to bundle assets together.
pub struct Name(pub String);
impl Component for Name {
    /// Storage is flag, so that when a name is changed or inserted, other systems
    /// are capable of applying the change.
    type Storage = FlaggedStorage<Self, VecStorage<Self>>;
}

/// Identity of entity for rendering.
#[derive(Component, Clone, Copy, Debug)]
pub struct RenderId(pub Option<usize>);

#[derive(Component, Clone, Copy, Debug)]
pub struct TextureIndex(pub Option<usize>);

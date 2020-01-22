use specs::prelude::*;
use specs::Component;
use vek::{Vec3, Vec4};

#[derive(Component, Clone, Debug)]
pub struct Position(Vec3<f32>);

#[derive(Component, Clone, Debug)]
pub struct Color(Vec4<f32>);

#[derive(Component, Clone, Debug)]
pub struct Tile;

#[derive(Component, Clone, Debug)]
pub struct Camera;

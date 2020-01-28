use specs::prelude::*;
use specs::Component;
use std::borrow::Cow;
use vek::{Vec3, Vec4};

#[derive(Component, Clone, Debug)]
pub struct Tile;

#[derive(Component, Clone, Debug)]
pub struct Camera;

/// Position in 3D space.
#[derive(Component, Clone, Debug)]
pub struct Position(pub Vec3<f32>);

#[derive(Component, Clone, Debug)]
pub struct Color(pub Vec4<f32>);

/// An internal name used to bundle assets together.
#[derive(Component, Clone, Debug)]
pub struct Identity(pub String);

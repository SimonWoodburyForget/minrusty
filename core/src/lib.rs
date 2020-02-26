mod error;
// mod platform;
mod clock;
mod components;
pub mod game;
mod loader;
mod map;
mod physics;
mod render;
mod state;
mod units;
mod window;

#[cfg(feature = "web")]
mod main_web;

pub use error::Error;
use game::*;

#[macro_use]
extern crate memoffset;

mod builder;
mod clock;
mod components;
mod error;
pub mod game;
mod loader;
mod logger;
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

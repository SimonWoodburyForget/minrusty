#[cfg(feature = "nat")]
mod native;
#[cfg(feature = "web")]
mod web;

#[cfg(feature = "nat")]
pub use native::Platform;
#[cfg(feature = "web")]
pub use web::Platform;

// TODO: figure out message passing structure

// pub struct Key {}
// // .. abcdefg ..

// pub enum Events {}
// // .. draw
// // .. key down
// // .. key up

// // .. rendered view of the game state, to be consumed by opengl
// pub struct Render {}
// // .. tile states
// // .. item states

// pub struct GametState {}
// // .. map size
// // .. tiles in map

// // .. compute the next state of the game
// pub struct UpdateSystem {}
// // .. event channel of message

// impl<'a> System<'a> for UpdateSystem {
//     type SystemData = ();

//     fn run(&mut self, _: ()) {
//         // .. for events from event channel
//         // .. gamestate update with message
//     }
// }

// // .. render the current state of the game
// pub struct RenderSystem;

// impl<'a> System<'a> for RenderSystem {
//     type SystemData = ();

//     fn run(&mut self, _: ()) {
//         // .. make a render from gamestate
//         // .. write render for renderer
//     }
// }

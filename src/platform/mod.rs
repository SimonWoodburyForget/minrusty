#[cfg(feature = "nat")]
mod native;
#[cfg(feature = "web")]
mod web;

#[cfg(feature = "nat")]
pub use native::init;
#[cfg(feature = "web")]
pub use web::init;

use winit::window::Window;

pub trait AsWindow {
    fn as_window(&self) -> &Window;
}

impl AsWindow for Window {
    fn as_window(&self) -> &Window {
        &self
    }
}

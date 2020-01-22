mod error;
#[cfg(feature = "web")]
mod main_web;
// mod platform;
mod components;
mod render;
mod state;
mod window;

pub use error::Error;
use window::Window;

#[cfg(feature = "web")]
mod wasm {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = console)]
        pub fn log(s: &str);
    }
}

pub fn log(x: &str) {
    #[cfg(feature = "web")]
    wasm::log(x);
    #[cfg(feature = "nat")]
    println!("{}", x);
}

pub fn main() {
    let game = state::GameState::new();
    let window = Window::new().unwrap();
    window.run();
}

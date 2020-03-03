use crate::game::resources::*;
use specs::prelude::*;

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

#[derive(Default)]
pub struct Sys;

impl<'a> System<'a> for Sys {
    type SystemData = (
        Read<'a, Frame>,
        Read<'a, Scene>,
        // ..
    );

    fn run(&mut self, (frame, scene): Self::SystemData) {
        if frame.0 % 100 == 0 {
            println!("");
            println!("frame {}", frame.0);
        }

        // for entry in entries.read(&mut self.reader_id.as_mut().unwrap()) {}
    }
}

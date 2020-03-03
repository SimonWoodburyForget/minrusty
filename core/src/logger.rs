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
        let mut buffer = String::new();

        buffer.push_str(&format!("\n frame {}", frame.0));
        buffer.push_str(&format!("\n dims {:?}", scene.screen_dimentions()));
        buffer.push_str(&format!("\n cursor {:?}", scene.screen_cursor()));

        if frame.0 % 100 == 0 {
            log(&buffer);
        }

        // for entry in entries.read(&mut self.reader_id.as_mut().unwrap()) {}
    }
}

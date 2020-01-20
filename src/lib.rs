#[cfg(feature = "web")]
mod main_web;

mod platform;
use platform::*;

mod render;

use specs::prelude::*;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
};

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
    let (event_loop, context, window_context) = init();

    // let mut world = World::new();
    let square = render::Square::new(&context).unwrap();
    // dispatcher.setup(&mut world);

    event_loop.run(move |event, _, control_flow| {
        let window = window_context.as_window();

        *control_flow = ControlFlow::Wait;

        // main_web::log(&format!("{:?}", event));

        match event {
            Event::WindowEvent { ref event, .. } => match event {
                WindowEvent::CloseRequested => {
                    // TODO: check window id?
                    *control_flow = ControlFlow::Exit;
                }
                _ => (),
            },
            Event::MainEventsCleared => {
                // window.request_redraw();
                // dispatcher.dispatch(&world);
                // world.maintain();

                square.draw(&context);

                #[cfg(feature = "nat")]
                {
                    // FIXME: this needs to be moved into platform::native, and
                    // drawing on web canvas probably doesn't work right now.
                    window_context.swap_buffers();
                }
            }
            _ => (),
        }
    });
}

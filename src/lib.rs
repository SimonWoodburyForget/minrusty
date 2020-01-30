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

use specs::prelude::*;
use state::*;

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
    let mut game = state::GameState::new();
    game.create_block(0., 1., "wall");
    game.create_block(1., 1., "core");

    let event_loop = winit::event_loop::EventLoop::new();
    let (window, renderer) = Window::new(&event_loop).unwrap();

    event_loop.run(move |event, _, control_flow| {
        use winit::event_loop::ControlFlow;
        *control_flow = ControlFlow::Poll;

        // #[cfg(feature = "nat")]
        // let window = windowed_context.window();

        game.update();

        use winit::event::Event::*;
        use winit::event::WindowEvent::*;

        match event {
            WindowEvent {
                event: CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,

            RedrawRequested(_) => {
                let game_render = &*game.ecs.read_resource::<GameRender>();

                let (w, h) = window.dimensions();
                renderer.draw(game_render.sin_wave, (w as _, h as _));
                window.on_draw();
            }

            WindowEvent {
                event: Resized(ref size),
                ..
            } => {
                crate::log(&format!("{:?}", size));
            }

            MainEventsCleared => {
                // crate::log(&format!("cleared!"));
                window.on_main_events_cleared();
            }

            // TODO:
            // .? LoopDestroyed => return
            _ => (),
        }
    });
}

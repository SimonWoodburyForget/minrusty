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
pub struct ScreenSize(pub (u32, u32));

pub fn main() {
    let mut game = state::GameState::new();
    game.create_block(0., 1., "wall");
    game.create_block(1., 1., "core");

    let event_loop = winit::event_loop::EventLoop::new();
    let (window, renderer) = Window::new(&event_loop).unwrap();
    game.ecs.insert(ScreenSize(window.dimensions()));

    event_loop.run(move |event, _, control_flow| {
        use winit::event_loop::ControlFlow;
        *control_flow = ControlFlow::Poll;

        game.update();

        {
            let mut ss = game.ecs.write_resource::<ScreenSize>();
            *ss = ScreenSize(window.dimensions());
        }

        use winit::event::Event;
        use winit::event::WindowEvent;

        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                // WindowEvent::Resized(ref size) => crate::log(&format!("{:?}", size)),
                _ => (),
            },

            Event::RedrawRequested(_) => {
                renderer.render(game.ecs.system_data());
                window.on_event(window::Event::Draw);
            }

            Event::MainEventsCleared => window.on_event(window::Event::Tick),

            // TODO:
            // - LoopDestroyed => return
            _ => (),
        }
    });
}

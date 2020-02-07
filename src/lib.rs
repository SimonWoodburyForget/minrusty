mod error;
#[cfg(feature = "web")]
mod main_web;
// mod platform;
mod components;
mod gfx_ex;
mod input;
mod physics;
mod player;
// mod render;
mod state;
mod units;
mod window;

mod graphics;

pub use error::Error;
use window::Window;

use shrev::*;
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
    game.create_player();
    game.create_block(0., 1., "wall");
    game.create_block(1., 1., "core");

    let event_loop = winit::event_loop::EventLoop::new();
    let (window, mut renderer) = Window::new(&event_loop).unwrap();
    game.ecs.insert(ScreenSize(window.dimensions()));

    event_loop.run(move |event, _, control_flow| {
        use winit::event_loop::ControlFlow;
        *control_flow = ControlFlow::Poll;

        {
            let mut ss = game.ecs.write_resource::<ScreenSize>();
            *ss = ScreenSize(window.dimensions());
        }

        use winit::event::{Event, KeyboardInput, WindowEvent};

        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::Resized(size) => {
                    #[cfg(feature = "nat")]
                    {
                        let context = renderer.surface.context();
                        context.resize(size);
                    }
                    use crate::gfx_ex::*;
                    renderer.dimensions = window::Extent2D {
                        width: size.width,
                        height: size.height,
                    };
                    renderer.recreate_swapchain();
                }
                WindowEvent::KeyboardInput { input, .. } => {
                    game.ecs
                        .write_resource::<EventChannel<KeyboardInput>>()
                        .single_write(input);
                }
                _ => (),
            },

            Event::RedrawRequested(_) => {
                game.update();
                renderer.render();
                window.on_event(window::Event::Draw);
            }

            Event::MainEventsCleared => window.on_event(window::Event::Tick),

            // TODO:
            // - LoopDestroyed => return
            _ => (),
        }
    });
}

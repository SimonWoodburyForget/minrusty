use crate::window::Window;
use crate::*;
use shrev::*;
use specs::prelude::*;
use std::time::Duration;
use vek::*;
use winit::dpi::PhysicalPosition;

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
pub struct ScreenSize(pub (i32, i32));

#[derive(Default)]
pub struct CursorInput(pub Vec2<i32>);

pub fn play() {
    let mut clock = clock::Clock::new();
    let event_loop = winit::event_loop::EventLoop::new();
    let (window, renderer) = Window::new(&event_loop).unwrap();

    let mut game = state::GameState::new(renderer);

    game.create_block(0, 1, "a");
    game.create_block(1, 1, "b");
    game.create_block(1, 0, "c");
    game.create_block(0, 0, "d");
    game.create_block(1, 2, "d");

    game.ecs.insert(ScreenSize(window.dimensions()));

    event_loop.run(move |event, _, control_flow| {
        use winit::event_loop::ControlFlow;
        *control_flow = ControlFlow::Poll;

        {
            let mut ss = game.ecs.write_resource::<ScreenSize>();
            *ss = ScreenSize(window.dimensions());
        }

        use winit::event::{Event, KeyboardInput, StartCause, WindowEvent};

        match event {
            Event::NewEvents(StartCause::Init) => {}

            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,

                WindowEvent::Resized(ref _size) => {}

                WindowEvent::KeyboardInput { input, .. } => {
                    game.ecs
                        .write_resource::<EventChannel<input::Event>>()
                        .single_write(input.into());
                }

                WindowEvent::CursorMoved { position, .. } => {
                    let PhysicalPosition { x, y } = position;
                    let cursor = CursorInput(Vec2::new(x, y));
                    game.ecs
                        .write_resource::<EventChannel<input::Event>>()
                        .single_write(cursor.into());
                }

                _ => {}
            },

            Event::RedrawRequested(_) => {
                clock.tick(Duration::from_millis(16));
                game.tick();
                window.on_event(window::Event::Draw);
            }

            Event::MainEventsCleared => {
                window.winit_window().request_redraw();
            }

            Event::LoopDestroyed => {}

            _ => {}
        }
    });
}

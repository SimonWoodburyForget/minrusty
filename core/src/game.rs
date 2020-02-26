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
struct KeyState {
    up: bool,
    down: bool,
    left: bool,
    right: bool,
}

impl From<KeyState> for Vec2<f32> {
    fn from(state: KeyState) -> Self {
        let to_float = |x| if x { 1.0 } else { 0.0 };

        #[rustfmt::skip]
        let KeyState { up, down, left, right, .. } = state;
        Vec2::new(
            to_float(right) - to_float(left),
            to_float(up) - to_float(down),
        )
        .try_normalized()
        .unwrap_or(Vec2::zero())
    }
}

#[derive(Default)]
pub struct ScreenSize(pub (i32, i32));

#[derive(Default)]
struct CursorState(pub Vec2<i32>);

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

    let mut key_state = KeyState::default();
    let mut cursor_state = CursorState::default();

    event_loop.run(move |event, _, control_flow| {
        use winit::event_loop::ControlFlow;
        *control_flow = ControlFlow::Poll;

        {
            let mut ss = game.ecs.write_resource::<ScreenSize>();
            *ss = ScreenSize(window.dimensions());
        }

        use winit::event::{Event, StartCause, WindowEvent};
        match event {
            Event::NewEvents(StartCause::Init) => {}

            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,

                WindowEvent::Resized(ref _size) => {}

                WindowEvent::KeyboardInput { input, .. } => {
                    use winit::event::{ElementState, KeyboardInput, VirtualKeyCode};
                    let KeyboardInput {
                        virtual_keycode,
                        state,
                        ..
                    } = input;
                    if let Some(vkc) = virtual_keycode {
                        let held = ElementState::Pressed == state;
                        match vkc {
                            VirtualKeyCode::Up => key_state.up = held,
                            VirtualKeyCode::Down => key_state.down = held,
                            VirtualKeyCode::Left => key_state.left = held,
                            VirtualKeyCode::Right => key_state.right = held,
                            _ => {}
                        };
                    };
                }

                WindowEvent::CursorMoved { position, .. } => {
                    let PhysicalPosition { x, y } = position;
                    cursor_state.0 = Vec2::new(x, y);
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

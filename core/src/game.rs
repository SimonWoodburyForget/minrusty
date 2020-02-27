use crate::window::Window;
use crate::*;
use specs::prelude::*;
use state::GameStart;
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

pub use resources::*;
pub mod resources {
    use super::*;

    #[derive(Default)]
    pub struct ScreenSize(pub Vec2<i32>);

    #[derive(Default, Clone, Copy)]
    pub struct CursorState(pub Vec2<i32>);

    /// As the player moves through the universe, the position of
    /// the universe changes, not the player itself.
    #[derive(Default, Clone, Copy, Debug)]
    pub struct UniversePosition(pub Vec2<f32>);

    #[derive(Default)]
    pub struct Frame(pub u64);
}

pub fn play() {
    let mut clock = clock::Clock::new();
    let event_loop = winit::event_loop::EventLoop::new();
    let (window, renderer) = Window::new(&event_loop).unwrap();

    let mut game = state::GameState::new(renderer);

    game.create_block(0, 1, "a", 0.1);
    game.create_block(1, 1, "b", 0.2);
    game.create_block(1, 0, "c", 0.3);
    game.create_block(0, 0, "d", 0.4);
    game.create_block(1, 2, "d", 0.5);

    game.ecs.insert(ScreenSize(window.dimensions().into()));

    let mut key_state = KeyState::default();
    let mut cursor_state = CursorState::default();
    let mut universe_position = UniversePosition::default();

    event_loop.run(move |event, _, control_flow| {
        use winit::event_loop::ControlFlow;
        *control_flow = ControlFlow::Poll;

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
                            VirtualKeyCode::Space => {
                                universe_position.0.x += 0.1;
                                println!("{:?}", universe_position);
                            }
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
                // let x = -1.0 + 0.3 * seconds.0.sin();
                // let y = -1.0 + 0.3 * seconds.0.cos();
                let frame = game.ecs.read_resource::<Frame>().0 + 1;
                if frame % 100 == 0 {
                    println!("");
                    println!("frame {}", frame);
                }

                *game.ecs.write_resource::<CursorState>() = cursor_state;
                *game.ecs.write_resource::<ScreenSize>() = ScreenSize(window.dimensions().into());
                *game.ecs.write_resource::<UniversePosition>() = universe_position;
                *game.ecs.write_resource::<Frame>() = Frame(frame);

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

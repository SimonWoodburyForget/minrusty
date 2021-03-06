//! Crate which holds the main event loop and various generic game
//! resources used globally throughout the game.

use crate::window::Window;
use crate::*;
use rand::prelude::*;
use specs::prelude::*;
use vek::*;
use winit::dpi::PhysicalPosition;

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
    pub struct Frame(pub u64);

    #[derive(Default)]
    pub struct Scene {
        screen_size: Vec2<i32>,
        cursor_state: Vec2<i32>,
    }

    impl Scene {
        pub fn new(screen_size: Vec2<i32>, cursor_state: Vec2<i32>) -> Self {
            Scene {
                screen_size,
                cursor_state,
            }
        }

        pub fn screen_dimentions(&self) -> &Vec2<i32> {
            &self.screen_size
        }

        /// normalize cursor coordinates into clip-space (-1 to 1)
        fn normalize(screen_size: Vec2<f32>, cursor_position: Vec2<f32>) -> Vec2<f32> {
            let mut v = ((cursor_position / screen_size) - Vec2::new(0.5, 0.5)) * 2.0;
            v.y = -v.y;
            // NOTE: screen-spaces starts at the top, so we reverse the y axis here.
            v
        }

        /// convert i32 into f32
        fn convert(vector: &Vec2<i32>) -> Vec2<f32> {
            vector.numcast().unwrap()
        }

        pub fn screen_cursor(&self) -> &Vec2<i32> {
            &self.cursor_state
        }

        /// cursor screen-space to clip-space to world-space transformation,
        /// for getting the coordinates of the cursor relative to the tiles.
        pub fn world_cursor(&self) -> Vec2<f32> {
            let Self {
                screen_size,
                cursor_state,
            } = self;

            let fscreen = Self::convert(screen_size);
            let fcursor = Self::convert(cursor_state);
            let ncursor = Self::normalize(fscreen, fcursor);
            let imatrix = self.transform().inverted();
            // NOTE: screen spaces goes top to bottom
            let [x, y, _, _] = (imatrix * Vec4::new(ncursor.x, ncursor.y, 0.0, 1.0)).into_array();
            Vec2::new(x, y)
        }

        /// a world cursor that rounds to tile coordinates
        pub fn coordinate_cursor(&self) -> Vec2<i32> {
            self.world_cursor().round().numcast().unwrap()
        }

        /// main world to clip-space transformation
        pub fn transform(&self) -> Mat4<f32> {
            let Self { screen_size, .. } = self;
            let screen_size = Self::convert(screen_size);

            // TODO: implement scaling
            let scale: Mat4<f32> = Mat4::scaling_3d(Vec3::new(100., 100., 1.0));
            #[rustfmt::skip]
        let frustum = {
            FrustumPlanes::<f32> {
                left: 0.0, right: screen_size.x,
                bottom: 0.0, top: screen_size.y,
                near: -10., far: 10.,
            }
        };
            let ortho = Mat4::orthographic_rh_zo(frustum);
            // TODO: implement player position
            let trans: Mat4<f32> = Mat4::translation_2d(Vec2::new(0.5, 1.0));
            (trans * ortho * scale) // * coordinate
        }
    }
}

pub fn play() {
    let event_loop = winit::event_loop::EventLoop::new();
    let (window, renderer) = Window::new(&event_loop).unwrap();
    let mut game = state::GameState::new(renderer);

    let mut rng = rand::thread_rng();
    let mut a = vec!["a"; 100];
    a.extend(["b"; 100].as_ref());
    a.extend(["c"; 100].as_ref());
    a.extend(["d"; 100].as_ref());
    a.shuffle(&mut rng);
    let mut content = a.iter().cycle();

    for i in 0..5 {
        for j in 0..5 {
            game.create_block(i, j, content.next().unwrap(), rng.gen_range(0.5, 0.9));
        }
    }

    let mut key_state = KeyState::default();
    let mut cursor_state = Default::default();
    // let mut universe_position = Default::default();

    event_loop.run(move |event, _, control_flow| {
        use winit::event_loop::ControlFlow;
        *control_flow = ControlFlow::Poll;

        use winit::event::{Event, StartCause, WindowEvent};
        match event {
            Event::NewEvents(StartCause::Init) => {}

            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,

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
                            VirtualKeyCode::Space => {}
                            _ => {}
                        };
                    };
                }

                WindowEvent::CursorMoved { position, .. } => {
                    let PhysicalPosition { x, y } = position;
                    cursor_state = Vec2::new(x, y);
                }

                _ => {}
            },

            Event::RedrawRequested(_) => {
                let frame = game.ecs.read_resource::<Frame>().0 + 1;
                if frame % 100 == 0 {
                    // println!("");
                    // println!("frame {}", frame);
                }

                // TODO: universe_position
                *game.ecs.write_resource::<Frame>() = Frame(frame);
                *game.ecs.write_resource::<Scene>() =
                    Scene::new(window.dimensions().into(), cursor_state);

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

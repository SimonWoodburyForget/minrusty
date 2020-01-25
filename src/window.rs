use crate::render::Renderer;
use crate::state;
use crate::Error;

#[cfg(feature = "web")]
use wasm_bindgen::JsCast;
#[cfg(feature = "web")]
use winit::platform::web::WindowExtWebSys;

use f32;
use winit::event_loop::ControlFlow;

// pub enum Event {}

/// Represents the games window on Web or Native.
pub struct Window {
    event_loop: winit::event_loop::EventLoop<()>,
    renderer: Renderer,

    // glutin wants to wrap the entire window to do it's own things
    #[cfg(feature = "nat")]
    windowed_context: glutin::ContextWrapper<glutin::PossiblyCurrent, winit::window::Window>,
    #[cfg(feature = "web")]
    window: winit::window::Window,
}

impl Window {
    pub fn new() -> Result<Self, Error> {
        // initialize a native context with glutin
        #[cfg(feature = "nat")]
        {
            let event_loop = glutin::event_loop::EventLoop::new();
            let window_builder = glutin::window::WindowBuilder::new()
                .with_title("Minrusty")
                .with_inner_size(glutin::dpi::LogicalSize::new(1024.0, 768.0));

            let windowed_context = glutin::ContextBuilder::new()
                .with_vsync(true)
                .build_windowed(window_builder, &event_loop)
                .unwrap();
            let windowed_context = unsafe { windowed_context.make_current().unwrap() };

            let context = glow::Context::from_loader_function(|s| {
                windowed_context.get_proc_address(s) as *const _
            });
            let renderer = Renderer::new(context)?;

            Ok(Self {
                event_loop,
                renderer,
                windowed_context,
            })
        }

        // initialize a web context with web-sys
        #[cfg(feature = "web")]
        {
            let event_loop = winit::event_loop::EventLoop::new();
            let window = winit::window::WindowBuilder::new()
                .with_title("Minrusty")
                .build(&event_loop)
                .unwrap();

            let context = {
                let canvas = window.canvas();

                let web_gl_2 = canvas
                    .get_context("webgl2")
                    .unwrap()
                    .unwrap()
                    .dyn_into::<web_sys::WebGl2RenderingContext>()
                    .unwrap();

                let window = web_sys::window().unwrap();
                let document = window.document().unwrap();
                let body = document.body().unwrap();

                body.append_child(&canvas)
                    .expect("Append canvas to HTML body");

                glow::Context::from_webgl2_context(web_gl_2)
            };

            let renderer = Renderer::new(context)?;

            Ok(Self {
                event_loop,
                renderer,
                window,
            })
        }
    }

    pub fn run(self, mut gs: state::GameState) {
        let Self {
            event_loop,
            renderer,
            #[cfg(feature = "nat")]
            windowed_context,
            #[cfg(feature = "web")]
            window,
        } = self;

        #[cfg(feature = "web")]
        let canvas = window.canvas();

        let mut counter = 1.0;

        event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Poll;

            #[cfg(feature = "nat")]
            let window = windowed_context.window();

            #[cfg(feature = "web")]
            {
                let w = canvas.client_width();
                let h = canvas.client_height();
                // crate::log(&format!("{:?}", (w, h)));

                // TODO:
                // .. send resize event
                // .. set gl viewport
            }

            // TODO:
            // .. on redraw request
            // .. read render state
            // .. pass render state to rendering function
            // .. check window id?

            gs.update();

            use winit::event::Event::*;
            use winit::event::WindowEvent::*;

            match event {
                WindowEvent {
                    event: CloseRequested,
                    ..
                } => *control_flow = ControlFlow::Exit,

                RedrawRequested(_) => {
                    let game_render = gs.render();
                    renderer.draw(game_render.sin_wave);

                    #[cfg(feature = "nat")]
                    windowed_context.swap_buffers().unwrap();
                }

                WindowEvent {
                    event: Resized(ref size),
                    ..
                } => {
                    crate::log(&format!("{:?}", size));
                }

                MainEventsCleared => {
                    // crate::log(&format!("cleared!"));
                    window.request_redraw();
                }

                // TODO:
                // .? LoopDestroyed => return
                _ => (),
            }
        });

        // .. destruction
    }
}

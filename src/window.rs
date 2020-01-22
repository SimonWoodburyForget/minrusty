use crate::render::Renderer;
use crate::Error;

#[cfg(feature = "web")]
use wasm_bindgen::JsCast;
#[cfg(feature = "web")]
use winit::platform::web::WindowExtWebSys;

use winit::event_loop::ControlFlow;

// pub enum Event {}

/// Represents the games window on Web or Native.
pub struct Window {
    event_loop: winit::event_loop::EventLoop<()>,
    renderer: Renderer,
    #[cfg(feature = "nat")]
    windowed_context: glutin::ContextWrapper<glutin::PossiblyCurrent, winit::window::Window>,
    #[cfg(feature = "web")]
    window: winit::window::Window,
}

impl Window {
    pub fn new() -> Result<Self, Error> {
        // Initialization difference between native and web is very large,
        // short of ending at almost the same types.

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

    pub fn run(self) {
        let Self {
            event_loop,
            renderer,
            #[cfg(feature = "nat")]
            windowed_context,
            #[cfg(feature = "web")]
            window,
        } = self;

        #[cfg(feature = "nat")]
        let window = windowed_context.window();

        event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Wait;

            // TODO:
            // .. on redraw request
            // .. read render state
            // .. pass render state to rendering function
            // .. check window id?

            // TODO:
            // .. put css in page to handle canvas size
            // .. set canvas.width to canvas.clientWidth
            // .. set canvas.height to canvas.clientHeight

            use winit::event::Event::*;
            use winit::event::WindowEvent::*;

            match event {
                WindowEvent {
                    event: CloseRequested,
                    ..
                } => *control_flow = ControlFlow::Exit,

                RedrawRequested(_) => {
                    renderer.draw();

                    #[cfg(feature = "nat")]
                    windowed_context.swap_buffers().unwrap();
                }

                WindowEvent {
                    event: Resized(ref size),
                    ..
                } => {
                    crate::log(&format!("{:?}", size));
                }

                // TODO:
                // .? Event::LoopDestroyed => return
                // .? Event::MainEventsCleared => { .. }
                _ => (),
            }
        });

        // .. destruction
    }
}

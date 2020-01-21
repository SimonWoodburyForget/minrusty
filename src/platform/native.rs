use crate::render::Renderer;

use glow::*;
use glutin::{ContextWrapper, PossiblyCurrent};
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};

pub struct Platform {
    event_loop: EventLoop<()>,
    renderer: Renderer,
    windowed_context: ContextWrapper<PossiblyCurrent, Window>,
}

impl Platform {
    pub fn new() -> Self {
        let event_loop = glutin::event_loop::EventLoop::new();
        let window_builder = glutin::window::WindowBuilder::new()
            .with_title("Hello triangle!")
            .with_inner_size(glutin::dpi::LogicalSize::new(1024.0, 768.0));

        let windowed_context = glutin::ContextBuilder::new()
            .with_vsync(true)
            .build_windowed(window_builder, &event_loop)
            .unwrap();
        let windowed_context = unsafe { windowed_context.make_current().unwrap() };

        let context = glow::Context::from_loader_function(|s| {
            windowed_context.get_proc_address(s) as *const _
        });
        let renderer = Renderer::new(context).unwrap();

        Self {
            event_loop,
            renderer,
            windowed_context,
        }
    }

    pub fn run(self) {
        let Self {
            event_loop,
            renderer,
            windowed_context,
        } = self;

        event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Wait;

            // TODO:
            // .. on redraw request
            // .. read render state
            // .. pass render state to rendering function
            // .. check window id?

            match event {
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => *control_flow = ControlFlow::Exit,

                Event::RedrawRequested(_) => {
                    renderer.draw();
                    windowed_context.swap_buffers().unwrap();
                }

                Event::WindowEvent {
                    event: WindowEvent::Resized(ref size),
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

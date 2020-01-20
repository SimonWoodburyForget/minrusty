#[cfg(feature = "web")]
mod main_web;

mod platform;
use platform::*;

mod render;

use specs::prelude::*;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
};

pub fn main() {
    let (event_loop, context, window_context) = init();

    let mut world = World::new();
    let mut dispatcher = DispatcherBuilder::new()
        .with_thread_local(render::RenderSystem::new(context).unwrap())
        .build();
    dispatcher.setup(&mut world);

    event_loop.run(move |event, _, control_flow| {
        let window = window_context.as_window();

        *control_flow = ControlFlow::Wait;

        // main_web::log(&format!("{:?}", event));

        match event {
            Event::WindowEvent { ref event, .. } => match event {
                WindowEvent::CloseRequested => {
                    // TODO: check window id?
                    *control_flow = ControlFlow::Exit;
                }
                _ => (),
            },
            Event::MainEventsCleared => {
                window.request_redraw();
                dispatcher.dispatch(&world);
                world.maintain();

                // FIXME: this needs to be moved into platform::native, and
                // drawing on web canvas probably doesn't work right now.
                window_context.swap_buffers();
            }
            _ => (),
        }
    });
}

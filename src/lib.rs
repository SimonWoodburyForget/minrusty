#[cfg(feature = "web")]
mod main_web;

mod platform;
use platform::*;

use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

pub fn main() {
    let (event_loop, context, window_context) = init();

    event_loop.run(move |event, _, control_flow| {
        let window = window_context.as_window();

        *control_flow = ControlFlow::Wait;

        // main_web::log(&format!("{:?}", event));

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.id() => *control_flow = ControlFlow::Exit,
            Event::MainEventsCleared => {
                window.request_redraw();
            }
            _ => (),
        }
    });
}

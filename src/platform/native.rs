use crate::platform::AsWindow;

use glow::*;
use glutin::{ContextWrapper, PossiblyCurrent};
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

impl AsWindow for ContextWrapper<PossiblyCurrent, Window> {
    fn as_window(&self) -> &Window {
        self.window()
    }
}

pub fn init() -> (
    EventLoop<()>,
    Context,
    ContextWrapper<PossiblyCurrent, Window>,
) {
    let event_loop = glutin::event_loop::EventLoop::new();
    let window = glutin::window::WindowBuilder::new()
        .with_title("Hello triangle!")
        .with_inner_size(glutin::dpi::LogicalSize::new(1024.0, 768.0));
    let windowed_context = glutin::ContextBuilder::new()
        .with_vsync(true)
        .build_windowed(window, &event_loop)
        .unwrap();
    let windowed_context = unsafe { windowed_context.make_current().unwrap() };
    let context =
        glow::Context::from_loader_function(|s| windowed_context.get_proc_address(s) as *const _);

    (event_loop, context, windowed_context)
}

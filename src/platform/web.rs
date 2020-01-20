use glow::*;
use wasm_bindgen::JsCast;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    platform::web::WindowExtWebSys,
    window::{Window, WindowBuilder},
};

pub fn init() -> (EventLoop<()>, Context, Window) {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
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

    (event_loop, context, window)
}
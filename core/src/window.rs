use crate::render::Renderer;
use crate::Error;
use std::convert::TryInto;

#[cfg(feature = "web")]
use wasm_bindgen::JsCast;
#[cfg(feature = "web")]
use winit::platform::web::WindowExtWebSys;

#[allow(dead_code)]
pub enum Event {
    /// Aften the window is drawn.
    Draw,

    /// Aften events have finished firing, and right before drawing.
    Tick,
}

/// Represents the games window on Web or Native.
pub struct Window {
    #[cfg(feature = "nat")]
    windowed_context: glutin::ContextWrapper<glutin::PossiblyCurrent, winit::window::Window>,
    #[cfg(feature = "web")]
    window: winit::window::Window,
}

impl Window {
    pub fn new(event_loop: &winit::event_loop::EventLoop<()>) -> Result<(Self, Renderer), Error> {
        // initialize a native context with glutin
        #[cfg(feature = "nat")]
        {
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

            Ok((Self { windowed_context }, renderer))
        }

        // initialize a web context with web-sys
        #[cfg(feature = "web")]
        {
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

            Ok((Self { window }, renderer))
        }
    }

    pub fn winit_window(&self) -> &winit::window::Window {
        #[cfg(feature = "web")]
        {
            &self.window
        }

        #[cfg(feature = "nat")]
        {
            self.windowed_context.window()
        }
    }

    pub fn device_pixel_ratio(&self) -> f64 {
        #[cfg(feature = "web")]
        {
            let window = web_sys::window().unwrap();
            window.device_pixel_ratio()
        }
        #[cfg(feature = "nat")]
        {
            unimplemented!();
        }
    }

    pub fn dimensions(&self) -> (i32, i32) {
        use winit::dpi::*;
        #[cfg(feature = "web")]
        {
            let canvas = self.window.canvas();
            // NOTE: must query clientWidth and clientHeight in order to get screen space coordinates,
            // as querying `self.window.inner_size()` returns the canvas size, which isn't in screen space.
            let (width, height) = (canvas.client_width(), canvas.client_height());

            // NOTE: prevents browser zoom factor from affecting the canvas resolution.
            let factor = self.window.scale_factor();
            let logical = LogicalSize { width, height };
            let PhysicalSize { width, height } = logical.to_physical(factor);

            // NOTE: canvas doesn't expect to be resized by the user, but we use CSS to
            // resize it, which doesn't fire any events, so this is required to maintain.
            canvas.set_width(width);
            canvas.set_height(height);

            (width as i32, height as i32)
        }
        #[cfg(feature = "nat")]
        {
            let PhysicalSize { width, height } = self.windowed_context.window().inner_size();
            (width.try_into().unwrap(), height.try_into().unwrap())
        }
    }

    pub fn on_event(&self, event: Event) {
        match event {
            Event::Draw => {
                #[cfg(feature = "nat")]
                self.windowed_context.swap_buffers().unwrap();
            }

            _ => {}
        }
    }
}

use glow::*;
use specs::prelude::*;
use std::{thread::sleep, time::Duration};

mod components;
mod resources;
mod systems;

use systems::RenderSystem;

pub fn main() {
    // Create a context from a sdl2 window
    #[cfg(feature = "window-sdl2")]
    let (gl, mut events_loop, render_loop, shader_version, _gl_context, window) = {
        let sdl = sdl2::init().unwrap();
        let video = sdl.video().unwrap();
        let gl_attr = video.gl_attr();
        gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
        gl_attr.set_context_version(3, 0);

        let window = video
            .window("Hello triangle!", 1024, 769)
            .opengl()
            .resizable()
            .build()
            .unwrap();
        let gl_context = window.gl_create_context().unwrap();
        let context =
            glow::Context::from_loader_function(|s| video.gl_get_proc_address(s) as *const _);
        // let render_loop = glow::RenderLoop::<sdl2::video::Window>::from_sdl_window(window);
        let event_loop = sdl.event_pump().unwrap();
        (context, event_loop, 0, "#version 410", gl_context, window)
    };

    let mut world = World::new();
    let mut dispatcher = DispatcherBuilder::new()
        .with_thread_local(RenderSystem::new(gl, shader_version))
        .build();
    dispatcher.setup(&mut world);

    let mut running = true;
    while running {
        dispatcher.dispatch(&world);
        world.maintain();

        window.gl_swap_window();
        for event in events_loop.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => running = false,
                _ => {}
            }
        }

        sleep(Duration::from_millis(50));
    }
}

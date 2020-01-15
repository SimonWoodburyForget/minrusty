use specs::prelude::*;

mod components;
mod systems;
mod resources;

use systems::{
    RenderSystem,
    EventSystem,
};

pub fn main() {
    let (context, event_pump, glsl_version) = {
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
        let render_loop = glow::RenderLoop::<sdl2::video::Window>::from_sdl_window(window);
        let event_pump = sdl.event_pump().unwrap();
        // (context, event_loop, render_loop, "#version 410", gl_context)
        let glsl_version = "#version 410";
        (context, event_pump, glsl_version)
    };
    
    let mut world = World::new();
    let mut dispatcher = DispatcherBuilder::new()
        .with_thread_local(RenderSystem::new(context, glsl_version))
        .with_thread_local(EventSystem(event_pump))
        .build();
    dispatcher.setup(&mut world);
}

use glow::*;
use specs::prelude::*;

#[cfg(all(target_arch = "wasm32", feature = "web-sys"))]
use wasm_bindgen::prelude::*;

#[cfg(all(target_arch = "wasm32", feature = "stdweb"))]
use std_web::{
    traits::*,
    unstable::TryInto,
    web::{document, html_element::*},
};
#[cfg(all(target_arch = "wasm32", feature = "stdweb"))]
use webgl_stdweb::WebGL2RenderingContext;

mod components;
mod systems;
mod resources;

#[cfg_attr(all(target_arch = "wasm32", feature = "web-sys"), wasm_bindgen(start))]
pub fn wasm_main() {
    main();
}

pub fn main() {

    let world = World::new();
    
    // Create a context from a WebGL2 context on wasm32 targets
    #[cfg(all(target_arch = "wasm32", feature = "web-sys"))]
    let (_window, gl, _events_loop, render_loop, shader_version) = {
        use wasm_bindgen::JsCast;
        let canvas = web_sys::window().unwrap()
            .document().unwrap()
            .get_element_by_id("canvas").unwrap()
            .dyn_into::<web_sys::HtmlCanvasElement>().unwrap();
        let webgl2_context = canvas
            .get_context("webgl2").unwrap().unwrap()
            .dyn_into::<web_sys::WebGl2RenderingContext>().unwrap();
        (
            (),
            glow::Context::from_webgl2_context(webgl2_context),
            (),
            glow::RenderLoop::from_request_animation_frame(),
            "#version 300 es",
        )
    };

    #[cfg(all(target_arch = "wasm32", feature = "stdweb"))]
    let (_window, gl, _events_loop, render_loop, shader_version) = {
        let canvas: CanvasElement = document()
            .create_element("canvas")
            .unwrap()
            .try_into()
            .unwrap();
        document().body().unwrap().append_child(&canvas);
        canvas.set_width(640);
        canvas.set_height(480);
        let webgl2_context: WebGL2RenderingContext = canvas.get_context().unwrap();
        (
            (),
            glow::Context::from_webgl2_context(webgl2_context),
            (),
            glow::RenderLoop::from_request_animation_frame(),
            "#version 300 es",
        )
    };
    
    // Create a context from a sdl2 window
    #[cfg(feature = "window-sdl2")]
    let (gl, mut events_loop, render_loop, shader_version, _gl_context) = {
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
        let event_loop = sdl.event_pump().unwrap();
        (context, event_loop, render_loop, "#version 410", gl_context)
    };
    
    let (vertex_array, program) = unsafe {
        let va = gl
            .create_vertex_array()
            .expect("Cannot create vertex array");
        gl.bind_vertex_array(Some(va));

        let p = gl.create_program().expect("Cannot create program");
        (va, p)
    };

    let shader_sources = [
        (glow::VERTEX_SHADER, include_str!("shaders/vss.glsl")),
        (glow::FRAGMENT_SHADER, include_str!("shaders/fss.glsl")),
    ];

    let mut shaders = Vec::with_capacity(shader_sources.len());
    
    for (shader_type, shader_source) in shader_sources.iter() {
        unsafe {
            let shader = gl.create_shader(*shader_type).expect("Cannot create shader");
            gl.shader_source(shader, &format!("{}\n{}", shader_version, shader_source));
            gl.compile_shader(shader);
            if !gl.get_shader_compile_status(shader) {
                panic!(gl.get_shader_info_log(shader));
            }
            gl.attach_shader(program, shader);
            shaders.push(shader);
        }
    }

    unsafe {
        gl.link_program(program);
        if !gl.get_program_link_status(program) {
            panic!(gl.get_program_info_log(program));
        }

        for shader in shaders {
            gl.detach_shader(program, shader);
            gl.delete_shader(shader);
        }

        gl.use_program(Some(program));
        gl.clear_color(0.1, 0.2, 0.3, 1.0);
    }
    
    #[cfg(not(feature = "window-glutin"))]
    render_loop.run(move |running: &mut bool| {
        #[cfg(feature = "window-sdl2")]
        {
            for event in events_loop.poll_iter() {
                match event {
                    sdl2::event::Event::Quit { .. } => *running = false,
                    _ => {}
                }
            }
        }
        
        unsafe {
            gl.clear(glow::COLOR_BUFFER_BIT);
            gl.draw_arrays(glow::TRIANGLES, 0, 3);
            
            if !*running {
                gl.delete_program(program);
                gl.delete_vertex_array(vertex_array);
            }
        }
    });
}
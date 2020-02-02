//! This module contains the OpenGL rendering pipelines.
//!
//! It's a multi-platform module, meaning it's for the most part all going to be used
//! on Web and Native targets, so we're targetting mostly OpenGL ES 3.0 features.

mod buffer;
mod error;
mod program;
mod texture;
mod types;
mod uniform;
mod vertex_array;

pub use buffer::*;
pub use error::*;
pub use program::*;
pub use texture::*;
pub use types::*;
pub use uniform::*;
pub use vertex_array::*;

use glow::*;

use crate::components::*;
use crate::state::GameStart;
use crate::ScreenSize;
use specs::prelude::*;

use image::io::Reader;
use image::DynamicImage;
use image::FilterType;
use image::ImageFormat;
use std::io::Cursor;
use vek::Mat4;

/// Type which holds onto the OpenGL context, and the various objects that surrounds it.
pub struct Renderer {
    gl: Context,

    tx: Texture,
    va: VertexArray,
    pg: Program,
}

/// Loads an image from bytes, resizes it to 32x32 to avoid dealing with varying size images.
fn load_bytes(bytes: &[u8]) -> DynamicImage {
    Reader::new(Cursor::new(bytes.as_ref()))
        .with_guessed_format()
        .expect("Cursor io never fails!")
        .decode()
        .unwrap()
        // TODO:
        // - handle images of varying sizes
        .resize(32, 32, FilterType::Nearest)
}

impl Renderer {
    pub fn new(gl: Context) -> Result<Self, RenderError> {
        let (vert_pos, text_pos, tile_pos) = (0, 1, 2);

        let pg = Program::new(
            &gl,
            include_str!("shaders/vss.glsl"),
            include_str!("shaders/fss.glsl"),
            &[
                (vert_pos, "vert_pos"),
                (text_pos, "text_pos"),
                (tile_pos, "tile_pos"),
            ],
        )?;

        // TODO:
        // - bind attrib location in program

        let images = [
            load_bytes(include_bytes!("../../assets/a.png")),
            load_bytes(include_bytes!("../../assets/b.png")),
            load_bytes(include_bytes!("../../assets/d.png")),
            load_bytes(include_bytes!("../../assets/c.png")),
        ];

        let tx = Texture::from_images(&gl, &images)?;

        #[rustfmt::skip]
        let vertices = [
             // square 1 
             // pos       // texture
             0.5,  0.5,   1.0,  1.0, // top right
             0.5, -0.5,   1.0,  0.0, // bottom right
            -0.5,  0.5,   0.0,  1.0, // top left
            -0.5, -0.5,   0.0,  0.0_f32, // bottom left
        ];
        let vertex_buffer = Buffer::immutable(&gl, glow::ARRAY_BUFFER, &vertices)?;
        let vertex_buffer_attributes = [
            VertexAttribute::new(vert_pos, 2),
            VertexAttribute::new(text_pos, 2),
        ];

        #[rustfmt::skip]
        let indices: [u32; 6] = [
            0, 1, 2, // top right triangle
            2, 3, 1, // buttom left triangle
        ];
        let element_buffer = Buffer::immutable(&gl, glow::ELEMENT_ARRAY_BUFFER, &indices)?;

        #[rustfmt::skip]
        let instance_positions = [
            0.0, 0.0_f32,
            0.0, 1.0,
            0.0, 2.0,
            0.0, 3.0,
            0.0, 4.0,
            0.0, 5.0,
        ];
        let instance_buffer = Buffer::immutable(&gl, glow::ARRAY_BUFFER, &instance_positions)?;
        let instance_buffer_attributes = [
            VertexAttribute::new(tile_pos, 2).with_div(1), // tiling
        ];

        let va = {
            let bindings = &[
                (&instance_buffer, instance_buffer_attributes.as_ref()),
                (&vertex_buffer, vertex_buffer_attributes.as_ref()),
            ];

            VertexArray::new(&gl, bindings, &element_buffer)
        }?;

        Ok(Self { va, pg, tx, gl })
    }

    pub fn render<'a>(
        &self,
        (start, screen_size, ent, positions): (
            Read<'a, GameStart>,
            Read<'a, ScreenSize>,
            Entities<'a>,
            ReadStorage<'a, Position>,
        ),
    ) {
        let Self { va, pg, tx, gl } = self;

        let mut pos_vec = Vec::new();
        for (_, pos) in (&*ent, &positions).join() {
            pos_vec.push(pos.0);
        }

        let elapsed = start.0.elapsed();
        let sec_from_start = elapsed.as_secs() as f32 + elapsed.subsec_nanos() as f32 * 1e-9;

        use f32;
        let scale = sec_from_start.sin();

        let ScreenSize((w, h)) = *screen_size;

        let mut m = Mat4::identity();
        m.scale_3d(scale);

        unsafe {
            gl.clear(glow::COLOR_BUFFER_BIT);

            // using this on web makes things weird
            #[cfg(feature = "nat")]
            gl.viewport(0, 0, w as _, h as _);

            pg.use_program(&gl);
            // pg.set_uniform(&gl, "ourColor", Vec4::new(0.0, green, 0.0, 1.0));
            pg.set_uniform(&gl, "transform", m);
            tx.bind(&gl);
            va.bind(&gl);
            // gl.draw_arrays(glow::TRIANGLES, 0, 6);
            gl.draw_elements_instanced(glow::TRIANGLES, 6, glow::UNSIGNED_INT, 0, 4);

            // gl.draw_elements(glow::TRIANGLES, 6, glow::UNSIGNED_INT, 0);
            gl.bind_vertex_array(None);
        }
    }
}

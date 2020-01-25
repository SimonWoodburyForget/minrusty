//! This module contains thin OpenGL wrappers for rendering data.

mod error;
mod program;
mod texture;
mod types;
mod uniform;
mod vertex_array;

pub use error::*;
pub use program::*;
pub use texture::*;
pub use types::*;
pub use uniform::*;
pub use vertex_array::*;

use glow::*;

use image::io::Reader;
use image::DynamicImage;
use image::ImageFormat;
use std::io::Cursor;
use vek::Mat4;

/// Type for handling all GPU operations.
pub struct Renderer {
    gl: Context,

    tx: Texture,
    va: VertexArray,
    pg: Program,
}

impl Renderer {
    pub fn new(gl: Context) -> Result<Self, RenderError> {
        let raw_data = include_bytes!("../../assets/core-shard.png");
        let mut reader = Reader::new(Cursor::new(raw_data.as_ref()))
            .with_guessed_format()
            .expect("Cursor io never fails!");
        let image = reader.decode().unwrap();

        // TODO:
        // - this is pretty unsound
        #[rustfmt::skip]
        let vertices: [f32; 32] = [
             // pos            // col           // tex
             0.5,  0.5, 0.0,   1.0, 0.0, 0.0,   1.0,  1.0, // top right
             0.5, -0.5, 0.0,   0.0, 1.0, 0.0,   1.0,  0.0, // bottom right
            -0.5,  0.5, 0.0,   0.0, 0.0, 1.0,   0.0,  1.0, // top left
            -0.5, -0.5, 0.0,   1.0, 1.0, 0.0,   0.0,  0.0, // bottom left
        ];

        #[rustfmt::skip]
        let indices: [u32; 6] = [
            0, 1, 2, // top right triangle
            2, 3, 1, // buttom left triangle
        ];

        Ok(Self {
            va: VertexArray::new(&gl, &vertices, &indices)?,
            tx: Texture::new(&gl, image)?,
            pg: Program::new(
                &gl,
                include_str!("shaders/vss.glsl"),
                include_str!("shaders/fss.glsl"),
            )?,

            gl: gl,
        })
    }

    pub fn draw(&self, green: f32) {
        let Self { va, pg, tx, gl } = self;
        unsafe {
            gl.clear(glow::COLOR_BUFFER_BIT);

            let mut m = Mat4::identity();
            m.scale_3d(green);

            pg.use_program(&gl);
            // pg.set_uniform(&gl, "ourColor", Vec4::new(0.0, green, 0.0, 1.0));
            pg.set_uniform(&gl, "transform", m);
            tx.bind(&gl);
            va.bind(&gl);
            // gl.draw_arrays(glow::TRIANGLES, 0, 6);
            gl.draw_elements(glow::TRIANGLES, 6, glow::UNSIGNED_INT, 0);
            gl.bind_vertex_array(None);
        }
    }
}

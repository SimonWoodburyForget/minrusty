//! This module contains thin OpenGL wrappers for rendering data.

mod error;
mod program;
mod types;
mod uniform;
mod vertex_array;

pub use error::*;
pub use program::*;
pub use types::*;
pub use uniform::*;
pub use vertex_array::*;

use glow::*;
use specs::prelude::*;
use std::mem;
use vek::Vec4;

/// Type for handling all GPU operations.
pub struct Renderer {
    gl: Context,
    square: Square,
}

impl Renderer {
    pub fn new(gl: Context) -> Result<Self, RenderError> {
        Ok(Self {
            square: Square::new(&gl)?,
            gl: gl,
        })
    }

    pub fn draw(&self, green: f32) {
        self.square.draw(&self.gl, green);
    }
}

pub struct Texture {
    tex: TextureId,
}

impl Texture {
    pub fn new(gl: Context) -> Result<Self, RenderError> {
        unsafe {
            let tex = gl.create_texture()?;

            Ok(Self { tex })
        }
    }
}

/// A ..Square renderer
pub struct Square {
    va: VertexArray,
    pg: Program,
}

impl Square {
    pub fn new(gl: &Context) -> Result<Self, String> {
        // TODO:
        // - this is pretty unsound

        let vstride = 3;

        #[rustfmt::skip]
        let vertices: [f32; 24] = [
             0.5,  0.5, 0.0,   1.0, 0.0, 0.0, // top right
             0.5, -0.5, 0.0,   0.0, 1.0, 0.0, // bottom right
            -0.5,  0.5, 0.0,   0.0, 0.0, 1.0, // top left
            -0.5, -0.5, 0.0,   1.0, 1.0, 0.0, // bottom left
        ];

        #[rustfmt::skip]
        let indices: [u32; 6] = [
            0, 1, 2, // top right triangle
            2, 3, 1, // buttom left triangle
        ];

        Ok(Self {
            va: VertexArray::new(&gl, &vertices, &indices, vstride)?,
            pg: Program::new(
                &gl,
                include_str!("shaders/vss.glsl"),
                include_str!("shaders/fss.glsl"),
            )?,
        })
    }

    pub fn draw(&self, gl: &Context, green: f32) {
        let Self { va, pg } = self;
        unsafe {
            gl.clear(glow::COLOR_BUFFER_BIT);

            pg.use_program(&gl);
            // pg.set_uniform(&gl, "ourColor", Vec4::new(0.0, green, 0.0, 1.0));
            va.bind(&gl);
            // gl.draw_arrays(glow::TRIANGLES, 0, 6);
            gl.draw_elements(glow::TRIANGLES, 6, glow::UNSIGNED_INT, 0);
            gl.bind_vertex_array(None);
        }
    }
}

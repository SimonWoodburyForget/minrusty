//! This module contains thin OpenGL wrappers for rendering data.

mod error;
mod program;
mod types;
mod uniform;

pub use error::*;
pub use program::*;
pub use types::*;
pub use uniform::*;

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

pub struct VertexArray {
    vao: VertexArrayId,
    vbo: BufferId,
    ebo: BufferId,
}

impl VertexArray {
    /// Initializes vertex and index buffers from an OpenGL context.
    pub fn new(
        gl: &Context,
        vertices: &[f32],
        indices: &[u32],
        vstride: i32,
    ) -> Result<Self, String> {
        let Self { vao, vbo, ebo } = unsafe { Self::create(&gl) }?;
        unsafe {
            gl.bind_vertex_array(Some(vao));

            gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));
            gl.buffer_data_u8_slice(
                glow::ARRAY_BUFFER,
                &vertices.align_to::<u8>().1,
                glow::STATIC_DRAW,
            );

            gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(ebo));
            gl.buffer_data_u8_slice(
                glow::ELEMENT_ARRAY_BUFFER,
                &indices.align_to::<u8>().1,
                glow::STATIC_DRAW,
            );

            let stride = 6 * mem::size_of::<f32>() as i32;
            gl.vertex_attrib_pointer_f32(0, 3, glow::FLOAT, false, stride, 0);
            gl.enable_vertex_attrib_array(0);

            let start = vstride * mem::size_of::<f32>() as i32;
            gl.vertex_attrib_pointer_f32(1, 3, glow::FLOAT, false, stride, start);
            gl.enable_vertex_attrib_array(1);

            gl.bind_vertex_array(None);
        };

        Ok(Self { vao, vbo, ebo })
    }

    pub unsafe fn create(gl: &Context) -> Result<Self, String> {
        Ok(Self {
            vao: gl.create_vertex_array()?,
            vbo: gl.create_buffer()?,
            ebo: gl.create_buffer()?,
        })
    }

    pub unsafe fn bind(&self, gl: &Context) {
        gl.bind_vertex_array(Some(self.vao));
        // gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(self.ebo));
        // gl.bind_buffer(glow::ARRAY_BUFFER, Some(self.vbo));
    }

    pub unsafe fn delete(&self, gl: &Context) {
        gl.delete_vertex_array(self.vao);
        gl.delete_buffer(self.vbo);
        gl.delete_buffer(self.ebo);
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

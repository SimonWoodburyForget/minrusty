use crate::render::error::*;
use crate::render::types::*;

use glow::*;
use std::mem;

pub struct VertexArray {
    vao: VertexArrayId,
    vbo: BufferId,
    ebo: BufferId,
}

impl VertexArray {
    /// Initializes vertex and index buffers from an OpenGL context.
    pub fn new(gl: &Context, vertices: &[f32], indices: &[u32]) -> Result<Self, String> {
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

            let strides = 8 * mem::size_of::<f32>() as i32;

            let offset = 0;
            gl.vertex_attrib_pointer_f32(0, 3, glow::FLOAT, false, strides, offset);
            gl.enable_vertex_attrib_array(0);

            let offset = 3 * mem::size_of::<f32>() as i32;
            gl.vertex_attrib_pointer_f32(1, 3, glow::FLOAT, false, strides, offset);
            gl.enable_vertex_attrib_array(1);

            let offset = 6 * mem::size_of::<f32>() as i32;
            gl.vertex_attrib_pointer_f32(2, 3, glow::FLOAT, false, strides, offset);
            gl.enable_vertex_attrib_array(2);

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

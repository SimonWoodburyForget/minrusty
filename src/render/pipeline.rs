use super::*;

use glow::*;
use std::iter;
use std::mem;

pub struct Pipeline {
    buffer: Buffer,
    vertex_attributes: Vec<VertexAttribute>,
    stride_size: usize,
    stride_count: usize,

    /// Raw data sent to OpenGL.
    data: Vec<f32>,

    /// Lookup Vec of free memory into raw data, used mainly to distribute Id's.
    free: Vec<bool>,
}

impl Pipeline {
    pub fn new(buffer: Buffer, vertex_attributes: Vec<VertexAttribute>, data: Vec<f32>) -> Self {
        let stride_count = vertex_attributes
            .iter()
            .map(|attr| attr.size as usize)
            .sum();
        let stride_size = stride_count * mem::size_of::<f32>();

        // TODO: either initialize data here, or make the caller declare this, otherwise
        // we really don't know what is "free" or not.
        let free = iter::repeat(true).take(data.len() / stride_count).collect();

        Self {
            buffer,
            vertex_attributes,
            stride_count,
            stride_size,
            data,
            free,
        }
    }

    /// Returns a free id, or an error if it failed to allocate.
    pub fn next_free(&mut self) -> Result<usize, RenderError> {
        for (id, is_free) in self.free.iter_mut().enumerate() {
            if *is_free {
                *is_free = false;
                return Ok(id);
            }
        }

        return Err(RenderError::BufferFull);
    }

    /// Stupid simple update function, pushes one change to the buffer right away, uses
    /// an `id` to refer to a specific slice of sub data in the buffer.
    pub fn update_slice(&mut self, gl: &Context, id: usize, values: &[f32]) {
        let index = id * self.stride_count;

        // NOTE: this is probably a redundant operation, we could just send sub
        // slices directly to OpenGL, instead of the entire buffer.
        for (a, b) in self.data[index..].iter_mut().zip(values.iter()) {
            *a = *b;
        }

        self.buffer.update(&gl, &self.data);
    }

    pub fn setup(&self, gl: &Context) {
        let data_type = glow::FLOAT;

        self.buffer.bind(&gl);
        let mut offset = 0;
        for attr in self.vertex_attributes.iter() {
            attr.setup(&gl, self.stride_size as _, offset, data_type);
            offset += attr.size;
        }
    }
}

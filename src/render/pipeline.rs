use super::*;

use glow::*;
use std::iter;
use std::mem;

pub struct Pipeline {
    buffer: Buffer,
    vertex_attributes: Vec<VertexAttribute>,
    stride_size: usize,
}

impl Pipeline {
    pub fn new(buffer: Buffer, vertex_attributes: Vec<VertexAttribute>) -> Self {
        let stride_count = vertex_attributes
            .iter()
            .map(|attr| attr.size as usize)
            .sum::<usize>();
        let stride_size = stride_count * mem::size_of::<f32>();

        Self {
            buffer,
            vertex_attributes,
            stride_size,
        }
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

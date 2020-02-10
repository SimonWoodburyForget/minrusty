use super::*;

use glow::*;
use std::mem;

pub struct Layout<T> {
    vertex_attributes: Vec<VertexAttribute<T>>,
    stride_size: usize,
}

impl<T> Layout<T> {
    pub fn new(vertex_attributes: Vec<VertexAttribute<T>>) -> Self {
        let stride_count = vertex_attributes
            .iter()
            .map(|attr| attr.size as usize)
            .sum::<usize>();
        let stride_size = stride_count * mem::size_of::<T>();

        Self {
            vertex_attributes,
            stride_size,
        }
    }

    pub fn setup(&self, gl: &Context, buffer: &Buffer<T>) {
        let data_type = glow::FLOAT;

        buffer.bind(&gl);
        let mut offset = 0;
        for attr in self.vertex_attributes.iter() {
            attr.setup(&gl, self.stride_size as _, offset, data_type);
            offset += attr.size;
        }
    }
}

pub struct Buffer<T> {
    buffer_id: Option<BufferId>,
    buffer_type: u32,
    size: usize,
    phantom: std::marker::PhantomData<T>,
    layout: Layout<T>,
}

impl<T> Buffer<T> {
    pub fn immutable(
        gl: &Context,
        buffer_type: u32,
        data: &[T],
        va: Vec<VertexAttribute<T>>,
    ) -> Result<Self, RenderError> {
        // SAFETY: questionable, more testing needed.

        let size = data.len();
        let buffer_id = Some(unsafe { gl.create_buffer()? });

        let (head, body, tail) = unsafe { data.align_to::<u8>() };
        assert!(head.is_empty());
        assert!(tail.is_empty());

        unsafe {
            gl.bind_buffer(buffer_type, buffer_id);
            gl.buffer_data_u8_slice(buffer_type, body, glow::STATIC_DRAW);
            gl.bind_buffer(buffer_type, None);
        }

        let layout = Layout::new(va);

        Ok(Self {
            buffer_id,
            buffer_type,
            size,
            phantom: std::marker::PhantomData,
            layout,
        })
    }

    pub fn _dynamic(_gl: &Context, _buffer_type: u32, _size: usize) -> Result<Self, RenderError> {
        unimplemented!();
    }

    pub fn update(&self, gl: &Context, index: i32, data: &[T]) {
        assert!(index as usize + data.len() <= self.size);

        let (head, body, tail) = unsafe { data.align_to::<u8>() };
        assert!(head.is_empty());
        assert!(tail.is_empty());

        self.bind(&gl);
        unsafe { gl.buffer_sub_data_u8_slice(self.buffer_type, index, body) };
        self.unbind(&gl);
    }

    pub fn bind(&self, gl: &Context) {
        unsafe {
            gl.bind_buffer(self.buffer_type, self.buffer_id);
        }
    }

    pub fn unbind(&self, gl: &Context) {
        unsafe {
            gl.bind_buffer(self.buffer_type, None);
        }
    }

    pub fn _delete(&self, gl: &Context) {
        if let Some(buffer_id) = self.buffer_id {
            unsafe { gl.delete_buffer(buffer_id) };
        }
    }

    pub fn setup(&self, gl: &Context) {
        self.layout.setup(&gl, &self);
    }
}

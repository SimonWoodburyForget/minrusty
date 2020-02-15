use super::*;

use glow::*;
use std::mem;

pub struct Buffer<P: Pipeline> {
    buffer_id: Option<BufferId>,
    buffer_type: u32,
    size: usize,
    phantom: std::marker::PhantomData<P::Vertex>,
}

impl<P: Pipeline> Buffer<P> {
    /// # SAFETY
    ///
    /// `data` will be read after this function has been called.
    pub unsafe fn immutable(
        gl: &Context,
        buffer_type: u32,
        data: &[P::Vertex],
    ) -> Result<Self, RenderError> {
        let size = data.len();
        let buffer_id = Some(unsafe { gl.create_buffer()? });

        let (head, data, tail) = unsafe { data.align_to::<u8>() };
        assert!(head.is_empty());
        assert!(tail.is_empty());

        unsafe {
            gl.bind_buffer(buffer_type, buffer_id);
            gl.buffer_data_u8_slice(buffer_type, data, glow::STATIC_DRAW);
            gl.bind_buffer(buffer_type, None);
        }

        Ok(Self {
            buffer_id,
            buffer_type,
            size,
            phantom: std::marker::PhantomData,
        })
    }

    pub fn dynamic(gl: &Context, buffer_type: u32, size: usize) -> Result<Self, RenderError> {
        let buffer_id;
        unsafe {
            // SAFETY: should be safe because it doesn't pass any raw memory.
            buffer_id = Some(gl.create_buffer()?);
            gl.bind_buffer(buffer_type, buffer_id);
            gl.buffer_data_size(buffer_type, size as _, glow::STREAM_DRAW);
            gl.bind_buffer(buffer_type, None);
        }

        Ok(Self {
            buffer_id,
            buffer_type,
            size,
            phantom: std::marker::PhantomData,
        })
    }

    /// # SAFETY
    ///
    /// `data` will be read after this function has been called.
    pub unsafe fn update(&self, gl: &Context, index: i32, data: &[P::Vertex]) {
        assert!(index as usize + data.len() <= self.size);

        let (head, data, tail) = data.align_to::<u8>();
        assert!(head.is_empty());
        assert!(tail.is_empty());

        gl.bind_buffer(self.buffer_type, self.buffer_id);
        gl.buffer_sub_data_u8_slice(self.buffer_type, index, data);
        gl.bind_buffer(self.buffer_type, None)
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
}

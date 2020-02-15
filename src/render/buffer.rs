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
    /// unsafe because `gl.buffer_data_u8_slice` reads `data` asynchronously, which
    /// means mutating it after this function is called could result in errors, as
    /// the underlaying implementation just gives OpenGL a raw pointer into the slice.
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

    pub fn _dynamic(_gl: &Context, _buffer_type: u32, _size: usize) -> Result<Self, RenderError> {
        unimplemented!();
    }

    pub unsafe fn update(&self, gl: &Context, index: i32, data: &[P::Vertex]) {
        assert!(index as usize + data.len() <= self.size);

        let (head, data, tail) = unsafe { data.align_to::<u8>() };
        assert!(head.is_empty());
        assert!(tail.is_empty());

        unsafe {
            gl.bind_buffer(self.buffer_type, self.buffer_id);
            gl.buffer_sub_data_u8_slice(self.buffer_type, index, data);
            gl.bind_buffer(self.buffer_type, None)
        };
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

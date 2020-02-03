use super::*;

use glow::*;

#[derive(Clone, Copy, Debug, Default)]
pub struct Buffer {
    buffer_id: Option<BufferId>,
    buffer_type: u32,
    size: usize,
}

impl Buffer {
    pub fn immutable<T>(gl: &Context, buffer_type: u32, data: &[T]) -> Result<Self, RenderError> {
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

        Ok(Self {
            buffer_id,
            buffer_type,
            size,
        })
    }

    pub fn _dynamic(_gl: &Context, _buffer_type: u32, _size: usize) -> Result<Self, RenderError> {
        unimplemented!();
    }

    pub fn update<T>(&self, gl: &Context, data: &[T]) {
        debug_assert!(data.len() <= self.size);

        let (head, body, tail) = unsafe { data.align_to::<u8>() };
        assert!(head.is_empty());
        assert!(tail.is_empty());

        self.bind(&gl);
        unsafe { gl.buffer_sub_data_u8_slice(self.buffer_type, 0, body) };
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
}

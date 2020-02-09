use super::*;

use glow::*;
use std::iter;
use std::mem;

#[derive(Clone, Copy)]
pub struct VertexAttribute<T> {
    location: u32,
    pub size: i32,
    divisor: Option<u32>,
    phantom: std::marker::PhantomData<T>,
}

impl<T> VertexAttribute<T> {
    pub fn new(location: u32, size: i32) -> Self {
        Self {
            location,
            size,
            divisor: None,
            phantom: std::marker::PhantomData,
        }
    }

    pub fn with_div(self, divisor: u32) -> Self {
        Self {
            divisor: Some(divisor),
            ..self
        }
    }

    pub fn setup(&self, gl: &Context, stride_size: i32, offset: i32, data_type: u32) {
        unsafe {
            gl.vertex_attrib_pointer_f32(
                self.location,
                self.size,
                data_type,
                false,
                stride_size,
                offset * mem::size_of::<T>() as i32,
            );
            gl.vertex_attrib_divisor(self.location, self.divisor.unwrap_or(0));
            gl.enable_vertex_attrib_array(self.location);
        }
    }
}

pub struct VertexArray {
    /// Vertex Array
    vao: Option<VertexArrayId>,
}

impl VertexArray {
    /// Initializes the vertex array object and it's vertex attribute pointers. Uses `vertex_bindings`
    /// to setup `VertexAttribute` in relation to some `Buffer`.
    pub fn new<F: Fn(&Context)>(gl: &Context, f: F) -> Result<Self, RenderError> {
        let vao = Some(unsafe { gl.create_vertex_array()? });
        unsafe { gl.bind_vertex_array(vao) };

        f(&gl);

        unsafe {
            gl.bind_buffer(glow::ARRAY_BUFFER, None);
            gl.bind_vertex_array(None);
        }

        Ok(Self { vao })
    }

    pub fn bind(&self, gl: &Context) {
        unsafe { gl.bind_vertex_array(self.vao) };
    }

    pub fn _delete(&self, gl: &Context) {
        if let Some(vao) = self.vao {
            unsafe { gl.delete_vertex_array(vao) };
        }

        // TODO
        // self.vertex_buffer.delete(&gl);
        // self.element_buffer.delete(&gl);
    }
}

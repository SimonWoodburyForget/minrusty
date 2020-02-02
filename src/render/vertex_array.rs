use super::*;

use glow::*;
use std::mem;

#[derive(Clone, Copy)]
pub struct VertexAttribute {
    location: u32,
    size: i32,
    divisor: Option<u32>,
}

impl VertexAttribute {
    pub fn new(location: u32, size: i32) -> Self {
        Self {
            location,
            size,
            divisor: None,
        }
    }

    pub fn with_div(self, divisor: u32) -> Self {
        Self {
            divisor: Some(divisor),
            ..self
        }
    }

    fn setup(&self, gl: &Context, stride_size: i32, offset: i32, data_type: u32) {
        unsafe {
            gl.vertex_attrib_pointer_f32(
                self.location,
                self.size,
                data_type,
                false,
                stride_size,
                offset * mem::size_of::<f32>() as i32,
            );
            gl.vertex_attrib_divisor(self.location, self.divisor.unwrap_or(0));
            gl.enable_vertex_attrib_array(self.location);
        }
    }
}

fn setup_vertex_attributes(
    gl: &Context,
    data_type: u32,
    attrs: &[VertexAttribute],
    vertex_buffer: &Buffer,
) {
    let stride_count = attrs.iter().map(|attr| attr.size).sum::<i32>();
    let stride_size = stride_count * unsafe { mem::size_of::<f32>() } as i32;

    vertex_buffer.bind(&gl);
    let mut offset = 0;
    for attr in attrs.iter() {
        attr.setup(&gl, stride_size, offset, data_type);
        offset += attr.size;
    }
}

pub struct VertexArray {
    /// Vertex Array
    vao: Option<VertexArrayId>,
}

impl VertexArray {
    /// Initializes the vertex array object and it's vertex attribute pointers. Uses `vertex_bindings`
    /// to setup `VertexAttribute` in relation to some `Buffer`.
    pub fn new(
        gl: &Context,
        vertex_bindings: &[(&Buffer, &[VertexAttribute])],
        element_buffer: &Buffer,
    ) -> Result<Self, RenderError> {
        let vao = Some(unsafe { gl.create_vertex_array()? });
        unsafe { gl.bind_vertex_array(vao) };

        element_buffer.bind(&gl);

        for (vertex_buffer, vertex_attributes) in vertex_bindings.iter() {
            setup_vertex_attributes(&gl, glow::FLOAT, &vertex_attributes, &vertex_buffer);
        }

        unsafe {
            gl.bind_buffer(glow::ARRAY_BUFFER, None);
            gl.bind_vertex_array(None);
        }

        Ok(Self { vao })
    }

    pub unsafe fn bind(&self, gl: &Context) {
        gl.bind_vertex_array(self.vao);
    }

    pub unsafe fn delete(&self, gl: &Context) {
        if let Some(vao) = self.vao {
            gl.delete_vertex_array(vao);
        }

        // TODO
        // self.vertex_buffer.delete(&gl);
        // self.element_buffer.delete(&gl);
    }
}

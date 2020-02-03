use super::*;

use glow::*;
use std::mem;

pub struct BufferLayout {
    buffer: Buffer,
    vertex_attributes: Vec<VertexAttribute>,
    data: Vec<f32>,
    stride_size: i32,
}

impl BufferLayout {
    pub fn new(buffer: Buffer, vertex_attributes: Vec<VertexAttribute>, data: Vec<f32>) -> Self {
        let stride_count = vertex_attributes.iter().map(|attr| attr.size).sum::<i32>();
        let stride_size = stride_count * mem::size_of::<f32>() as i32;

        Self {
            buffer,
            vertex_attributes,
            stride_size,
            data,
        }
    }

    /// stupid simple update function, pushes one change to the buffer right away
    pub fn update(&mut self, gl: &Context, index: usize, value: f32) {
        self.data[index] = value;
        self.buffer.update(&gl, &self.data);
    }

    pub fn setup(&self, gl: &Context) {
        let data_type = glow::FLOAT;

        self.buffer.bind(&gl);
        let mut offset = 0;
        for attr in self.vertex_attributes.iter() {
            attr.setup(&gl, self.stride_size, offset, data_type);
            offset += attr.size;
        }
    }
}

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

pub struct VertexArray {
    /// Vertex Array
    vao: Option<VertexArrayId>,
}

impl VertexArray {
    /// Initializes the vertex array object and it's vertex attribute pointers. Uses `vertex_bindings`
    /// to setup `VertexAttribute` in relation to some `Buffer`.
    pub fn new(
        gl: &Context,
        vertex_buffers_layout: &[&BufferLayout],
        element_buffer: &Buffer,
    ) -> Result<Self, RenderError> {
        let vao = Some(unsafe { gl.create_vertex_array()? });
        unsafe { gl.bind_vertex_array(vao) };

        element_buffer.bind(&gl);

        for layout in vertex_buffers_layout.iter() {
            layout.setup(&gl);
        }

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

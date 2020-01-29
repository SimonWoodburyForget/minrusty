use super::*;

use glow::*;
use std::mem;

pub struct VertexArray {
    /// Vertex Array
    vao: Option<VertexArrayId>,
    vertex_buffer: Buffer,
    element_buffer: Buffer,
}

impl VertexArray {
    /// Initializes vertex and index buffers from an OpenGL context.
    pub fn quad(gl: &Context) -> Result<Self, RenderError> {
        #[rustfmt::skip]
        let vertices: [f32; 32] = [
             // square 1 
             // pos            // color         // texture
             0.5,  0.5, 0.0,   1.0, 0.0, 0.0,   1.0,  1.0, // top right
             0.5, -0.5, 0.0,   0.0, 1.0, 0.0,   1.0,  0.0, // bottom right
            -0.5,  0.5, 0.0,   0.0, 0.0, 1.0,   0.0,  1.0, // top left
            -0.5, -0.5, 0.0,   1.0, 1.0, 0.0,   0.0,  0.0, // bottom left
        ];
        let vertex_buffer = Buffer::immutable(&gl, glow::ARRAY_BUFFER, &vertices)?;

        #[rustfmt::skip]
        let indices: [u32; 6] = [
            0, 1, 2, // top right triangle
            2, 3, 1, // buttom left triangle
        ];
        let element_buffer = Buffer::immutable(&gl, glow::ELEMENT_ARRAY_BUFFER, &indices)?;

        struct VertexAttribute {
            location: u32,
            size: i32,
            data_type: u32,
        }

        impl VertexAttribute {
            fn new(location: u32, size: i32, data_type: u32) -> Self {
                Self {
                    location,
                    size,
                    data_type,
                }
            }
        }

        let attrs: [VertexAttribute; 3] = [
            VertexAttribute::new(0, 3, glow::FLOAT), // positon
            VertexAttribute::new(1, 3, glow::FLOAT), // color
            VertexAttribute::new(2, 2, glow::FLOAT), // texture
        ];

        let vao = Some(unsafe { gl.create_vertex_array()? });
        unsafe { gl.bind_vertex_array(vao) };

        vertex_buffer.bind(&gl);
        element_buffer.bind(&gl);

        let stride_count = attrs.iter().map(|a| a.size).sum::<i32>();
        let stride_size = stride_count * unsafe { mem::size_of::<f32>() } as i32;

        let mut offset = 0;
        for attr in attrs.iter() {
            unsafe {
                gl.vertex_attrib_pointer_f32(
                    attr.location,
                    attr.size,
                    attr.data_type,
                    false,
                    stride_size,
                    offset * mem::size_of::<f32>() as i32,
                );
                // TODO: gl vertex attrib devisor?
                gl.enable_vertex_attrib_array(attr.location);
            }
            offset += attr.size;
        }

        unsafe {
            gl.bind_buffer(glow::ARRAY_BUFFER, None);
            gl.bind_vertex_array(None);
        }

        Ok(Self {
            vao,
            vertex_buffer,
            element_buffer,
        })
    }

    pub unsafe fn bind(&self, gl: &Context) {
        gl.bind_vertex_array(self.vao);
        // gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(self.ebo));
        // gl.bind_buffer(glow::ARRAY_BUFFER, Some(self.vbo));
    }

    pub unsafe fn delete(&self, gl: &Context) {
        if let Some(vao) = self.vao {
            gl.delete_vertex_array(vao);
        }

        self.vertex_buffer.delete(&gl);
        self.element_buffer.delete(&gl);
    }
}

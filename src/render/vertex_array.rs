use super::*;

use glow::*;
use std::mem;

#[derive(Clone, Copy)]
struct VertexAttribute {
    location: u32,
    size: i32,
    divisor: Option<u32>,
}

impl VertexAttribute {
    fn new(location: u32, size: i32) -> Self {
        Self {
            location,
            size,
            divisor: None,
        }
    }
}

struct VertexAttributes {
    data_type: u32,
    attrs: Vec<VertexAttribute>,
}

impl VertexAttributes {
    fn new(data_type: u32, attrs: Vec<VertexAttribute>) -> Self {
        // TODO: implement this for other types
        assert_eq!(data_type, glow::FLOAT);

        Self { data_type, attrs }
    }

    fn setup(&self, gl: &Context) {
        let Self { data_type, attrs } = self;

        let stride_count = attrs.iter().map(|a| a.size).sum::<i32>();
        let stride_size = stride_count * unsafe { mem::size_of::<f32>() } as i32;

        let mut offset = 0;
        for VertexAttribute {
            location,
            size,
            divisor,
        } in attrs.iter()
        {
            unsafe {
                // TODO: bind buffer?
                gl.vertex_attrib_pointer_f32(
                    *location,
                    *size,
                    *data_type,
                    false,
                    stride_size,
                    offset * mem::size_of::<f32>() as i32,
                );
                if let Some(d) = *divisor {
                    gl.vertex_attrib_divisor(*location, d);
                }
                gl.enable_vertex_attrib_array(*location);
            }
            offset += *size;
        }
    }
}

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

        let attrs = VertexAttributes::new(
            glow::FLOAT,
            vec![
                VertexAttribute::new(0, 3), // positon
                VertexAttribute::new(1, 3), // color
                VertexAttribute::new(2, 2), // texture
            ],
        );

        let vao = Some(unsafe { gl.create_vertex_array()? });
        unsafe { gl.bind_vertex_array(vao) };

        vertex_buffer.bind(&gl);
        element_buffer.bind(&gl);
        attrs.setup(&gl);

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
    }

    pub unsafe fn delete(&self, gl: &Context) {
        if let Some(vao) = self.vao {
            gl.delete_vertex_array(vao);
        }

        self.vertex_buffer.delete(&gl);
        self.element_buffer.delete(&gl);
    }
}

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

    fn with_div(self, divisor: u32) -> Self {
        Self {
            divisor: Some(divisor),
            ..self
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

    fn setup(&self, gl: &Context, vertex_buffer: &Buffer, element_buffer: Option<&Buffer>) {
        let Self { data_type, attrs } = self;

        let stride_count = attrs.iter().map(|a| a.size).sum::<i32>();
        let stride_size = stride_count * unsafe { mem::size_of::<f32>() } as i32;

        if let Some(ebo) = element_buffer {
            ebo.bind(&gl);
        }

        let mut offset = 0;
        for VertexAttribute {
            location,
            size,
            divisor,
        } in attrs.iter()
        {
            vertex_buffer.bind(&gl);
            unsafe {
                gl.vertex_attrib_pointer_f32(
                    *location,
                    *size,
                    *data_type,
                    false,
                    stride_size,
                    offset * mem::size_of::<f32>() as i32,
                );
                gl.vertex_attrib_divisor(*location, divisor.unwrap_or(0));
                gl.enable_vertex_attrib_array(*location);
            }
            // vertex_buffer.unbind(&gl);
            offset += *size;
        }
    }
}

pub struct VertexArray {
    /// Vertex Array
    vao: Option<VertexArrayId>,
    attrs: VertexAttributes,
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

        #[rustfmt::skip]
        let instance_positions = [
            0.0, 0.0_f32,
            0.0, 1.0,
            0.0, 2.0,
            0.0, 3.0,
            0.0, 4.0,
            0.0, 5.0,
        ];
        let instance_positions = Buffer::immutable(&gl, glow::ARRAY_BUFFER, &instance_positions)?;

        let attrs = VertexAttributes::new(
            glow::FLOAT,
            vec![
                VertexAttribute::new(0, 3), // position
                VertexAttribute::new(1, 3), // color
                VertexAttribute::new(2, 2), // texture
            ],
        );

        let attrs_b = VertexAttributes::new(
            glow::FLOAT,
            vec![
                VertexAttribute::new(3, 2).with_div(1), // tiling
            ],
        );

        let vao = Some(unsafe { gl.create_vertex_array()? });
        unsafe { gl.bind_vertex_array(vao) };

        attrs.setup(&gl, &vertex_buffer, Some(&element_buffer));
        attrs_b.setup(&gl, &instance_positions, None);

        unsafe {
            gl.bind_buffer(glow::ARRAY_BUFFER, None);
            gl.bind_vertex_array(None);
        }

        Ok(Self { vao, attrs })
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

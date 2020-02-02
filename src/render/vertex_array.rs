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
    /// Initializes vertex and index buffers from an OpenGL context.
    pub fn quad(gl: &Context) -> Result<Self, RenderError> {
        #[rustfmt::skip]
        let vertices = [
             // square 1 
             // pos       // texture
             0.5,  0.5,   1.0,  1.0, // top right
             0.5, -0.5,   1.0,  0.0, // bottom right
            -0.5,  0.5,   0.0,  1.0, // top left
            -0.5, -0.5,   0.0,  0.0_f32, // bottom left
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

        let vao = Some(unsafe { gl.create_vertex_array()? });
        unsafe { gl.bind_vertex_array(vao) };

        element_buffer.bind(&gl);

        setup_vertex_attributes(
            &gl,
            glow::FLOAT,
            &[
                VertexAttribute::new(0, 2), // position
                VertexAttribute::new(1, 2), // texture
            ],
            &vertex_buffer,
        );

        setup_vertex_attributes(
            &gl,
            glow::FLOAT,
            &[
                VertexAttribute::new(2, 2).with_div(1), // tiling
            ],
            &instance_positions,
        );

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

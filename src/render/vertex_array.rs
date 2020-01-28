use super::*;

use glow::*;
use std::mem;

pub struct VertexArray {
    /// Vertex Array
    vao: VertexArrayId,
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

        let vao = unsafe {
            let vao = gl.create_vertex_array()?;
            gl.bind_vertex_array(Some(vao));

            vertex_buffer.bind(&gl);
            element_buffer.bind(&gl);

            let strides = 8 * mem::size_of::<f32>() as i32;

            let offset = 0;
            gl.vertex_attrib_pointer_f32(0, 3, glow::FLOAT, false, strides, offset);
            gl.enable_vertex_attrib_array(0);

            let offset = 3 * mem::size_of::<f32>() as i32;
            gl.vertex_attrib_pointer_f32(1, 3, glow::FLOAT, false, strides, offset);
            gl.enable_vertex_attrib_array(1);

            let offset = 6 * mem::size_of::<f32>() as i32;
            gl.vertex_attrib_pointer_f32(2, 2, glow::FLOAT, false, strides, offset);
            gl.enable_vertex_attrib_array(2);

            // // Texture identity data?
            // gl.bind_buffer(glow::ARRAY_BUFFER, Some(ibo));
            // gl.buffer_data_u8_slice(
            //     glow::ARRAY_BUFFER,
            //     &texture_indices.align_to::<u8>().1,
            //     glow::STATIC_DRAW,
            // );
            // gl.vertex_attrib_divisor(3, 1);

            // let offset = 0;
            // let strides = 1 * mem::size_of::<i32>() as i32;
            // gl.vertex_attrib_pointer_f32(3, 1, glow::UNSIGNED_INT, false, strides, offset);
            // gl.enable_vertex_attrib_array(3);

            gl.bind_buffer(glow::ARRAY_BUFFER, None);
            gl.bind_vertex_array(None);
            vao
        };

        Ok(Self {
            vao,
            vertex_buffer,
            element_buffer,
        })
    }

    pub unsafe fn bind(&self, gl: &Context) {
        gl.bind_vertex_array(Some(self.vao));
        // gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(self.ebo));
        // gl.bind_buffer(glow::ARRAY_BUFFER, Some(self.vbo));
    }

    pub unsafe fn delete(&self, gl: &Context) {
        gl.delete_vertex_array(self.vao);
        self.vertex_buffer.delete(&gl);
        self.element_buffer.delete(&gl);
    }
}

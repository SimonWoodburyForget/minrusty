//! This module contains the OpenGL rendering pipelines.
//!
//! It's a multi-platform module, meaning it's for the most part all going to be used
//! on Web and Native targets, so we're targetting mostly OpenGL ES 3.0 features.

mod buffer;
mod error;
mod img;
mod program;
mod texture;
mod types;
mod uniform;
mod vertex_array;

pub use buffer::*;
pub use error::*;
pub use img::*;
pub use program::*;
pub use texture::*;
pub use types::*;
pub use uniform::*;
pub use vertex_array::*;

use glow::*;

use crate::components::*;
use crate::state::GameStart;
use crate::ScreenSize;
use specs::prelude::*;

use vek::Mat4;

/// Type which holds onto the OpenGL context, and the various objects that surrounds it.
pub struct Renderer {
    gl: Context,

    texture: Texture,
    vertex_array: VertexArray,
    program: Program,

    instance_data: Vec<f32>,
    instance_buffer_layout: BufferLayout,
}

impl Renderer {
    pub fn new(gl: Context) -> Result<Self, RenderError> {
        let (vert_pos, text_pos, tile_pos, tile_size) = (0, 1, 2, 3);

        let program = Program::new(
            &gl,
            include_str!("shaders/vss.glsl"),
            include_str!("shaders/fss.glsl"),
            &[
                (vert_pos, "vert_pos"),
                (text_pos, "text_pos"),
                (tile_pos, "tile_pos"),
                (tile_size, "tile_size"),
            ],
        )?;

        // TODO:
        // - bind attrib location in program

        let images = [
            load_bytes(include_bytes!("../../assets/a.png")),
            load_bytes(include_bytes!("../../assets/b.png")),
            load_bytes(include_bytes!("../../assets/d.png")),
            load_bytes(include_bytes!("../../assets/c.png")),
        ];

        let texture = Texture::from_images(&gl, &images)?;

        #[rustfmt::skip]
        let vertices = [
             // pos       // texture
             0.5,  0.5,   1.0,  1.0, // top right
             0.5, -0.5,   1.0,  0.0, // bottom right
            -0.5,  0.5,   0.0,  1.0, // top left
            -0.5, -0.5,   0.0,  0.0_f32, // bottom left
        ];
        let vertex_buffer_layout = BufferLayout::new(
            Buffer::immutable(&gl, glow::ARRAY_BUFFER, &vertices)?,
            vec![
                VertexAttribute::new(vert_pos, 2),
                VertexAttribute::new(text_pos, 2),
            ],
        );

        #[rustfmt::skip]
        let indices: [u32; 6] = [
            0, 1, 2, // top right triangle
            2, 3, 1, // buttom left triangle
        ];
        let element_buffer = Buffer::immutable(&gl, glow::ELEMENT_ARRAY_BUFFER, &indices)?;

        #[rustfmt::skip]
        let instance_data = vec![
            // pos     // size
            0.0, 0.0,  2.0_f32,
            0.0, 1.0,  1.0,
            0.0, 2.0,  1.0,
            0.0, 3.0,  1.0,
            0.0, 4.0,  1.0,
            0.0, 5.0,  1.0,
        ];
        let instance_buffer_layout = BufferLayout::new(
            Buffer::immutable(&gl, glow::ARRAY_BUFFER, &instance_data)?,
            vec![
                VertexAttribute::new(tile_pos, 2).with_div(1),
                VertexAttribute::new(tile_size, 1).with_div(1),
            ],
        );

        let vertex_array = {
            let bindings = &[&instance_buffer_layout, &vertex_buffer_layout];

            VertexArray::new(&gl, bindings, &element_buffer)
        }?;

        Ok(Self {
            vertex_array,
            program,
            texture,

            instance_data,
            instance_buffer_layout,

            gl,
        })
    }

    pub fn render<'a>(
        &mut self,
        (start, screen_size, ent, positions, mut id): (
            Read<'a, GameStart>,
            Read<'a, ScreenSize>,
            Entities<'a>,
            ReadStorage<'a, Position>,
            WriteStorage<'a, RenderId>,
        ),
    ) {
        let Self {
            gl,
            ref mut instance_data,
            instance_buffer_layout,
            ..
        } = self;

        let mut pos_vec = Vec::new();
        for (_, pos, ref mut id) in (&*ent, &positions, &mut id).join() {
            pos_vec.push(pos.0);
        }

        let elapsed = start.0.elapsed();
        let sec_from_start = elapsed.as_secs() as f32 + elapsed.subsec_nanos() as f32 * 1e-9;

        let scale = sec_from_start.sin();

        instance_data[0] = scale;
        instance_buffer_layout.buffer.update(&gl, &instance_data);

        #[allow(dead_code)]
        let ScreenSize((w, h)) = *screen_size;

        let mut m = Mat4::identity();
        m.scale_3d(scale);

        unsafe {
            gl.clear(glow::COLOR_BUFFER_BIT);

            // using this on web makes things weird
            #[cfg(feature = "nat")]
            gl.viewport(0, 0, w as _, h as _);

            self.program.use_program(&gl);
            // program.set_uniform(&gl, "ourColor", Vec4::new(0.0, green, 0.0, 1.0));
            self.program.set_uniform(&gl, "transform", m);
            self.texture.bind(&gl);
            self.vertex_array.bind(&gl);
            // gl.draw_arrays(glow::TRIANGLES, 0, 6);
            gl.draw_elements_instanced(glow::TRIANGLES, 6, glow::UNSIGNED_INT, 0, 4);

            // gl.draw_elements(glow::TRIANGLES, 6, glow::UNSIGNED_INT, 0);
            gl.bind_vertex_array(None);
        }
    }
}

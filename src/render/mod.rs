//! This module contains the OpenGL rendering pipelines.
//!
//! It's a multi-platform module, meaning it's for the most part all going to be used
//! on Web and Native targets, so we're targetting mostly OpenGL ES 3.0 features.

mod buffer;
mod error;
mod img;
mod pipeline;
mod program;
mod texture;
mod types;
mod uniform;
mod vertex_array;

pub use buffer::*;
pub use error::*;
pub use img::*;
pub use pipeline::*;
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

use vek::*;

/// Type which holds onto the OpenGL context, and the various objects that surrounds it.
pub struct Renderer {
    gl: Context,

    texture: Texture,
    vertex_array: VertexArray,
    program: Program,
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

        let images = [
            load_bytes(include_bytes!("../../assets/a.png")),
            load_bytes(include_bytes!("../../assets/b.png")),
            load_bytes(include_bytes!("../../assets/d.png")),
            load_bytes(include_bytes!("../../assets/c.png")),
        ];
        let texture = Texture::from_images(&gl, &images)?;
        
        #[rustfmt::skip]
        fn quad() -> [f32; 24] {
            [
                0.5,  0.5,   1.0, 1.0,
                0.5, -0.5,   1.0, 0.0,
               -0.5,  0.5,   0.0, 1.0,
                
                0.5, -0.5,   1.0, 0.0,
               -0.5, -0.5,   0.0, 0.0,
               -0.5,  0.5,   0.0, 1.0
            ]
        }
        
        let mut mesh: Vec<f32> = vec![];
        mesh.extend(quad().iter());
        
        let vertex_buffer_layout = Pipeline::new(
            Buffer::immutable(&gl, glow::ARRAY_BUFFER, &mesh)?,
            vec![
                VertexAttribute::new(vert_pos, 2),
                VertexAttribute::new(text_pos, 2),
            ],
        );

        let vertex_array = {
            let bindings = &[&vertex_buffer_layout];

            VertexArray::new(&gl, bindings)
        }?;

        Ok(Self {
            vertex_array,
            program,
            texture,

            gl,
        })
    }

    pub fn render<'a>(
        &mut self,
        (_ent, start, screen_size, _positions, mut _id): (
            Entities<'a>,
            Read<'a, GameStart>,
            Read<'a, ScreenSize>,
            ReadStorage<'a, Position>,
            WriteStorage<'a, RenderId>,
        ),
    ) -> Result<(), RenderError> {
        let Self { gl, .. } = self;

        // let mut pos_vec = Vec::new();
        // for (_, pos, id) in (&*ent, &positions, &mut id).join() {
        //     pos_vec.push(pos.0);

        //     if let RenderId(None) = id {
        //         *id = RenderId(Some(self.instance_buffer.next_free()?));
        //     }

        //     let x = pos.0.x;
        //     let y = pos.0.y;
        //     let z = pos.0.z;
        //     self.instance_buffer
        //         .update_slice(&gl, id.0.unwrap(), &[x, y, z]);
        // }

        let seconds = crate::units::Seconds::<f32>::from(start.0.elapsed());

        let _scale = 0.3 * seconds.0.sin();

        #[allow(dead_code)]
        let ScreenSize((w, h)) = *screen_size;

        
        // let trans = Mat4::translation_3d(Vec3::new(1., 1., 1.));
        // #[rustfmt::skip]
        // let trans = Mat4::new(1.0, 0.0, 0.0, 0.0,
        //                       0.0, 1.0, 0.0, 0.0,
        //                       0.0, 0.0, 1.0, 0.0,
        //                       0.0, 0.0, 0.0, 1.0);

        let scale = Mat4::scaling_3d(Vec3::new(0.3, 0.3, 1.0));

        let m = scale;

        unsafe {
            gl.clear(glow::COLOR_BUFFER_BIT);
            gl.viewport(0, 0, w as _, h as _);

            self.program.use_program(&gl);
            self.program.set_uniform(&gl, "transform", m);
            self.texture.bind(&gl);
            self.vertex_array.bind(&gl);
            gl.draw_arrays(glow::TRIANGLES, 0, 6);
            
            // gl.draw_elements(glow::TRIANGLES, 6, glow::UNSIGNED_INT, 0);
            gl.bind_vertex_array(None);
        }

        Ok(())
    }
}

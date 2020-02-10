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

    grid_size: Vec2<usize>,
}

impl Renderer {
    pub fn new(gl: Context) -> Result<Self, RenderError> {
        let (grid_height, grid_width) = (2, 2);
        let (vert_pos, text_pos, tile_pos, tile_size, text_idx) = (0, 1, 2, 3, 4);

        let program = Program::new(
            &gl,
            include_str!("shaders/vss.glsl"),
            include_str!("shaders/fss.glsl"),
            &[
                (vert_pos, "vert_pos"),
                (text_pos, "text_pos"),
                (tile_pos, "tile_pos"),
                (tile_size, "tile_size"),
                (text_idx, "text_idx"),
            ],
        )?;

        let images = [
            load_bytes(include_bytes!("../../assets/a.png")),
            load_bytes(include_bytes!("../../assets/b.png")),
            load_bytes(include_bytes!("../../assets/d.png")),
            load_bytes(include_bytes!("../../assets/c.png")),
        ];
        let texture = Texture::from_images(&gl, &images)?;

        let vertex_buffer_layout = {
            #[rustfmt::skip]
            fn quad(x: f32, y: f32, size: f32) -> [f32; 24] {
                let s = size;
                [
                    0.5 + x + s,  0.5 + y + s,   1.0, 1.0,
                    0.5 + x + s, -0.5 + y    ,   1.0, 0.0,
                   -0.5 + x    ,  0.5 + y + s,   0.0, 1.0,
                    0.5 + x + s, -0.5 + y    ,   1.0, 0.0,
                   -0.5 + x    , -0.5 + y    ,   0.0, 0.0,
                   -0.5 + x    ,  0.5 + y + s,   0.0, 1.0
                ]
            }

            let grid = (0..grid_height)
                .map(|x| (0..grid_width).map(move |y| (x, y)))
                .flatten();

            let mut mesh: Vec<f32> = vec![];
            for (x, y) in grid {
                mesh.extend(quad(x as _, y as _, 0.0).iter());
            }

            Pipeline::new(
                Buffer::immutable(&gl, glow::ARRAY_BUFFER, &mesh)?,
                vec![
                    VertexAttribute::new(vert_pos, 2),
                    VertexAttribute::new(text_pos, 2),
                ],
            )
        };

        let texture_array_indices = {
            #[rustfmt::skip]
            let indices: Vec<i32> = vec![0; grid_height * grid_width * 6 * 4];

            Pipeline::new(
                Buffer::immutable(&gl, glow::ARRAY_BUFFER, &indices)?,
                vec![VertexAttribute::new(text_idx, 1)],
            )
        };

        let vertex_array = VertexArray::new(&gl, |gl| {
            vertex_buffer_layout.setup(&gl);
            texture_array_indices.setup(&gl);
        })?;

        Ok(Self {
            vertex_array,
            program,
            texture,

            gl,

            grid_size: Vec2::new(grid_height, grid_width),
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
            gl.scissor(0, 0, w as _, h as _);

            self.program.use_program(&gl);
            self.program.set_uniform(&gl, "transform", m);
            self.texture.bind(&gl);
            self.vertex_array.bind(&gl);
            gl.draw_arrays(glow::TRIANGLES, 0, (6 * self.grid_size.product()) as _);

            // gl.draw_elements(glow::TRIANGLES, 6, glow::UNSIGNED_INT, 0);
            gl.bind_vertex_array(None);
        }

        Ok(())
    }
}

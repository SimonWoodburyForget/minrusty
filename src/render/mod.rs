//! This module contains the OpenGL rendering pipelines.
//!
//! It's a multi-platform module, meaning it's for the most part all going to be used
//! on Web and Native targets, so we're targetting mostly OpenGL ES 3.0 features.

mod buffer;
mod error;
mod program;
mod texture;
mod types;
mod uniform;
mod vertex_array;

pub use buffer::*;
pub use error::*;
pub use program::*;
pub use texture::*;
pub use types::*;
pub use uniform::*;
pub use vertex_array::*;

use glow::*;

use crate::components::*;
use crate::loader::*;
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
    grid_textures: Buffer<i32>,
    vert_per_quad: usize,
    vert_size: usize,

    frame: u64,
}

impl Renderer {
    pub fn new(gl: Context) -> Result<Self, RenderError> {
        let (grid_height, grid_width) = (6, 6);
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

        let texture = Texture::new(&gl, 32, 32, 6)?;

        const VERT_PER_QUAD: usize = 6;
        const VERT_SIZE: usize = 4;
        let vertex_buffer = {
            #[rustfmt::skip]
            fn quad(x: f32, y: f32, size: f32) -> [f32; VERT_PER_QUAD * VERT_SIZE] {
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

            Buffer::immutable(
                &gl,
                glow::ARRAY_BUFFER,
                &mesh,
                vec![
                    VertexAttribute::new(vert_pos, 2),
                    VertexAttribute::new(text_pos, 2),
                ],
            )?
        };

        let texture_array_indices = {
            #[rustfmt::skip]
            let indices: Vec<i32> = vec![0; grid_height * grid_width * VERT_PER_QUAD * VERT_SIZE];

            Buffer::immutable(
                &gl,
                glow::ARRAY_BUFFER,
                &indices,
                vec![VertexAttribute::new(text_idx, 1)],
            )?
        };

        let vertex_array = VertexArray::new(&gl, |gl| {
            vertex_buffer.setup(&gl);
            texture_array_indices.setup(&gl);
        })?;

        Ok(Self {
            vertex_array,
            program,
            texture,

            gl,

            grid_size: Vec2::new(grid_height, grid_width),
            grid_textures: texture_array_indices,
            vert_per_quad: VERT_PER_QUAD,
            vert_size: VERT_SIZE,

            frame: 0,
        })
    }
}

impl<'a> System<'a> for Renderer {
    type SystemData = (
        Entities<'a>,
        Read<'a, GameStart>,
        Read<'a, ScreenSize>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Coordinate>,
        ReadStorage<'a, TextureIndex>,
    );

    fn setup(&mut self, world: &mut World) {
        let loader = world.fetch::<Loader>();
        for (e, image) in loader.iter_images() {
            self.texture.update_image(&self.gl, e as u32, image);
        }
    }

    fn run(
        &mut self,
        (entities, start, screen_size, _positions, coordinates, textures): Self::SystemData,
    ) {
        let Self { gl, .. } = self;

        for (_, coord, text) in (&*entities, &coordinates, &textures).join() {
            let t = text.0.unwrap() as _;
            let (x, y) = (coord.0.x, coord.0.y);

            // TODO: refactor into a `grid model` of some kind.
            let index = (x * self.grid_size.x * self.vert_per_quad * self.vert_size)
                + y * self.vert_per_quad * self.vert_size;
            self.grid_textures
                .update(&gl, index as _, &[t, t, t, t, t, t]);
        }

        let seconds = crate::units::Seconds::<f32>::from(start.0.elapsed());

        let _scale = 0.3 * seconds.0.sin();

        #[allow(dead_code)]
        let ScreenSize((w, h)) = *screen_size;

        let scale = Mat4::scaling_3d(Vec3::new(300., 300., 1.0));

        #[rustfmt::skip]
        let frustum = {
            let (w, h) = (w as f32, h as f32);
            FrustumPlanes::<f32> {
                left: -w, right: w,
                bottom: -h, top: h,
                near: -2., far: 2.,
            }
        };

        let ortho = Mat4::orthographic_rh_no(frustum);

        let m = ortho * scale;

        unsafe {
            gl.clear(glow::COLOR_BUFFER_BIT);
            gl.viewport(0, 0, w as _, h as _);
            gl.scissor(0, 0, w as _, h as _);

            self.program.use_program(&gl);
            self.program.set_uniform(&gl, "transform", m);
            self.texture.bind(&gl);
            self.vertex_array.bind(&gl);
            gl.draw_arrays(
                glow::TRIANGLES,
                0,
                (self.vert_per_quad * self.grid_size.product()) as _,
            );

            // gl.draw_elements(glow::TRIANGLES, 6, glow::UNSIGNED_INT, 0);
            gl.bind_vertex_array(None);
        }

        self.frame += 1;
    }
}

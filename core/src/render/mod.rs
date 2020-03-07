//! This module contains the OpenGL rendering pipelines.
//!
//! It's a multi-platform module, meaning it's for the most part all going to be used
//! on Web and Native targets, so we're targetting mostly OpenGL ES 3.0 features.

mod buffer;
mod error;
mod memory;
mod program;
mod texture;
mod types;
mod uniform;

pub use buffer::*;
pub use error::*;
pub use program::*;
pub use texture::*;
pub use types::*;
pub use uniform::*;

use crate::components::*;
use crate::game::{Frame, Scene};
use crate::loader::Loader;
use crate::state::GameStart;
use memory::Pod;

use glow::*;
use instant::{Duration, Instant};
use specs::prelude::*;
use std::convert::TryInto;
use vek::*;

/// Constant vertex attribute locations used across shaders.
mod loc {
    pub const VERT_POS: u32 = 0;
    pub const TEXT_POS: u32 = 1;
    pub const TEXT_IDX: u32 = 2;
    pub const VERT_COL: u32 = 3;
}

#[derive(Copy, Clone, Default)]
#[repr(C)]
pub struct Vertex {
    /// Position of the vertex.
    pos: [f32; 2],

    /// Texture coordinates.
    tex: [f32; 2],

    /// Texture array index.
    idx: u32,

    /// Color of this vertex.
    color: [f32; 4],
}

unsafe impl Pod for Vertex {}

pub trait Pipeline {
    /// A vertex should be capable of casting itself to buffer data, and configuring
    /// the vertex attribute pointer, by knowing it's own memory layout.
    type Vertex: Copy + Pod;
}

pub struct Quad<P: Pipeline> {
    a: P::Vertex,
    b: P::Vertex,
    c: P::Vertex,
    d: P::Vertex,
}

impl<P: Pipeline> Quad<P> {
    pub fn new(a: P::Vertex, b: P::Vertex, c: P::Vertex, d: P::Vertex) -> Self {
        Quad { a, b, c, d }
    }
}

impl Quad<SpritePipeline> {
    #[rustfmt::skip]
    pub fn rect(xy: Vec2<f32>, s: f32, idx: u32, color: Rgba<f32>) -> Self {
        let [x, y] = xy.into_array();
        let color = color.into_array();
        Self::new(
            Vertex { pos: [ 0.5 + x + s,  0.5 + y + s], tex: [1.0, 1.0], idx, color },
            Vertex { pos: [ 0.5 + x + s, -0.5 + y    ], tex: [1.0, 0.0], idx, color },
            Vertex { pos: [-0.5 + x    ,  0.5 + y + s], tex: [0.0, 1.0], idx, color },
            Vertex { pos: [-0.5 + x    , -0.5 + y    ], tex: [0.0, 0.0], idx, color },
        )
    }
}

#[derive(Default)]
pub struct Mesh<P: Pipeline> {
    data: Vec<P::Vertex>,
}

impl<P: Pipeline> Mesh<P> {
    /// Clear vertices to reuse allocated memory.
    pub fn clear(&mut self) {
        self.data.clear();
    }

    /// Push a quad at the end of the mesh.
    pub fn push_quad(&mut self, quad: Quad<P>) {
        let Quad { a, b, c, d } = quad;
        self.data.extend(&[a, b, c, c, b, d]);
    }
}

#[derive(Default)]
struct SpritePipeline;
impl Pipeline for SpritePipeline {
    type Vertex = Vertex;
}

/// Type which holds onto the OpenGL context, and the various objects that surrounds it.
pub struct Renderer {
    gl: Context,

    texture: Texture,
    vertex_array: Option<VertexArrayId>,
    program: Program,
    vertex_buffer: Buffer<SpritePipeline>,

    grid_size: Vec2<usize>,
    tile_mesh: Mesh<SpritePipeline>,

    frame_duration: Vec<Duration>,
}

impl Renderer {
    pub fn new(gl: Context) -> Result<Self, RenderError> {
        let (grid_height, grid_width) = (6, 6);

        let program = Program::new(
            &gl,
            include_str!("shaders/vss.glsl"),
            include_str!("shaders/fss.glsl"),
            &[
                (loc::VERT_POS, "vert_pos"),
                (loc::TEXT_POS, "text_pos"),
                (loc::TEXT_IDX, "text_idx"),
            ],
        )?;

        let texture = Texture::new(&gl, Vec3::new(32, 32, 6))?;

        let tile_mesh = Mesh::default();

        let buffer_size = 6 * 4 * 1000 * 321; // FIXME: buffer size shouldn't be hardcoded
        let vertex_buffer = Buffer::dynamic(&gl, glow::ARRAY_BUFFER, buffer_size)?;

        let vertex_array;
        unsafe {
            vertex_array = Some(gl.create_vertex_array()?);
            gl.bind_vertex_array(vertex_array);

            vertex_buffer.bind(&gl);

            // TODO: refactor this mess
            gl.vertex_attrib_pointer_f32(
                loc::VERT_POS,
                2,
                glow::FLOAT,
                false,
                std::mem::size_of::<Vertex>().try_into().unwrap(),
                offset_of!(Vertex, pos).try_into().unwrap(),
            );
            gl.enable_vertex_attrib_array(loc::VERT_POS);

            gl.vertex_attrib_pointer_f32(
                loc::TEXT_POS,
                2,
                glow::FLOAT,
                false,
                std::mem::size_of::<Vertex>().try_into().unwrap(),
                offset_of!(Vertex, tex).try_into().unwrap(),
            );
            gl.enable_vertex_attrib_array(loc::TEXT_POS);

            gl.vertex_attrib_pointer_i32(
                loc::TEXT_IDX,
                1,
                glow::UNSIGNED_INT,
                std::mem::size_of::<Vertex>().try_into().unwrap(),
                offset_of!(Vertex, idx).try_into().unwrap(),
            );
            gl.enable_vertex_attrib_array(loc::TEXT_IDX);

            gl.vertex_attrib_pointer_f32(
                loc::VERT_COL,
                4,
                glow::FLOAT,
                false,
                std::mem::size_of::<Vertex>().try_into().unwrap(),
                offset_of!(Vertex, color).try_into().unwrap(),
            );
            gl.enable_vertex_attrib_array(loc::VERT_COL);
        }

        Ok(Self {
            vertex_array,
            program,
            texture,
            vertex_buffer,

            gl,

            tile_mesh,

            grid_size: Vec2::new(grid_height, grid_width),
            frame_duration: Vec::new(),
        })
    }
}

impl<'a> System<'a> for Renderer {
    type SystemData = (
        Entities<'a>,
        Read<'a, GameStart>,
        Read<'a, Scene>,
        ReadStorage<'a, Color>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Coordinate>,
        ReadStorage<'a, TextureIndex>,
    );

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);
        let loader = world.fetch::<Loader>();
        for (idx, image) in loader.iter_images() {
            self.texture.update_image(&self.gl, idx.try_into().unwrap(), image);
        }
    }

    fn run(
        &mut self,
        (
            entities,
            _start,
            scene,
            colors,
            _positions,
            coordinates,
            textures,
        ): Self::SystemData,
    ) {
        let Self {
            gl,
            tile_mesh,
            vertex_buffer,
            ..
        } = self;

        unsafe {
            // expected blocking operation for vsync
            gl.clear(glow::COLOR_BUFFER_BIT);
        }

        // let frame_start = Instant::now();

        // TODO: this needs a way to handle changing vertices count.
        // NOTE: this could be done much more efficiently,
        // but we're not rendering thousands of tiles yet.
        tile_mesh.clear();
        for (_, coord, text, color) in (&*entities, &coordinates, &textures, &colors).join() {
            let t = text.0.unwrap_or(0) as _;
            let v = coord.0.numcast().unwrap();
            tile_mesh.push_quad(Quad::rect(v, 0.0, t, color.0));
        }

        tile_mesh.push_quad(Quad::rect(
            scene.coordinate_cursor().numcast().unwrap(),
            0.0,
            2,
            Rgba::broadcast(0.7),
        ));

        unsafe {
            // SAFETY: safe if we don't mutate the mesh before drawing.
            vertex_buffer.update(&gl, 0, &tile_mesh.data);

            let [x, y] = scene.screen_dimentions().into_array();
            gl.viewport(0, 0, x, y);
            gl.scissor(0, 0, x, y);

            self.program.use_program(&gl);
            self.program
                .set_uniform(&gl, "transform", scene.transform());
            self.texture.bind(&gl);
            gl.bind_vertex_array(self.vertex_array);

            gl.draw_arrays(
                glow::TRIANGLES,
                0,
                (self.tile_mesh.data.len() * self.grid_size.product()) as _,
            );

            // gl.draw_elements(glow::TRIANGLES, 6, glow::UNSIGNED_INT, 0);
            gl.bind_vertex_array(None);
        }

        // let now = Instant::now();
        // let duration = now.duration_since(frame_start);
        // self.frame_duration.push(duration);

        // if frame.0 % 100 == 0 {
        //     let n: u32 = self.frame_duration.len().try_into().unwrap();
        //     let avg_duration = self
        //         .frame_duration
        //         .iter()
        //         .fold(Duration::new(0, 0), |a, b| a + *b)
        //         / n;

        //     self.frame_duration.clear();

        //     // println!("draw ({:>3} μs)", avg_duration.as_micros());

        //     let cursor = scene.world_cursor().round();
        //     // println!("cursor ({:>3}, {:>3})", cursor.x, cursor.y);
        // }
    }
}

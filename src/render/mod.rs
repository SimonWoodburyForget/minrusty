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
use crate::loader::*;
use crate::state::GameStart;
use crate::ScreenSize;
use memory::Pod;

use glow::*;
use specs::prelude::*;
use vek::*;

pub struct VertexAttribute {
    location: u32,
    size: i32,
    size_type: usize,
    data_type: u32,
}

impl VertexAttribute {
    fn new(location: u32, size: i32, size_type: usize, data_type: u32) -> Self {
        Self {
            location,
            size,
            size_type,
            data_type,
        }
    }
}

#[derive(Copy, Clone, Default)]
#[repr(C)]
pub struct Vertex {
    /// Position of the vertex.
    pos: [f32; 2],

    /// Texture coordinates.
    tex: [f32; 2],

    /// Texture array index.
    idx: i32,
}

unsafe impl Pod for Vertex {}

const VERT_POS: u32 = 0;
const TEXT_POS: u32 = 1;
const TEXT_IDX: u32 = 4;

impl Vertex {
    fn stride_size() -> usize {
        std::mem::size_of::<f32>() * 4 + std::mem::size_of::<i32>()
    }

    fn vertex_attributes() -> [VertexAttribute; 3] {
        [
            VertexAttribute::new(VERT_POS, 2, std::mem::size_of::<f32>() as _, glow::FLOAT),
            VertexAttribute::new(TEXT_POS, 2, std::mem::size_of::<f32>() as _, glow::FLOAT),
            VertexAttribute::new(TEXT_IDX, 1, std::mem::size_of::<i32>() as _, glow::INT),
        ]
    }
}

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

#[derive(Default)]
pub struct Mesh<P: Pipeline> {
    pub data: Vec<P::Vertex>,
}

impl<P: Pipeline> Mesh<P> {
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
    vert_per_quad: usize,
    vert_size: usize,

    frame: u64,
}

impl Renderer {
    pub fn new(gl: Context) -> Result<Self, RenderError> {
        let (grid_height, grid_width) = (6, 6);

        let program = Program::new(
            &gl,
            include_str!("shaders/vss.glsl"),
            include_str!("shaders/fss.glsl"),
            &[
                (VERT_POS, "vert_pos"),
                (TEXT_POS, "text_pos"),
                (TEXT_IDX, "text_idx"),
            ],
        )?;

        let texture = Texture::new(&gl, 32, 32, 6)?;

        const VERT_PER_QUAD: usize = 6;
        const VERT_SIZE: usize = 4;
        let vertex_buffer = {
            #[rustfmt::skip]
            fn quad(x: f32, y: f32, size: f32) -> Quad<SpritePipeline> {
                let s = size;
                Quad::new(
                    Vertex { pos: [ 0.5 + x + s,  0.5 + y + s], tex: [1.0, 1.0], idx: 0 },
                    Vertex { pos: [ 0.5 + x + s, -0.5 + y    ], tex: [1.0, 0.0], idx: 0 },
                    Vertex { pos: [-0.5 + x    ,  0.5 + y + s], tex: [0.0, 1.0], idx: 0 },
                    Vertex { pos: [-0.5 + x    , -0.5 + y    ], tex: [0.0, 0.0], idx: 0 },
                )
            }

            let grid = (0..grid_height)
                .map(|x| (0..grid_width).map(move |y| (x, y)))
                .flatten();

            let mut mesh: Mesh<SpritePipeline> = Mesh::default();
            for (x, y) in grid {
                mesh.push_quad(quad(x as _, y as _, 0.0));
            }

            Buffer::immutable(&gl, glow::ARRAY_BUFFER, &mesh.data)
        }?;

        let vertex_array;
        unsafe {
            vertex_array = Some(gl.create_vertex_array()?);
            gl.bind_vertex_array(vertex_array);

            vertex_buffer.bind(&gl);
            let mut offset = 0;
            for attr in Vertex::vertex_attributes().iter() {
                gl.vertex_attrib_pointer_f32(
                    attr.location,
                    attr.size,
                    attr.data_type,
                    false,
                    Vertex::stride_size() as _,
                    offset,
                );
                gl.enable_vertex_attrib_array(attr.location);
                offset += attr.size * attr.size_type as i32;
            }
        }

        Ok(Self {
            vertex_array,
            program,
            texture,
            vertex_buffer,

            gl,

            grid_size: Vec2::new(grid_height, grid_width),
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

        // for (_, coord, text) in (&*entities, &coordinates, &textures).join() {
        //     let t = text.0.unwrap() as _;
        //     let (x, y) = (coord.0.x, coord.0.y);

        //     // TODO: refactor into a `grid model` of some kind.
        //     let index = (x * self.grid_size.x * self.vert_per_quad * self.vert_size)
        //         + y * self.vert_per_quad * self.vert_size;
        //     self.grid_textures.update(&gl, index as _, &[t; 6]);
        // }

        let seconds = crate::units::Seconds::<f32>::from(start.0.elapsed());

        let _scale = 0.3 * seconds.0.sin();

        let ScreenSize((w, h)) = *screen_size;

        let scale: Mat4<f32> = Mat4::scaling_3d(Vec3::new(300., 300., 1.0));

        #[rustfmt::skip]
        let frustum = {
            let (w, h) = (w as f32, h as f32);
            FrustumPlanes::<f32> {
                left: 0.0, right: w,
                bottom: 0.0, top: h,
                near: -10., far: 10.,
            }
        };

        let ortho = Mat4::frustum_rh_no(frustum);

        let x = -1.0 + 0.3 * seconds.0.sin();
        let y = -1.0 + 0.3 * seconds.0.cos();

        #[rustfmt::skip]
        let movit = Mat4::new(
            1.0, 0.0, 0.0, 0.0,
            0.0, 1.0, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            x, y, 0.0, 1.0,
        );

        let m = movit * ortho * scale;

        unsafe {
            gl.clear(glow::COLOR_BUFFER_BIT);
            gl.viewport(0, 0, w as _, h as _);
            gl.scissor(0, 0, w as _, h as _);

            self.program.use_program(&gl);
            self.program.set_uniform(&gl, "transform", m);
            self.texture.bind(&gl);
            gl.bind_vertex_array(self.vertex_array);
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

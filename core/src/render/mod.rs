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
use crate::game::{CursorState, ScreenSize, UniversePosition};
use crate::loader::Loader;
use crate::state::GameStart;
use memory::Pod;

use glow::*;
use instant::{Duration, Instant};
use specs::prelude::*;
use std::convert::TryInto;
use vek::*;

#[allow(dead_code)]
pub enum DataType {
    Float,
    Int,
    Uint,
}

impl DataType {
    fn size(&self) -> i32 {
        match self {
            DataType::Float => 4,
            DataType::Int => 4,
            DataType::Uint => 4,
        }
    }

    fn data_type(&self) -> u32 {
        match self {
            DataType::Float => glow::FLOAT,
            DataType::Int => glow::INT,
            DataType::Uint => glow::UNSIGNED_INT,
        }
    }

    fn normalize(&self) -> bool {
        false
    }
}

pub struct VertexAttribute {
    location: u32,
    size: i32,
    size_type: i32,
    data_type: u32,
    norm: bool,
}

impl VertexAttribute {
    fn new(location: u32, size: i32, dtype: DataType) -> Self {
        Self {
            location,
            size,
            size_type: dtype.size(),
            data_type: dtype.data_type(),
            norm: dtype.normalize(),
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
    idx: u32,

    /// Color of this vertex.
    color: [f32; 4],
}

unsafe impl Pod for Vertex {}

const VERT_POS: u32 = 0;
const TEXT_POS: u32 = 1;
const TEXT_IDX: u32 = 2;
const VERT_COL: u32 = 3;

impl Vertex {
    fn stride_size() -> usize {
        std::mem::size_of::<Self>()
    }

    fn vertex_attributes() -> [VertexAttribute; 4] {
        [
            VertexAttribute::new(VERT_POS, 2, DataType::Float),
            VertexAttribute::new(TEXT_POS, 2, DataType::Float),
            // FIXME: `Float` works but `Uint` doesn't
            VertexAttribute::new(TEXT_IDX, 1, DataType::Float),
            VertexAttribute::new(VERT_COL, 4, DataType::Float),
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

impl Quad<SpritePipeline> {
    #[rustfmt::skip]
    pub fn rect(xy: Vec2<f32>, size: f32, idx: u32, color: Rgba<f32>) -> Self {
        let [x, y] = xy.into_array();
        let color = color.into_array();
        let s = size;
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

    frame: u64,
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
                (VERT_POS, "vert_pos"),
                (TEXT_POS, "text_pos"),
                (TEXT_IDX, "text_idx"),
            ],
        )?;

        let texture = Texture::new(&gl, 32, 32, 6)?;

        //     let grid = (0..grid_height)
        //         .map(|x| (0..grid_width).map(move |y| (x, y)))
        //         .flatten();

        //     for (x, y) in grid {
        //         tile_mesh.push_quad(Quad::rect(x as _, y as _, 0.0, 0));
        //     }

        let tile_mesh = Mesh::default();

        let buffer_size = 6 * 4 * 1000;
        let vertex_buffer = Buffer::dynamic(&gl, glow::ARRAY_BUFFER, buffer_size)?;

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
                    attr.norm,
                    Vertex::stride_size() as _,
                    offset,
                );
                gl.enable_vertex_attrib_array(attr.location);
                offset += attr.size * attr.size_type;
            }
        }

        Ok(Self {
            vertex_array,
            program,
            texture,
            vertex_buffer,

            gl,

            tile_mesh,

            grid_size: Vec2::new(grid_height, grid_width),

            frame: 0,
            frame_duration: Vec::new(),
        })
    }
}

impl<'a> System<'a> for Renderer {
    type SystemData = (
        Entities<'a>,
        Read<'a, GameStart>,
        Read<'a, ScreenSize>,
        Read<'a, CursorState>,
        Read<'a, UniversePosition>,
        ReadStorage<'a, Color>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Coordinate>,
        ReadStorage<'a, TextureIndex>,
    );

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);
        let loader = world.fetch::<Loader>();
        for (e, image) in loader.iter_images() {
            self.texture.update_image(&self.gl, e as u32, image);
        }
    }

    fn run(
        &mut self,
        (
            entities,
            _start,
            screen_size,
            _cursor_state,
            universe_position,
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
            // here to avoid timing vsync blocking time
            gl.clear(glow::COLOR_BUFFER_BIT);
        }

        let frame_start = Instant::now();
        tile_mesh.clear();

        // TODO: this needs a way to handle changing vertices count.
        // NOTE: this could be done much more efficiently,
        // but we're not rendering thousands of tiles yet.
        for (_, coord, text, color) in (&*entities, &coordinates, &textures, &colors).join() {
            let t = text.0.unwrap_or(0) as _;
            let (x, y) = (coord.0.x as _, coord.0.y as _);
            let v = Vec2::new(x, y);
            tile_mesh.push_quad(Quad::rect(v, 0.0, t, color.0));
        }

        unsafe {
            // SAFETY: safe if we don't mutate the mesh before drawing.
            vertex_buffer.update(&gl, 0, &tile_mesh.data)
        };

        // let seconds = crate::units::Seconds::<f32>::from(start.0.elapsed());

        let [w, h] = screen_size.0.into_array();
        let (w, h) = (w as f32, h as f32);

        let scale: Mat4<f32> = Mat4::scaling_3d(Vec3::new(300., 300., 1.0));

        #[rustfmt::skip]
        let frustum = {
            FrustumPlanes::<f32> {
                left: 0.0, right: w,
                bottom: 0.0, top: h,
                near: -10., far: 10.,
            }
        };

        let ortho = Mat4::frustum_rh_no(frustum);

        // let (x, y) = {
        //     let [a, b] = cursor_state.0.into_array();
        //     let (a, b) = (a as f32, b as f32);
        //     let v = Vec2::new(a, b) / Vec2::new(w, h);
        //     (-1.0 + 0.3 * v.x, -1.0 + 0.3 * v.y)
        // };

        // let x = -1.0 + 0.3 * seconds.0.sin();
        // let y = -1.0 + 0.3 * seconds.0.cos();

        let [x, y] = universe_position.0.into_array();

        #[rustfmt::skip]
        let movit = Mat4::new(
            1.0, 0.0, 0.0, 0.0,
            0.0, 1.0, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
              x,   y, 0.0, 1.0,
        );

        let m = movit * ortho * scale;

        unsafe {
            gl.viewport(0, 0, w as _, h as _);
            gl.scissor(0, 0, w as _, h as _);

            self.program.use_program(&gl);
            self.program.set_uniform(&gl, "transform", m);
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

        let now = Instant::now();
        let duration = now.duration_since(frame_start);
        self.frame_duration.push(duration);

        if self.frame % 100 == 0 {
            let n: u32 = self.frame_duration.len().try_into().unwrap();
            let avg_duration = self
                .frame_duration
                .iter()
                .fold(Duration::new(0, 0), |a, b| a + *b)
                / n;

            self.frame_duration.clear();

            println!(
                "draw {:6} ({:6} -- {:>16})",
                self.frame,
                "",
                &format!("{}", humantime::format_duration(avg_duration)),
            );
        }

        self.frame += 1;
    }
}

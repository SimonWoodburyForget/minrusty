//! This module contains thin OpenGL wrappers for rendering data.

mod error;
mod program;
mod types;
mod uniform;
mod vertex_array;

pub use error::*;
pub use program::*;
pub use types::*;
pub use uniform::*;
pub use vertex_array::*;

use glow::*;

use image::io::Reader;
use image::DynamicImage;
use image::ImageFormat;
use std::io::Cursor;

/// Type for handling all GPU operations.
pub struct Renderer {
    gl: Context,
    square: Square,
}

impl Renderer {
    pub fn new(gl: Context) -> Result<Self, RenderError> {
        // loading image at compile time, because we can
        Ok(Self {
            square: Square::new(&gl)?,
            gl: gl,
        })
    }

    pub fn draw(&self, green: f32) {
        self.square.draw(&self.gl, green);
    }
}

pub struct Texture {
    tex: TextureId,
}

impl Texture {
    /// Creates a texture from an image.
    pub fn new(gl: &Context, image: DynamicImage) -> Result<Self, RenderError> {
        let rgb = image.to_rgba();
        let (width, height) = rgb.dimensions();
        let bytes = rgb.into_raw(); // is this correct?

        unsafe {
            let tex = gl.create_texture()?;
            gl.active_texture(glow::TEXTURE0);
            gl.bind_texture(glow::TEXTURE_2D, Some(tex));

            gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_WRAP_S, glow::REPEAT as i32);
            gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_WRAP_T, glow::REPEAT as i32);
            gl.tex_parameter_i32(
                glow::TEXTURE_2D,
                glow::TEXTURE_MIN_FILTER,
                glow::LINEAR as i32,
            );
            gl.tex_parameter_i32(
                glow::TEXTURE_2D,
                glow::TEXTURE_MAG_FILTER,
                glow::LINEAR as i32,
            );

            // TODO:
            // - look into tex_storage_2d
            gl.tex_image_2d(
                glow::TEXTURE_2D,
                0,
                glow::RGBA as i32, // wat
                width as i32,
                height as i32,
                0,
                glow::RGBA,
                glow::UNSIGNED_BYTE,
                Some(&bytes.align_to::<u8>().1),
            );
            gl.generate_mipmap(glow::TEXTURE_2D);

            Ok(Self { tex })
        }
    }

    pub unsafe fn bind(&self, gl: &Context) {
        gl.active_texture(glow::TEXTURE0);
        gl.bind_texture(glow::TEXTURE_2D, Some(self.tex));
    }
}

/// A ..Square renderer
pub struct Square {
    tx: Texture,
    va: VertexArray,
    pg: Program,
}

impl Square {
    pub fn new(gl: &Context) -> Result<Self, RenderError> {
        // TODO:
        // - this is pretty unsound

        let raw_data = include_bytes!("../../assets/core-shard.png");
        let mut reader = Reader::new(Cursor::new(raw_data.as_ref()))
            .with_guessed_format()
            .expect("Cursor io never fails!");
        let image = reader.decode().unwrap();

        #[rustfmt::skip]
        let vertices: [f32; 32] = [
             // pos            // col           // tex
             0.5,  0.5, 0.0,   1.0, 0.0, 0.0,   1.0,  1.0, // top right
             0.5, -0.5, 0.0,   0.0, 1.0, 0.0,   1.0,  0.0, // bottom right
            -0.5,  0.5, 0.0,   0.0, 0.0, 1.0,   0.0,  1.0, // top left
            -0.5, -0.5, 0.0,   1.0, 1.0, 0.0,   0.0,  0.0, // bottom left
        ];

        #[rustfmt::skip]
        let indices: [u32; 6] = [
            0, 1, 2, // top right triangle
            2, 3, 1, // buttom left triangle
        ];

        Ok(Self {
            va: VertexArray::new(&gl, &vertices, &indices)?,
            tx: Texture::new(&gl, image)?,
            pg: Program::new(
                &gl,
                include_str!("shaders/vss.glsl"),
                include_str!("shaders/fss.glsl"),
            )?,
        })
    }

    pub fn draw(&self, gl: &Context, green: f32) {
        let Self { va, pg, tx } = self;
        unsafe {
            gl.clear(glow::COLOR_BUFFER_BIT);

            pg.use_program(&gl);
            // pg.set_uniform(&gl, "ourColor", Vec4::new(0.0, green, 0.0, 1.0));
            tx.bind(&gl);
            va.bind(&gl);
            // gl.draw_arrays(glow::TRIANGLES, 0, 6);
            gl.draw_elements(glow::TRIANGLES, 6, glow::UNSIGNED_INT, 0);
            gl.bind_vertex_array(None);
        }
    }
}

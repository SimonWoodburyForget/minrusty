use super::error::*;
use super::types::TextureId;

use glow::*;
use image::DynamicImage;

pub struct Texture {
    texture_id: TextureId,
    width: u32,
    height: u32,
    depth: u32,
}

impl Texture {
    /// Creates a texture array for n-images of a specific size.
    pub fn new(gl: &Context, width: u32, height: u32, depth: u32) -> Result<Self, RenderError> {
        let texture_id = unsafe { gl.create_texture() }?;

        Ok(Self {
            texture_id,
            width,
            height,
            depth,
        })
    }

    /// Creates a texture array from dynamic image.
    pub fn from_images(gl: &Context, images: &[DynamicImage]) -> Result<Self, RenderError> {
        let size = images
            .iter()
            .map(|dimg| dimg.to_rgba())
            .fold(Ok(None), |result, image| match result {
                Ok(some_size) => match some_size {
                    Some(size) if size == image.dimensions() => Ok(Some(size)),
                    Some(size) if size != image.dimensions() => Err(RenderError::ImageVaryingSize),
                    Some(_) => unreachable!(),
                    None => Ok(Some(image.dimensions())),
                },
                e => e,
            })?
            // size can only be None if there are no images
            .expect("tried to load no images");

        let texture = Self::new(&gl, size.0, size.1, images.len() as u32)?;

        let bytes = images
            .iter()
            .map(|image| image.to_rgba().into_raw())
            .flatten()
            .collect::<Vec<u8>>();

        unsafe { texture.load(&gl, &bytes) };

        Ok(texture)
    }

    unsafe fn load(&self, gl: &Context, data: &[u8]) {
        self.bind(&gl);
        gl.tex_image_3d(
            glow::TEXTURE_2D_ARRAY,
            0,
            glow::RGBA as i32,
            self.width as i32,
            self.height as i32,
            self.depth as i32,
            0,
            glow::RGBA,
            glow::UNSIGNED_BYTE,
            Some(&data.align_to::<u8>().1),
        );
        gl.generate_mipmap(glow::TEXTURE_2D_ARRAY);
        self.pixelated(&gl);
    }

    unsafe fn pixelated(&self, gl: &Context) {
        gl.tex_parameter_i32(
            glow::TEXTURE_2D_ARRAY,
            glow::TEXTURE_MIN_FILTER,
            glow::NEAREST_MIPMAP_LINEAR as i32,
        );
        gl.tex_parameter_i32(
            glow::TEXTURE_2D_ARRAY,
            glow::TEXTURE_MAG_FILTER,
            glow::NEAREST as i32,
        );
        gl.tex_parameter_i32(glow::TEXTURE_2D_ARRAY, glow::TEXTURE_MAX_LEVEL, 4);
        gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_WRAP_S, glow::REPEAT as i32);
        gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_WRAP_T, glow::REPEAT as i32);
    }

    pub unsafe fn bind(&self, gl: &Context) {
        gl.active_texture(glow::TEXTURE0);
        gl.bind_texture(glow::TEXTURE_2D_ARRAY, Some(self.texture_id));
    }
}

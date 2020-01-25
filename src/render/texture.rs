use super::error::*;
use super::types::TextureId;

use glow::*;
use image::DynamicImage;

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

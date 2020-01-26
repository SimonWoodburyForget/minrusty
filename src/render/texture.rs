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
        let bytes = rgb.into_raw();

        unsafe {
            let tex = gl.create_texture()?;

            gl.active_texture(glow::TEXTURE0);
            gl.bind_texture(glow::TEXTURE_2D_ARRAY, Some(tex));

            gl.tex_image_3d(
                glow::TEXTURE_2D_ARRAY,
                0,
                glow::RGBA as i32,
                width as i32,
                height as i32,
                1,
                0,
                glow::RGBA,
                glow::UNSIGNED_BYTE,
                Some(&bytes.align_to::<u8>().1),
            );

            gl.generate_mipmap(glow::TEXTURE_2D_ARRAY);
            pixelated(&gl);

            Ok(Self { tex })
        }
    }

    pub unsafe fn bind(&self, gl: &Context) {
        gl.active_texture(glow::TEXTURE0);
        gl.bind_texture(glow::TEXTURE_2D_ARRAY, Some(self.tex));
    }
}

use super::error::*;
use super::types::TextureId;

use glow::*;
use image::DynamicImage;
use std::convert::TryInto;
use vek::*;

#[derive(Clone, Copy)]
enum Type {
    Texture2dArray,
    Texture2d,
}

impl Type {
    fn into_gl(&self) -> u32 {
        match self {
            Type::Texture2dArray => glow::TEXTURE_2D_ARRAY,
            Type::Texture2d => glow::TEXTURE_2D,
        }
    }
}

#[derive(Clone, Copy)]
pub struct Texture {
    texture_id: Option<TextureId>,
    texture_type: Type,
    size: Vec3<i32>,
    slot: u32,
    level: i32,
}

impl Texture {
    pub fn _delete(&self, gl: &Context) {
        if let Some(id) = self.texture_id {
            unsafe { gl.delete_texture(id) }
        }
    }

    /// Creates a texture array for n-images of a specific size.
    pub fn new(gl: &Context, size: Vec3<u32>) -> Result<Self, RenderError> {
        let texture_type = Type::Texture2dArray;
        let size = size.numcast().unwrap();
        let level = 0;
        let border = 0;
        let slot = 0;

        let texture_id;
        #[rustfmt::skip]
        unsafe {
            texture_id = Some(gl.create_texture()?);

            gl.active_texture(glow::TEXTURE0 + slot);
            gl.bind_texture(texture_type.into_gl(), texture_id);

            gl.tex_image_3d(
                texture_type.into_gl(),
                level,
                glow::RGBA as i32,
                size.x,
                size.y,
                size.z,
                border,
                glow::RGBA,
                glow::UNSIGNED_BYTE,
                None,
            );

            // gl.pixel_store_i32(glow::PACK_ALIGNMENT, 1);

            gl.generate_mipmap(texture_type.into_gl());

            gl.tex_parameter_i32(
                texture_type.into_gl(),
                glow::TEXTURE_MIN_FILTER,
                glow::NEAREST_MIPMAP_LINEAR as i32,
            );

            gl.tex_parameter_i32(
                texture_type.into_gl(),
                glow::TEXTURE_MAG_FILTER,
                glow::NEAREST as i32,
            );

            gl.tex_parameter_i32(
                texture_type.into_gl(),
                glow::TEXTURE_MAX_LEVEL,
                4
            );

            gl.tex_parameter_i32(
                texture_type.into_gl(),
                glow::TEXTURE_WRAP_S,
                glow::REPEAT as i32,
            );

            gl.tex_parameter_i32(
                texture_type.into_gl(),
                glow::TEXTURE_WRAP_T,
                glow::REPEAT as i32,
            );
        };

        Ok(Self {
            level,
            texture_id,
            size,
            slot,
            texture_type,
        })
    }

    /// Loads an image into the texture array.
    ///
    /// # Panic
    ///
    /// - image must be exactly the same dimensions as the texture array,
    /// - image depth must not exceed texture array depth,
    pub fn update_image(&self, gl: &Context, depth: u32, image: &DynamicImage) {
        let rgb = image.to_rgba();
        let (width, height) = rgb.dimensions();

        let raw = &rgb.into_raw();
        assert_eq!(raw.len(), (self.size.x * self.size.y * 4) as usize);
        assert!(self.size.z > depth.try_into().unwrap());

        let pos1 = Vec3::new(0, 0, depth).numcast().unwrap();
        let size = Vec3::new(width, height, 1).numcast().unwrap();
        let pos2 = pos1 + size;

        unsafe {
            self.bind(&gl);

            gl.tex_sub_image_3d_u8_slice(
                self.texture_type.into_gl(),
                self.level,
                pos1.x,
                pos1.y,
                pos1.z,
                pos2.x,
                pos2.y,
                pos2.z,
                glow::RGBA,
                glow::UNSIGNED_BYTE,
                Some(&raw),
            );
        }
    }

    pub fn bind(&self, gl: &Context) {
        unsafe {
            gl.active_texture(glow::TEXTURE0 + self.slot);
            gl.bind_texture(self.texture_type.into_gl(), self.texture_id);
        }
    }
}

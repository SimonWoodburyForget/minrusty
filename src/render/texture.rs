use super::error::*;
use super::types::TextureId;

use glow::*;
use image::DynamicImage;

#[derive(Clone, Copy, Debug, Default)]
pub struct Texture {
    texture_id: Option<TextureId>,
    width: u32,
    height: u32,
    depth: u32,
    slot: u32,
}

impl Texture {
    pub fn _delete(&self, gl: &Context) {
        if let Some(id) = self.texture_id {
            unsafe { gl.delete_texture(id) }
        }
    }

    /// Creates a texture array for n-images of a specific size.
    pub fn new(gl: &Context, width: u32, height: u32, depth: u32) -> Result<Self, RenderError> {
        let texture_id;
        unsafe {
            texture_id = Some(gl.create_texture()?);

            gl.active_texture(glow::TEXTURE0);
            gl.bind_texture(glow::TEXTURE_2D_ARRAY, texture_id);

            gl.tex_image_3d(
                glow::TEXTURE_2D_ARRAY,
                0,
                glow::RGBA as i32,
                width as i32,
                height as i32,
                depth as i32 + 1,
                0,
                glow::RGBA,
                glow::UNSIGNED_BYTE,
                None,
            );

            gl.generate_mipmap(glow::TEXTURE_2D_ARRAY);

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
        };

        Ok(Self {
            texture_id,
            width,
            height,
            depth,
            slot: 0,
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

        assert!(self.width == width);
        assert!(self.height == height);
        assert!(self.depth > depth);

        unsafe {
            self.bind(&gl);

            gl.tex_sub_image_3d_u8_slice(
                glow::TEXTURE_2D_ARRAY,
                0,
                0,
                0,
                depth as i32,
                self.width as i32,
                self.height as i32,
                depth as i32 + 1,
                glow::RGBA,
                glow::UNSIGNED_BYTE,
                Some(&rgb.into_raw()),
            );
        }
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

    pub fn bind(&self, gl: &Context) {
        unsafe {
            gl.active_texture(glow::TEXTURE0 + self.slot);
            gl.bind_texture(glow::TEXTURE_2D_ARRAY, self.texture_id);
        }
    }

    pub fn _update(&self, gl: &Context, bytes: &[u8]) {
        assert_eq!(
            self.width as usize * self.height as usize * self.depth as usize * 4,
            bytes.len()
        );

        self.bind(&gl);

        unsafe {
            gl.tex_sub_image_3d_u8_slice(
                glow::TEXTURE_2D_ARRAY,
                0,
                0,
                0,
                0,
                self.width as _,
                self.height as _,
                self.depth as _,
                glow::RGBA,
                glow::UNSIGNED_BYTE,
                Some(bytes),
            );
        }
    }
}

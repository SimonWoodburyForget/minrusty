use super::*;

use glow::*;

/// Trait for values to be used as uniforms. This exists mostly as a simple wrapper
/// to match function signatures to their corresponding types.
pub trait Uniform {
    unsafe fn set_as_uniform(&self, gl: &Context, location: Option<UniformLocation>);
}

impl Uniform for f32 {
    unsafe fn set_as_uniform(&self, gl: &Context, location: Option<UniformLocation>) {
        gl.uniform_1_f32(location, *self);
    }
}

impl Uniform for vek::Vec3<f32> {
    unsafe fn set_as_uniform(&self, gl: &Context, location: Option<UniformLocation>) {
        gl.uniform_3_f32_slice(location, &self.into_array());
    }
}

impl Uniform for vek::Vec4<f32> {
    unsafe fn set_as_uniform(&self, gl: &Context, location: Option<UniformLocation>) {
        gl.uniform_4_f32_slice(location, &self.into_array());
    }
}

impl Uniform for vek::Mat4<f32> {
    unsafe fn set_as_uniform(&self, gl: &Context, location: Option<UniformLocation>) {
        gl.uniform_matrix_4_f32_slice(location, false, &self.into_row_array());
    }
}

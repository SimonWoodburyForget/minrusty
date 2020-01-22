use crate::render::program::Program;
use crate::render::types::ProgramId;

use glow::*;
use vek::{Vec3, Vec4};

/// Trait for values to be used as uniforms. This exists mostly as a simple wrapper
/// to match function signatures to their corresponding types.
pub trait Uniform {
    unsafe fn set_as_uniform(&self, gl: &Context, pg: &ProgramId, name: &str);
}

impl Uniform for f32 {
    unsafe fn set_as_uniform(&self, gl: &Context, pg: &ProgramId, name: &str) {
        gl.uniform_1_f32(gl.get_uniform_location(*pg, name), *self);
    }
}

impl Uniform for Vec3<f32> {
    unsafe fn set_as_uniform(&self, gl: &Context, pg: &ProgramId, name: &str) {
        gl.uniform_3_f32(gl.get_uniform_location(*pg, name), self.x, self.y, self.z);
    }
}

impl Uniform for Vec4<f32> {
    unsafe fn set_as_uniform(&self, gl: &Context, pg: &ProgramId, name: &str) {
        gl.uniform_4_f32(
            gl.get_uniform_location(*pg, name),
            self.x,
            self.y,
            self.z,
            self.w,
        );
    }
}

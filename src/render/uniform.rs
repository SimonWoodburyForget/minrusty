use crate::render::program::Program;

use glow::*;
use vek::{Vec3, Vec4};

/// Trait for implementing generic type bindings to OpenGL function signatures.
pub trait UniformSetter<T> {
    unsafe fn set_uniform(&self, gl: &Context, name: &str, value: T);
}

impl UniformSetter<f32> for Program {
    unsafe fn set_uniform(&self, gl: &Context, name: &str, value: f32) {
        gl.uniform_1_f32(gl.get_uniform_location(self.pg, name), value);
    }
}

impl UniformSetter<Vec4<f32>> for Program {
    unsafe fn set_uniform(&self, gl: &Context, name: &str, value: Vec4<f32>) {
        gl.uniform_4_f32(
            gl.get_uniform_location(self.pg, name),
            value.x,
            value.y,
            value.z,
            value.w,
        );
    }
}

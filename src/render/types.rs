#![allow(dead_code)]

#[cfg(feature = "web")]
use web_sys;

#[cfg(feature = "nat")]
pub type ShaderId = u32;
#[cfg(feature = "nat")]
pub type ProgramId = u32;
#[cfg(feature = "nat")]
pub type BufferId = u32;
#[cfg(feature = "nat")]
pub type VertexArrayId = u32;
#[cfg(feature = "nat")]
pub type TextureId = u32;
#[cfg(feature = "nat")]
pub type UniformLocation = u32;

#[cfg(feature = "web")]
pub type ShaderId = glow::WebShaderKey;
#[cfg(feature = "web")]
pub type ProgramId = glow::WebProgramKey;
#[cfg(feature = "web")]
pub type BufferId = glow::WebBufferKey;
#[cfg(feature = "web")]
pub type VertexArrayId = glow::WebVertexArrayKey;
#[cfg(feature = "web")]
pub type TextureId = glow::WebTextureKey;
#[cfg(feature = "web")]
pub type UniformLocation = web_sys::WebGlUniformLocation;

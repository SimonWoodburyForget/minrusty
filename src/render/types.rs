use glow::*;
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
pub type ShaderId = WebShaderKey;
#[cfg(feature = "web")]
pub type ProgramId = WebProgramKey;
#[cfg(feature = "web")]
pub type BufferId = WebBufferKey;
#[cfg(feature = "web")]
pub type VertexArrayId = WebVertexArrayKey;
#[cfg(feature = "web")]
pub type TextureId = WebTextureKey;
#[cfg(feature = "web")]
pub type UniformLocation = web_sys::WebGlUniformLocation;

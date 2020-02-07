//! gfx-hal convinience namespace

pub use gfx_backend_gl as backend;

pub use gfx_hal::{
    buffer, command, format as f,
    format::{AsFormat, ChannelType, Rgba8Srgb as ColorFormat, Swizzle},
    image as i, memory as m, pass,
    pass::Subpass,
    pool,
    prelude::*,
    pso,
    pso::{PipelineStage, ShaderStageFlags, VertexInputRate},
    queue::{QueueGroup, Submission},
    window,
};

pub use gfx_hal as hal;

pub use std::{
    borrow::Borrow,
    io::Cursor,
    iter,
    mem::{self, ManuallyDrop},
    ptr,
};

#[derive(Clone, Debug)]
pub enum RenderError {
    /// A raw untyped message from OpenGL.
    Message(String),

    /// Image failed to load because of varying sizes.
    ImageVaryingSize,

    /// Data allocated for a statically sized buffer is now full.
    BufferFull,
}

impl From<String> for RenderError {
    fn from(err: String) -> Self {
        RenderError::Message(err)
    }
}

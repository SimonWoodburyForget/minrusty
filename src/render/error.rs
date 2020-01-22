#[derive(Clone, Debug)]
pub enum RenderError {
    /// A raw untyped message from OpenGL.
    Message(String),
}

impl From<String> for RenderError {
    fn from(err: String) -> Self {
        RenderError::Message(err)
    }
}

use crate::render::RenderError;

#[derive(Clone, Debug)]
pub enum Error {
    RenderError(RenderError),
}

impl From<RenderError> for Error {
    fn from(err: RenderError) -> Self {
        Error::RenderError(err)
    }
}

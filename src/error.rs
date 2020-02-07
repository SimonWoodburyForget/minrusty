/// TODO
#[derive(Clone, Debug)]
pub struct RenderError;

#[derive(Clone, Debug)]
pub enum Error {
    RenderError(RenderError),
}

impl From<RenderError> for Error {
    fn from(err: RenderError) -> Self {
        Error::RenderError(err)
    }
}

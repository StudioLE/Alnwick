use crate::prelude::*;

/// Errors from [`AddHandler`].
#[derive(Clone, Debug, Error)]
pub enum AddError {
    #[error("Unable to parse feed")]
    Parse,
    #[error("Unable to save podcast")]
    Save,
}

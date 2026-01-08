use crate::prelude::*;

/// Errors from [`AddHandler`].
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum AddError {
    #[error("Unable to parse feed")]
    Parse,
    #[error("Unable to save podcast")]
    Save,
}

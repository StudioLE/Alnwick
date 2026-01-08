use crate::prelude::*;

/// Errors from reading metadata files.
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum GetMetadataError {
    #[error("Podcast not found")]
    NotFound,
    #[error("Unable to open file")]
    Open,
    #[error("Unable to deserialize file")]
    Deserialize,
}

/// Errors from writing metadata files.
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum PutMetadataError {
    #[error("Unable to create file")]
    Create,
    #[error("Unable to serialize file")]
    Serialize,
}

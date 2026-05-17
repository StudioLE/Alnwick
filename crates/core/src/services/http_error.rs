use crate::prelude::*;
use reqwest::StatusCode;

/// Errors from HTTP operations.
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum HttpError {
    #[error("Unexpected response status: {}", get_reason(&{0}))]
    Status(u16),
    #[error("A request error occurred")]
    Request,
    #[error("Failed to read response chunk")]
    Chunk,
    #[error("Unable to deserialize")]
    Deserialize,
    #[error("Unable to create destination directory")]
    CreateDestinationDirectory,
    #[error("Unable to write to destination file")]
    WriteDestination,
    #[error("File size is zero")]
    Size,
    #[error("Unable to remove existing file")]
    RemoveExisting,
}

fn get_reason(number: &u16) -> &str {
    StatusCode::from_u16(*number)
        .map(|e| e.canonical_reason())
        .ok()
        .flatten()
        .unwrap_or_default()
}

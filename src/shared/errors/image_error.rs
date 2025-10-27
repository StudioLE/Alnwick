use crate::prelude::*;

#[derive(Debug)]
#[allow(clippy::absolute_paths)]
pub enum ImageError {
    IO(std::io::Error),
    UnknownFormat,
    Image(image::error::ImageError),
    ImageBuffer(fast_image_resize::ImageBufferError),
    Resize(fast_image_resize::ResizeError),
    UnsupportedFormat(image::ImageFormat),
}

impl Display for ImageError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let message = match self {
            ImageError::IO(e) => {
                format!("An I/O error occurred: {e}")
            }
            ImageError::UnknownFormat => "Unable to determine image format".to_owned(),
            ImageError::Image(e) => {
                format!("An image error occurred: {e}")
            }
            ImageError::ImageBuffer(e) => {
                format!("An image buffer error occurred: {e}")
            }
            ImageError::Resize(e) => {
                format!("A resize error occurred: {e}")
            }
            ImageError::UnsupportedFormat(format) => {
                format!("Unable to encode image format: {format:?}")
            }
        };
        write!(f, "{message}")
    }
}

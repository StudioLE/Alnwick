use crate::prelude::*;

/// Fixed image sizes useful for avatars.
///
/// An implementation of
/// [Bulma fixed square image sizes](https://bulma.io/documentation/elements/image/#fixed-square-images).
#[derive(Clone, Debug, PartialEq)]
pub enum ImageSize {
    _16,
    _24,
    _32,
    _48,
    _64,
    _96,
    _128,
}

impl ImageSize {
    #[must_use]
    pub fn as_integer(&self) -> u8 {
        match self {
            ImageSize::_16 => 16,
            ImageSize::_24 => 24,
            ImageSize::_32 => 32,
            ImageSize::_48 => 48,
            ImageSize::_64 => 64,
            ImageSize::_96 => 96,
            ImageSize::_128 => 128,
        }
    }

    #[must_use]
    pub fn get_class(&self) -> String {
        let num = self.as_integer();
        format!("is-{num}x{num}")
    }
}

impl Display for ImageSize {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.as_integer())
    }
}

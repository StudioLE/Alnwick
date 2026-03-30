use crate::prelude::*;

/// Paths to committed sample fixture files in `tests/samples/`.
pub struct SampleFixtures;

impl SampleFixtures {
    /// 1-second 440Hz sine wave MP3.
    #[must_use]
    pub fn mp3() -> PathBuf {
        PathBuf::from(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/samples/sample.mp3"
        ))
    }

    /// 1400x1400 solid colour PNG.
    #[must_use]
    pub fn png() -> PathBuf {
        PathBuf::from(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/samples/sample.png"
        ))
    }

    /// 1400x1400 solid colour JPEG.
    #[must_use]
    pub fn jpg() -> PathBuf {
        PathBuf::from(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/samples/sample.jpg"
        ))
    }
}

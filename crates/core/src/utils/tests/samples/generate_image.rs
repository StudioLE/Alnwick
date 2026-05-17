use crate::prelude::*;
use image::codecs::png::{CompressionType, FilterType, PngEncoder};
use image::{ImageBuffer, Rgb, RgbImage};
use std::fs::File;
use std::io::BufWriter;
use std::process::Command;

/// Image dimensions.
///
/// Matches the minimum Apple Podcasts artwork spec (1400x1400).
const SIZE: u32 = 1400;

/// Generate a solid colour PNG with best compression.
fn generate_png(path: &Path) {
    let img = solid_image(SIZE, SIZE);
    let file = File::create(path).expect("should create file");
    let writer = BufWriter::new(file);
    let encoder = PngEncoder::new_with_quality(writer, CompressionType::Best, FilterType::Adaptive);
    img.write_with_encoder(encoder).expect("should save PNG");
}

/// Generate a solid colour JPEG via ffmpeg.
///
/// The `image` crate's JPEG encoder produces ~53 KB for a solid colour 1400x1400
/// regardless of quality setting. ffmpeg produces ~12 KB for the same image.
fn generate_jpeg(path: &Path) {
    let status = Command::new("ffmpeg")
        .args(["-y", "-f", "lavfi", "-i"])
        .arg(format!("color=c=0x4682B4:s={SIZE}x{SIZE}:d=1"))
        .args(["-frames:v", "1", "-update", "1"])
        .arg(path)
        .status()
        .expect("ffmpeg should be available");
    assert!(status.success(), "ffmpeg should succeed");
}

fn solid_image(width: u32, height: u32) -> RgbImage {
    ImageBuffer::from_pixel(width, height, Rgb([70, 130, 180]))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    const SAMPLES_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/samples");

    #[test]
    #[ignore = "generates sample files"]
    fn generate_sample_png() {
        fs::create_dir_all(SAMPLES_DIR).expect("should create samples dir");
        let path = PathBuf::from(SAMPLES_DIR).join("sample.png");
        generate_png(&path);
        let size = fs::metadata(&path).expect("should read metadata").len();
        #[expect(
            clippy::cast_precision_loss,
            clippy::as_conversions,
            reason = "acceptable precision loss for human-readable display"
        )]
        let kb = size as f64 / 1024.0;
        println!("Generated PNG: {size} bytes ({kb:.1} KB)");
    }

    #[test]
    #[ignore = "generates sample files"]
    fn generate_sample_jpeg() {
        fs::create_dir_all(SAMPLES_DIR).expect("should create samples dir");
        let path = PathBuf::from(SAMPLES_DIR).join("sample.jpg");
        generate_jpeg(&path);
        let size = fs::metadata(&path).expect("should read metadata").len();
        #[expect(
            clippy::cast_precision_loss,
            clippy::as_conversions,
            reason = "acceptable precision loss for human-readable display"
        )]
        let kb = size as f64 / 1024.0;
        println!("Generated JPEG: {size} bytes ({kb:.1} KB)");
    }
}

use crate::prelude::*;
use std::process::Command;

/// Generate a 1-second 440Hz sine wave audio file via ffmpeg.
fn generate_audio(path: &Path, bitrate: &str) {
    let status = Command::new("ffmpeg")
        .args(["-y", "-f", "lavfi", "-i", "sine=frequency=440:duration=1"])
        .args(["-ac", "1", "-b:a", bitrate])
        .arg(path)
        .status()
        .expect("ffmpeg should be available");
    assert!(status.success(), "ffmpeg should succeed");
}

/// Generate a minimal MP3 via ffmpeg.
fn generate_mp3(path: &Path) {
    generate_audio(path, "64k");
}

/// Generate a minimal AAC file via ffmpeg.
fn generate_aac(path: &Path) {
    generate_audio(path, "64k");
}

/// Generate a minimal M4A (AAC in MP4 container) via ffmpeg.
fn generate_m4a(path: &Path) {
    generate_audio(path, "64k");
}

/// Generate a minimal OGG Vorbis file via ffmpeg.
fn generate_ogg(path: &Path) {
    generate_audio(path, "64k");
}

/// Generate a minimal Opus file via ffmpeg.
fn generate_opus(path: &Path) {
    generate_audio(path, "64k");
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    const SAMPLES_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/samples");

    fn print_size(label: &str, path: &Path) {
        let size = fs::metadata(path).expect("should read metadata").len();
        #[expect(
            clippy::cast_precision_loss,
            clippy::as_conversions,
            reason = "acceptable precision loss for human-readable display"
        )]
        let kb = size as f64 / 1024.0;
        println!("Generated {label}: {size} bytes ({kb:.1} KB)");
    }

    #[test]
    #[ignore = "generates sample files"]
    fn generate_sample_mp3() {
        fs::create_dir_all(SAMPLES_DIR).expect("should create samples dir");
        let path = PathBuf::from(SAMPLES_DIR).join("sample.mp3");
        generate_mp3(&path);
        print_size("MP3", &path);
    }

    #[test]
    #[ignore = "generates sample files"]
    fn generate_sample_aac() {
        fs::create_dir_all(SAMPLES_DIR).expect("should create samples dir");
        let path = PathBuf::from(SAMPLES_DIR).join("sample.aac");
        generate_aac(&path);
        print_size("AAC", &path);
    }

    #[test]
    #[ignore = "generates sample files"]
    fn generate_sample_m4a() {
        fs::create_dir_all(SAMPLES_DIR).expect("should create samples dir");
        let path = PathBuf::from(SAMPLES_DIR).join("sample.m4a");
        generate_m4a(&path);
        print_size("M4A", &path);
    }

    #[test]
    #[ignore = "generates sample files"]
    fn generate_sample_ogg() {
        fs::create_dir_all(SAMPLES_DIR).expect("should create samples dir");
        let path = PathBuf::from(SAMPLES_DIR).join("sample.ogg");
        generate_ogg(&path);
        print_size("OGG", &path);
    }

    #[test]
    #[ignore = "generates sample files"]
    fn generate_sample_opus() {
        fs::create_dir_all(SAMPLES_DIR).expect("should create samples dir");
        let path = PathBuf::from(SAMPLES_DIR).join("sample.opus");
        generate_opus(&path);
        print_size("Opus", &path);
    }
}

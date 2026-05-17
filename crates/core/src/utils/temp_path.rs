use crate::prelude::*;
use rand::random;
use std::fmt::Write;

/// Generate a temporary file path from a base path.
///
/// Appends `.{hash}.tmp` where `{hash}` is 8 random hex characters.
/// The resulting path is adjacent to the base file, preventing clashes
/// with concurrent operations.
#[must_use]
pub fn temp_path(base: &Path) -> PathBuf {
    let mut hash = String::with_capacity(8);
    let bytes: [u8; 4] = random();
    for byte in bytes {
        let _ = write!(hash, "{byte:02x}");
    }
    let mut name = base
        .file_name()
        .expect("base path should have a file name")
        .to_owned();
    name.push(format!(".{hash}.tmp"));
    base.with_file_name(name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn temp_path_unique() {
        // Arrange
        let base = PathBuf::from("/some/dir/cover.jpg");

        // Act
        let a = temp_path(&base);
        let b = temp_path(&base);

        // Assert
        assert_ne!(a, base, "should differ from input");
        assert!(
            a.to_str()
                .expect("should be valid utf8")
                .starts_with("/some/dir/cover.jpg."),
            "should start with the base path"
        );
        assert_ne!(a, b, "two calls should produce different paths");
    }
}

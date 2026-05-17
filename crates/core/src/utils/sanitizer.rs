const NON_BREAKING_SPACE: char = '\u{00A0}';
const ZERO_WIDTH_SPACE: char = '\u{200B}';
#[allow(dead_code)]
const ZERO_WIDTH_NON_JOINER: char = '\u{200C}';
#[allow(dead_code)]
const ZERO_WIDTH_JOINER: char = '\u{200D}';
const LEFT_TO_RIGHT_MARK: char = '\u{200E}';
const RIGHT_TO_LEFT_MARK: char = '\u{200F}';
const LEFT_TO_RIGHT_EMBEDDING: char = '\u{202A}';
const RIGHT_TO_LEFT_EMBEDDING: char = '\u{202B}';
const POP_DIRECTIONAL_FORMATTING: char = '\u{202C}';
const LEFT_TO_RIGHT_OVERRIDE: char = '\u{202D}';
const RIGHT_TO_LEFT_OVERRIDE: char = '\u{202E}';
const ZERO_WIDTH_NO_BREAK_SPACE: char = '\u{FEFF}';
const EN_DASH: char = '\u{2013}';
const EM_DASH: char = '\u{2014}';
const RESTRICTED: [char; 18] = [
    NON_BREAKING_SPACE,
    ZERO_WIDTH_SPACE,
    LEFT_TO_RIGHT_MARK,
    RIGHT_TO_LEFT_MARK,
    LEFT_TO_RIGHT_EMBEDDING,
    RIGHT_TO_LEFT_EMBEDDING,
    POP_DIRECTIONAL_FORMATTING,
    LEFT_TO_RIGHT_OVERRIDE,
    RIGHT_TO_LEFT_OVERRIDE,
    ZERO_WIDTH_NO_BREAK_SPACE,
    ':',
    '<',
    '>',
    '"',
    '?',
    '*',
    '#',
    '%',
];
const RESTRICTED_DIVIDERS: [char; 5] = ['/', '\\', '|', EN_DASH, EM_DASH];
const DIVIDER_REPLACEMENT: char = '-';

pub struct Sanitizer;

impl Sanitizer {
    /// Sanitize a string for use as a file name.
    ///
    /// - Removes restricted characters (Unicode control characters, Windows-forbidden characters, URL-problematic characters)
    /// - Replaces path dividers with dashes
    #[must_use]
    pub fn execute(input: &str) -> String {
        input
            .chars()
            .filter(|x| !RESTRICTED.contains(x) && !x.is_control())
            .map(|x| {
                if RESTRICTED_DIVIDERS.contains(&x) {
                    DIVIDER_REPLACEMENT
                } else {
                    x
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn removes_hash_character() {
        assert_eq!(Sanitizer::execute("Episode #1"), "Episode 1");
    }

    #[test]
    fn removes_percent_character() {
        assert_eq!(Sanitizer::execute("100% Complete"), "100 Complete");
    }

    #[test]
    fn removes_multiple_restricted_characters() {
        assert_eq!(
            Sanitizer::execute("Episode #1: 100% Done?"),
            "Episode 1 100 Done"
        );
    }

    #[test]
    fn replaces_path_dividers_with_dash() {
        assert_eq!(Sanitizer::execute("Part 1/2"), "Part 1-2");
        assert_eq!(Sanitizer::execute("A|B"), "A-B");
    }
}

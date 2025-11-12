pub struct Validator;

impl Validator {
    pub fn validate_id(id: &str) -> Result<String, String> {
        if id.is_empty() {
            return Err("Value must not be empty".to_owned());
        }
        if id
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
        {
            Ok(id.to_owned())
        } else {
            Err("Podcast ID must contain only lowercase letters and hyphens".to_owned())
        }
    }
}

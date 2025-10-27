use crate::prelude::*;
use std::num::ParseIntError;

#[derive(Debug)]
pub enum PodcastConvertError {
    Required(String),
    Url(String, url::ParseError),
    Date(String, chrono::ParseError),
    Integer(String, ParseIntError),
}

impl Display for PodcastConvertError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{self:?}")
    }
}

impl Error for PodcastConvertError {}

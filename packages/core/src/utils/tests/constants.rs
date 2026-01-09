use crate::prelude::*;

#[must_use]
pub fn example_rss_url() -> UrlWrapper {
    UrlWrapper::from_str("https://feeds.simplecast.com/lP7owBq8").expect("URL should parse")
}

#[must_use]
pub fn example_simplecast_url() -> UrlWrapper {
    UrlWrapper::from_str("https://irlpodcast.org").expect("URL should parse")
}

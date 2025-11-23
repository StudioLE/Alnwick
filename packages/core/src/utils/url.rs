use tracing::warn;
use url::Url;

#[must_use]
pub fn get_image_url(option: Option<String>) -> Option<Url> {
    option.and_then(|url| {
        Url::parse(&url)
            .map_err(|error| {
                warn!(%url, %error, "Failed to parse image URL");
            })
            .ok()
    })
}

pub trait UrlExtensions {
    fn get_extension(&self) -> Option<String>;
}

impl UrlExtensions for Url {
    fn get_extension(&self) -> Option<String> {
        let path = self.path();
        path.rsplit('.').next().map(ToOwned::to_owned)
    }
}

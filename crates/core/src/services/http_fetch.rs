//! Trait for fetching HTTP resources.
use crate::prelude::*;

/// Trait for fetching HTTP resources.
///
/// Object-safe async trait for HTTP operations. Implementations handle
/// rate limiting, retries, and other transport concerns.
#[async_trait]
pub trait HttpFetch: Send + Sync {
    /// Fetch HTML content from a URL and parse it.
    async fn get_html(&self, url: &UrlWrapper) -> Result<Html, Report<HttpError>>;

    /// Fetch the response body as a string.
    async fn get_string(&self, url: &UrlWrapper) -> Result<String, Report<HttpError>>;

    /// Perform a HEAD request and return the Content-Type header value.
    async fn head(&self, url: &UrlWrapper) -> Result<String, Report<HttpError>>;

    /// Download a file from a URL to a destination path.
    async fn download(
        &self,
        url: &UrlWrapper,
        destination: PathBuf,
    ) -> Result<(), Report<HttpError>>;
}

/// Extension trait for [`HttpFetch`] providing generic convenience methods.
///
/// These methods cannot be on the base trait because generic methods
/// break object safety.
#[async_trait]
pub trait HttpFetchExt {
    /// Fetch JSON content from a URL and deserialize it.
    async fn get_json<T: DeserializeOwned>(&self, url: &UrlWrapper)
    -> Result<T, Report<HttpError>>;
}

#[async_trait]
impl<H: HttpFetch + ?Sized> HttpFetchExt for H {
    async fn get_json<T: DeserializeOwned>(
        &self,
        url: &UrlWrapper,
    ) -> Result<T, Report<HttpError>> {
        let text = self.get_string(url).await?;
        serde_json::from_str(&text)
            .change_context(HttpError::Deserialize)
            .attach_url(url)
    }
}

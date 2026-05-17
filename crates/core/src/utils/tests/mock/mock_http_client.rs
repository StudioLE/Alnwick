use crate::prelude::*;
use tokio::fs::copy;

/// Mock implementation of [`HttpFetch`] for testing.
///
/// Stores URL-to-response mappings for strings and file paths. String
/// mappings are returned by `get_html` and `get_string`. File mappings
/// are copied to the destination by `download`.
pub struct MockHttpClient {
    strings: HashMap<String, String>,
    files: HashMap<String, PathBuf>,
    content_types: HashMap<String, String>,
}

impl MockHttpClient {
    /// Create an empty [`MockHttpClient`].
    #[must_use]
    pub fn new() -> Self {
        Self {
            strings: HashMap::new(),
            files: HashMap::new(),
            content_types: HashMap::new(),
        }
    }

    /// Map a URL to a string response.
    #[must_use]
    pub fn with_string(mut self, url: &str, body: String) -> Self {
        self.strings.insert(String::from(url), body);
        self
    }

    /// Map a URL to a file that will be copied on download.
    #[must_use]
    pub fn with_file(mut self, url: &str, path: PathBuf) -> Self {
        self.files.insert(String::from(url), path);
        self
    }

    /// Set the content type returned by `head` for a URL.
    #[must_use]
    pub fn with_content_type(mut self, url: &str, content_type: &str) -> Self {
        self.content_types
            .insert(String::from(url), String::from(content_type));
        self
    }

    fn get_string_value(&self, url: &UrlWrapper) -> Result<&String, Report<HttpError>> {
        self.strings
            .get(url.as_str())
            .ok_or_else(|| Report::new(HttpError::Request).attach_url(url))
    }

    fn get_file_value(&self, url: &UrlWrapper) -> Result<&PathBuf, Report<HttpError>> {
        self.files
            .get(url.as_str())
            .ok_or_else(|| Report::new(HttpError::Request).attach_url(url))
    }
}

#[async_trait]
impl HttpFetch for MockHttpClient {
    async fn get_html(&self, url: &UrlWrapper) -> Result<Html, Report<HttpError>> {
        let body = self.get_string_value(url)?;
        Ok(Html::parse_document(body))
    }

    async fn get_string(&self, url: &UrlWrapper) -> Result<String, Report<HttpError>> {
        let body = self.get_string_value(url)?;
        Ok(body.clone())
    }

    async fn head(&self, url: &UrlWrapper) -> Result<String, Report<HttpError>> {
        if let Some(content_type) = self.content_types.get(url.as_str()) {
            return Ok(content_type.clone());
        }
        if self.strings.contains_key(url.as_str()) {
            return Ok(String::from("text/html"));
        }
        if self.files.contains_key(url.as_str()) {
            return Ok(String::from("application/octet-stream"));
        }
        Err(Report::new(HttpError::Request).attach_url(url))
    }

    async fn download(
        &self,
        url: &UrlWrapper,
        destination: PathBuf,
    ) -> Result<(), Report<HttpError>> {
        let source = self.get_file_value(url)?;
        create_parent_dir_if_not_exist(&destination)
            .await
            .change_context(HttpError::CreateDestinationDirectory)?;
        copy(source, &destination)
            .await
            .change_context(HttpError::WriteDestination)
            .attach_url(url)
            .attach_path(&destination)?;
        Ok(())
    }
}

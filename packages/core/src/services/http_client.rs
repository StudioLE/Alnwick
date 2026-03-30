use crate::prelude::*;
use crate::services::HttpRateLimiter;
use crate::services::ipinfo::IpInfoProvider;
use reqwest::Client as ReqwestClient;
use reqwest::Response;
use reqwest::header::CONTENT_TYPE;

const DEFAULT_DOMAIN: &str = "__unknown";

/// A client for making HTTP requests with rate limiting.
///
/// - Uses `HttpRateLimiter` for per-domain rate limiting
/// - Uses `reqwest::Client` for actual HTTP requests
#[derive(Clone)]
pub struct HttpClient {
    rate_limiter: Arc<HttpRateLimiter>,
    client: ReqwestClient,
}

impl HttpClient {
    /// Send a rate-limited GET request and check the response status.
    async fn send_get(&self, url: &UrlWrapper) -> Result<Response, Report<HttpError>> {
        let domain = url.domain().unwrap_or(DEFAULT_DOMAIN);
        self.rate_limiter.wait_for_permit(domain).await;
        let response = self
            .client
            .get(url.as_str())
            .send()
            .await
            .change_context(HttpError::Request)
            .attach_url(url)?;
        if !response.status().is_success() {
            let report = Report::new(HttpError::Status(response.status().as_u16())).attach_url(url);
            return Err(report);
        }
        Ok(response)
    }
}

impl FromServicesAsync for HttpClient {
    type Error = ResolveError;

    async fn from_services_async(services: &ServiceProvider) -> Result<Self, Report<ResolveError>> {
        let ipinfo = services.get::<IpInfoProvider>()?;
        ipinfo
            .validate()
            .await
            .change_context(ResolveError::Factory)?;
        Ok(Self {
            rate_limiter: services.get()?,
            client: ReqwestClient::new(),
        })
    }
}

#[async_trait]
impl HttpFetch for HttpClient {
    async fn get_html(&self, url: &UrlWrapper) -> Result<Html, Report<HttpError>> {
        let body = self.get_string(url).await?;
        Ok(Html::parse_document(&body))
    }

    async fn get_string(&self, url: &UrlWrapper) -> Result<String, Report<HttpError>> {
        let response = self.send_get(url).await?;
        response
            .text()
            .await
            .change_context(HttpError::Request)
            .attach_url(url)
    }

    async fn head(&self, url: &UrlWrapper) -> Result<String, Report<HttpError>> {
        let domain = url.domain().unwrap_or(DEFAULT_DOMAIN);
        self.rate_limiter.wait_for_permit(domain).await;
        let response = self
            .client
            .head(url.as_str())
            .send()
            .await
            .change_context(HttpError::Request)
            .attach_url(url)?;
        let content_type = get_content_type(response).unwrap_or_default();
        Ok(content_type)
    }

    async fn download(
        &self,
        url: &UrlWrapper,
        destination: PathBuf,
    ) -> Result<(), Report<HttpError>> {
        let mut response = self.send_get(url).await?;
        write_response_to_file(&mut response, &destination).await?;
        Ok(())
    }
}

/// Stream a response body to a file on disk.
///
/// - Creates parent directories if needed
/// - Removes any existing file at the destination
/// - Writes chunks incrementally and syncs
/// - Errors if zero bytes are written
async fn write_response_to_file(
    response: &mut Response,
    destination: &Path,
) -> Result<(), Report<HttpError>> {
    create_parent_dir_if_not_exist(destination)
        .await
        .change_context(HttpError::CreateDestinationDirectory)?;
    if destination.exists() {
        remove_file(destination)
            .await
            .change_context(HttpError::RemoveExisting)?;
    }
    let mut file = AsyncFile::create(destination)
        .await
        .change_context(HttpError::WriteDestination)
        .attach_path(destination)?;
    let mut bytes_written: usize = 0;
    while let Some(chunk) = response
        .chunk()
        .await
        .change_context(HttpError::Chunk)
        .attach_path(destination)?
    {
        bytes_written += chunk.len();
        file.write_all(&chunk)
            .await
            .change_context(HttpError::WriteDestination)
            .attach_path(destination)?;
    }
    file.sync_all()
        .await
        .change_context(HttpError::WriteDestination)
        .attach_path(destination)?;
    if bytes_written == 0 {
        let report = Report::new(HttpError::Size).attach_path(destination);
        return Err(report);
    }
    Ok(())
}

fn get_content_type(response: Response) -> Option<String> {
    let content_type = response
        .headers()
        .get(CONTENT_TYPE)?
        .to_str()
        .ok()?
        .split(';')
        .next()
        .unwrap_or_default()
        .trim()
        .to_lowercase();
    Some(content_type)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::Value;
    use std::time::{Duration, Instant};

    #[tokio::test]
    #[ignore = "uses example.com"]
    pub async fn head() {
        // Arrange
        let services = ServiceBuilder::new().with_core().build();
        let http = services
            .get_async::<HttpClient>()
            .await
            .expect("should be able to get HttpClient");
        let url =
            UrlWrapper::from_str("https://example.com/?abc=123&def=456").expect("valid test URL");
        let _logger = init_test_logger();

        // Act
        let result = http.head(&url).await;

        // Assert
        let content_type = result.assert_ok_debug();
        assert_eq!(content_type, "text/html");
    }

    #[tokio::test]
    #[ignore = "uses simplecast.com"]
    pub async fn head_xml() {
        // Arrange
        let services = ServiceBuilder::new().with_core().build();
        let http = services
            .get_async::<HttpClient>()
            .await
            .expect("should be able to get HttpClient");
        let url = UrlWrapper::from_str("https://feeds.simplecast.com/lP7owBq8")
            .expect("URL should parse");
        let _logger = init_test_logger();

        // Act
        let result = http.head(&url).await;

        // Assert
        let content_type = result.assert_ok_debug();
        assert_eq!(content_type, "application/xml");
    }

    #[tokio::test]
    #[ignore = "uses example.com"]
    pub async fn get_string() {
        // Arrange
        let services = ServiceBuilder::new().with_core().build();
        let http = services
            .get_async::<HttpClient>()
            .await
            .expect("should be able to get HttpClient");
        let url =
            UrlWrapper::from_str("https://example.com/?abc=123&def=456").expect("valid test URL");
        let _logger = init_test_logger();

        // Act
        let result = http.get_string(&url).await;

        // Assert
        let body = result.assert_ok_debug();
        assert!(!body.is_empty());
    }

    #[tokio::test]
    #[ignore = "uses example.com"]
    pub async fn get_html() {
        // Arrange
        let services = ServiceBuilder::new().with_core().build();
        let http = services
            .get_async::<HttpClient>()
            .await
            .expect("should be able to get HttpClient");
        let url = UrlWrapper::from_str("https://example.com").expect("valid test URL");
        let _logger = init_test_logger();

        // Act
        let result = http.get_html(&url).await;

        // Assert
        let _html = result.assert_ok_debug();
    }

    #[tokio::test]
    #[ignore = "uses ipinfo.io"]
    pub async fn get_json() {
        // Arrange
        let services = ServiceBuilder::new().with_core().build();
        let http = services
            .get_async::<HttpClient>()
            .await
            .expect("should be able to get HttpClient");
        let url = UrlWrapper::from_str("https://ipinfo.io/json").expect("valid test URL");
        let _logger = init_test_logger();

        // Act
        let result = http.get_json::<Value>(&url).await;

        // Assert
        let _json = result.assert_ok_debug();
    }

    #[tokio::test]
    #[ignore = "requires network"]
    async fn domains_isolated() {
        // Arrange
        let services = ServiceBuilder::new().with_core().build();
        let http = services
            .get_async::<HttpClient>()
            .await
            .expect("should be able to get HttpClient");
        let _logger = init_test_logger();
        let http1 = http.clone();
        let http2 = http.clone();

        // Act
        let start = Instant::now();
        let task1 = tokio::spawn(async move {
            for i in 0..6 {
                let url = UrlWrapper::from_str(&format!("https://example.com/isolated-{i}"))
                    .expect("valid test URL");
                let _ = http1.get_string(&url).await;
            }
        });
        let task2 = tokio::spawn(async move {
            for i in 0..6 {
                let url = UrlWrapper::from_str(&format!("https://httpbin.org/isolated-{i}"))
                    .expect("valid test URL");
                let _ = http2.get_string(&url).await;
            }
        });
        let _ = tokio::try_join!(task1, task2);
        let elapsed = start.elapsed();

        // Assert
        assert!(
            elapsed < Duration::from_secs(3),
            "Expected parallel execution with independent rate limits, elapsed: {elapsed:?}"
        );
    }
}

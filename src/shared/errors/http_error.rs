use crate::prelude::*;

#[allow(clippy::absolute_paths)]
#[derive(Debug)]
pub enum HttpError {
    Response(Url, u16),
    Request(Url, reqwest::Error),
    Io(PathBuf, std::io::Error),
    ResponseIo(Url, reqwest::Error),
    InvalidJson(PathBuf, serde_json::Error),
    NoContentType(Url),
}

impl Display for HttpError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let message = match self {
            HttpError::Response(url, number) => {
                let reason = StatusCode::from_u16(*number)
                    .map(|e| e.canonical_reason())
                    .ok()
                    .flatten()
                    .unwrap_or_default();
                format!("Unexpected response status: {number} {reason}\nURL: {url}")
            }
            HttpError::Request(url, e) => format!("A request error occurred.\nURL:{url}\n{e}"),
            HttpError::Io(path, e) => {
                format!("An I/O error occurred.\nPath: {}\n{e}", path.display())
            }
            HttpError::InvalidJson(path, e) => {
                format!(
                    "A deserialization error occurred.\nPath: {}\n{e}",
                    path.display()
                )
            }
            HttpError::ResponseIo(url, e) => {
                format!("A response I/O error occurred.\nURL: {url}\n{e}",)
            }
            HttpError::NoContentType(url) => {
                format!("Response did not contain a Content-Type header:\nURL: {url}",)
            }
        };
        write!(f, "{message}")
    }
}

use crate::prelude::*;

pub struct ServiceProvider {
    pub options: AppOptions,
    pub paths: PathProvider,
    pub http: HttpClient,
    pub metadata: MetadataStore,
}

impl ServiceProvider {
    pub async fn create() -> Result<ServiceProvider, Report<ServiceError>> {
        let options = AppOptions::get()?;
        let paths = PathProvider::new(options.clone());
        paths.create()?;
        let http = HttpClient::new(paths.get_http_dir());
        let ip = IpInfoProvider::new(options.clone(), http.clone());
        ip.validate().await?;
        let metadata = MetadataStore::new(paths.get_metadata_dir());
        Ok(Self {
            options,
            paths,
            http,
            metadata,
        })
    }
}

#[derive(Clone, Debug, Error)]
pub enum ServiceError {
    #[error("Unable to read config from environment variables")]
    EnvConfig,
    #[error("Unable to create {0} directory")]
    CreateDirectory(String),
    #[error("Failed to make request for external IP")]
    IpRequest,
    #[error("IP validation failed")]
    ValidateIp,
}

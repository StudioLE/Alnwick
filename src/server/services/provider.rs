use crate::prelude::*;

pub struct ServiceProvider {
    pub options: AppOptions,
    pub paths: PathProvider,
    pub http: HttpClient,
    pub metadata: MetadataStore,
}

impl ServiceProvider {
    pub async fn create() -> Result<ServiceProvider, ServiceError> {
        let options = AppOptions::get().map_err(ServiceError::GetConfig)?;
        let paths = PathProvider::new(options.clone());
        paths.validate().map_err(ServiceError::ValidatePaths)?;
        paths.create().map_err(ServiceError::CreateDirectories)?;
        let http = HttpClient::new(paths.get_http_dir());
        let ip = IpInfoProvider::new(options.clone(), http.clone());
        ip.validate().await.map_err(ServiceError::ValidateIp)?;
        let metadata = MetadataStore::new(paths.get_metadata_dir());
        Ok(Self {
            options,
            paths,
            http,
            metadata,
        })
    }
}

#[derive(Debug)]
pub enum ServiceError {
    GetConfig(envy::Error),
    ValidatePaths(Vec<ValidationError>),
    CreateDirectories(CreateDirectoryError),
    ValidateIp(Vec<ValidationError>),
}

impl Display for ServiceError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let action = match self {
            ServiceError::GetConfig(_) => "read config",
            ServiceError::ValidatePaths(_) => "validate paths",
            ServiceError::CreateDirectories(_) => "create directories",
            ServiceError::ValidateIp(_) => "validate IP",
        };
        let reason = match self {
            ServiceError::GetConfig(e) => e.to_string(),
            ServiceError::ValidatePaths(errors) | ServiceError::ValidateIp(errors) => errors.log(),
            ServiceError::CreateDirectories(e) => e.to_string(),
        };
        write!(f, "{} to {action}\n{reason}", "Failed".bold())
    }
}

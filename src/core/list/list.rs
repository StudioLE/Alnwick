use std::str::FromStr;
use crate::prelude::*;

const CONCURRENCY: usize = 8;
const IMAGE_SIZE: u32 = 720;

pub struct ListCommand {
    paths: PathProvider,
    http: HttpClient,
    metadata: MetadataStore,
}

impl ListCommand {
    #[must_use]
    pub fn new(paths: PathProvider, http: HttpClient, metadata: MetadataStore) -> Self {
        Self {
            paths,
            http,
            metadata,
        }
    }

    pub async fn execute(&self) -> Result<Vec<Podcast>, ListError> {
        let ids = vec!["office-ladies", "irl", "frank-skinner"];
        ids
            .into_iter()
            .map(|id|
                self
                    .metadata
                    .get(id)
                    .map_err(|e| ListError::GetPodcast(e)))
            .collect::<Result<_, _>>()
    }
}

#[allow(clippy::absolute_paths)]
#[derive(Debug)]
pub enum ListError {
    GetPodcast(DatabaseError),
}

impl FromStr for ListError {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

impl Error for ListError {}

impl Display for ListError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let reason = match self {
            ListError::GetPodcast(e) => format!("Unable to get podcast\n{e}"),
        };
        write!(f, "{} to download\n{reason}", "Failed".bold())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    pub async fn list_command() {
        // Arrange
        let _ = init_logging();
        let services = ServiceProvider::create()
            .await
            .expect("ServiceProvider should not fail");
        let command = ListCommand::new(services.paths, services.http, services.metadata);

        // Act
        let result = command.execute().await;

        // Assert
        result.assert_ok();
    }
}

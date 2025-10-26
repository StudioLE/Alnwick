use crate::prelude::*;

#[allow(clippy::absolute_paths)]
#[derive(Debug)]
pub enum DatabaseError {
    NotFound(String),
    Io(PathBuf, std::io::Error),
    Serialization(PathBuf, serde_yaml::Error),
    Deserialization(PathBuf, serde_yaml::Error),
}

impl Display for DatabaseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let message = match self {
            DatabaseError::NotFound(id) => format!("Podcast not found: {id}"),
            DatabaseError::Io(path, e) => {
                format!("An I/O error occurred.\nPath: {}\n{e}", path.display())
            }
            DatabaseError::Serialization(path, e) => {
                format!(
                    "A serialization error occurred.\nPath: {}\n{e}",
                    path.display()
                )
            }
            DatabaseError::Deserialization(path, e) => {
                format!(
                    "A deserialization error occurred.\nPath: {}\n{e}",
                    path.display()
                )
            }
        };
        write!(f, "{message}")
    }
}
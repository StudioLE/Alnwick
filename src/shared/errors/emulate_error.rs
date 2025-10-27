use crate::prelude::*;

#[allow(clippy::absolute_paths)]
#[derive(Debug)]
pub enum EmulateError {
    GetPodcast(DatabaseError),
    Xml(PathBuf, std::io::Error),
}

impl Display for EmulateError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let reason = match self {
            EmulateError::GetPodcast(e) => format!("Unable to get podcast\n{e}"),
            EmulateError::Xml(path, e) => {
                format!("Unable to write RSS\nPath: {}\n{e}", path.display())
            }
        };
        write!(f, "{} to create RSS feeds\n{reason}", "Failed".bold())
    }
}

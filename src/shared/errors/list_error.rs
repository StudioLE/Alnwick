use crate::prelude::*;

#[allow(clippy::absolute_paths)]
#[derive(Debug)]
pub enum ListError {
    GetPodcast(DatabaseError),
}

impl Display for ListError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let reason = match self {
            ListError::GetPodcast(e) => format!("Unable to get podcast\n{e}"),
        };
        write!(f, "{} to download\n{reason}", "Failed".bold())
    }
}

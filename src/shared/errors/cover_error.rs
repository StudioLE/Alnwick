use crate::prelude::*;

#[derive(Debug)]
pub enum CoverError {
    GetPodcast(DatabaseError),
    NoImage,
    GetImage(HttpError),
    Image(ImageError),
}

impl Display for CoverError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let reason = match self {
            CoverError::GetPodcast(e) => format!("Unable to get podcast\n{e}"),
            CoverError::NoImage => "Podcast does not have an image".to_owned(),
            CoverError::GetImage(e) => format!("Unable to get image:\n{e}"),
            CoverError::Image(e) => format!("Unable to create image:\n{e}"),
        };
        write!(
            f,
            "{} to create cover and banner images\n{reason}",
            "Failed".bold()
        )
    }
}

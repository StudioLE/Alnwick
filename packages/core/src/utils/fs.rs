use crate::prelude::*;
use std::io::Error;

pub async fn create_parent_dir_if_not_exist(path: &Path) -> Result<(), Report<Error>> {
    let Some(dir) = path.parent() else {
        trace!(path = %path.display(), "No parent directory to create");
        return Ok(());
    };
    if !dir.exists() {
        trace!(dir = %dir.display(), "Creating directory");
        create_dir_all(&dir).await.attach_path(dir)?;
    }
    Ok(())
}

pub async fn copy_with_logging(
    source: PathBuf,
    destination: PathBuf,
) -> Result<(), Report<HttpError>> {
    create_parent_dir_if_not_exist(&destination)
        .await
        .change_context(HttpError::CreateDestinationDirectory)?;
    trace!(
        source = %source.display(),
        destination = %destination.display(),
        "Copying file"
    );
    if hard_link(&source, &destination).await.is_err() {
        copy(&source, &destination)
            .await
            .change_context(HttpError::Copy)
            .attach_with(|| {
                format!(
                    "Source: {}\nDestination: {}",
                    source.display(),
                    destination.display()
                )
            })?;
    }
    Ok(())
}

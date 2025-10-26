use crate::prelude::*;

pub struct MetadataStore {
    dir: PathBuf,
}

impl MetadataStore {
    pub(crate) fn new(dir: PathBuf) -> Self {
        Self { dir }
    }

    pub(crate) fn get(&self, id: &str) -> Result<Podcast, DatabaseError> {
        let path = self.get_path(id);
        if !path.exists() {
            return Err(DatabaseError::NotFound(id.to_owned()));
        }
        let file = File::open(&path).map_err(|e| DatabaseError::Io(path.clone(), e))?;
        let reader = BufReader::new(file);
        serde_yaml::from_reader(reader).map_err(|e| DatabaseError::Deserialization(path.clone(), e))
    }

    pub(crate) fn put(&self, podcast: &Podcast) -> Result<(), DatabaseError> {
        let path = self.get_path(&podcast.id);
        let file = File::create(&path).map_err(|e| DatabaseError::Io(path.clone(), e))?;
        let writer = BufWriter::new(file);
        serde_yaml::to_writer(writer, podcast)
            .map_err(|e| DatabaseError::Serialization(path.clone(), e))
    }

    fn get_path(&self, id: &str) -> PathBuf {
        self.dir.join(id).with_extension("yml")
    }
}

impl Default for MetadataStore {
    fn default() -> Self {
        Self {
            dir: PathProvider::default().get_metadata_dir(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::*;
    use super::*;

    #[test]
    pub fn put_then_get() {
        // Arrange
        let _ = init_logging();
        let metadata = MetadataStore::default();
        let podcast = Podcast::example();

        // Act
        metadata.put(&podcast).assert_ok();
        let result = metadata.get(&podcast.id);

        // Assert
        let result = result.assert_ok();
        assert_eq!(podcast, result);
    }
}

use crate::prelude::*;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Entry {
    pub date: NaiveDate,
    pub weight: Option<f32>,
    pub shot: Option<Shot>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_json() -> Result<(), serde_json::Error> {
        // Arrange
        let csv = include_str!("../../samples/shotsy.csv");
        let data = ShotsyData::from_csv(csv);
        let entries = ShotsyData::to_entries(data);
        // Act
        let json = serde_json::to_string_pretty(&entries)?;
        // Assert
        let expect = Expect::new()
            .string(&json, "json")
            .expect("Expect should not fail");
        assert!(expect);
        Ok(())
    }

    #[test]
    fn from_json() -> Result<(), serde_json::Error> {
        // Arrange
        let json = include_str!("../../samples/entries.json");
        // Act
        let entries: Vec<Entry> = serde_json::from_str(json)?;
        // Assert
        assert_eq!(entries.len(), 144);
        Ok(())
    }
}

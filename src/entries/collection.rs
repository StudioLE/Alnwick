use crate::prelude::*;

#[derive(Clone, Debug, Default)]
pub struct EntryCollection {
    pub entries: BTreeMap<NaiveDate, Entry>,
    pub statistics: BTreeMap<NaiveDate, WeightStatistics>,
    pub range: EntryRange,
}

impl EntryCollection {
    pub fn new(entries: Vec<Entry>) -> Result<Self, EntryRangeError> {
        let range = EntryRange::new(&entries)?;
        let entries = entries.into_iter().map(|e| (e.date, e)).collect();
        let statistics = BTreeMap::new();
        let mut collection = Self {
            entries,
            statistics,
            range,
        };
        for entry in collection.entries.values() {
            let statistics = collection.calculate_weight_statistics(entry.date);
            collection.statistics.insert(entry.date, statistics);
        }
        Ok(collection)
    }

    #[allow(clippy::as_conversions, clippy::cast_possible_wrap)]
    pub fn get_entries_before(&self, date: NaiveDate, days: usize) -> Vec<Option<Entry>> {
        let range = 1..=days as i64;
        range
            .into_iter()
            .map(|i| {
                let d = date - Duration::days(i);
                self.entries.get(&d).cloned()
            })
            .collect()
    }

    #[allow(clippy::as_conversions, clippy::cast_possible_wrap)]
    pub fn get_entries_after(&self, date: NaiveDate, days: usize) -> Vec<Option<Entry>> {
        let range = 1..=days as i64;
        range
            .into_iter()
            .map(|i| {
                let d = date + Duration::days(i);
                self.entries.get(&d).cloned()
            })
            .collect()
    }

    #[cfg(test)]
    pub fn get_sample() -> EntryCollection {
        let json = include_str!("../../samples/entries.json");
        let entries = serde_json::from_str(json).expect("Entries sample should be valid");
        EntryCollection::new(entries).expect("Range should be valid")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        // Arrange
        let collection = EntryCollection::get_sample();
        // Assert
        let statistics: Vec<_> = collection.statistics.into_values().collect();
        let expect = Expect::new()
            .values(&statistics)
            .expect("Expect should not fail");
        assert!(expect);
    }

    #[test]
    fn get_entries_before() {
        // Arrange
        let collection = EntryCollection::get_sample();
        // Act
        // Assert
        assert_eq!(count_entries_before(&collection, 2025, 2, 1), 0);
        assert_eq!(count_entries_before(&collection, 2025, 2, 6), 2);
        assert_eq!(count_entries_before(&collection, 2025, 2, 12), 3);
    }

    fn count_entries_before(
        collection: &EntryCollection,
        year: i32,
        month: u32,
        day: u32,
    ) -> usize {
        let date = NaiveDate::from_ymd_opt(year, month, day).expect("Date is valid");
        let results = collection.get_entries_before(date, 6);
        let results: Vec<_> = results.iter().flatten().collect();
        results.len()
    }
}

use crate::prelude::*;
use EntryRangeError::*;

#[derive(Clone, Debug, Default)]
pub struct EntryRange {
    pub min_date: NaiveDate,
    pub max_date: NaiveDate,
    pub min_weight: f32,
    pub max_weight: f32,
}

impl EntryRange {
    pub fn new(entries: &[Entry]) -> Result<EntryRange, EntryRangeError> {
        let mut range = Self::default();
        range.set_date_range(entries)?;
        range.set_weight_range(entries)?;
        Ok(range)
    }

    fn set_date_range(&mut self, entries: &[Entry]) -> Result<(), EntryRangeError> {
        let mut dates: Vec<_> = entries.iter().map(|x| x.date).collect();
        if dates.len() < 2 {
            return Err(NeedTwoDates);
        }
        dates.sort();
        self.min_date = *dates.first().expect("should be at least 2 entries");
        self.max_date = *dates.last().expect("should be at least 2 entries");
        Ok(())
    }

    fn set_weight_range(&mut self, entries: &[Entry]) -> Result<(), EntryRangeError> {
        let mut weights: Vec<_> = entries.iter().filter_map(|x| x.weight).collect();
        if weights.len() < 2 {
            return Err(NeedTwoWeights);
        }
        weights.sort_by(|a, b| a.partial_cmp(b).expect("weights should be comparable"));
        self.min_weight = *weights.first().expect("should be at least 2 weights");
        self.max_weight = *weights.last().expect("should be at least 2 weights");
        Ok(())
    }

    pub fn get_day(&self, date: NaiveDate) -> usize {
        usize::try_from((date - self.min_date).num_days()).expect("should not overflow")
    }

    #[allow(dead_code)]
    pub fn get_total_days(&self) -> isize {
        isize::try_from((self.max_date - self.min_date).num_days()).expect("should not overflow")
    }
}

#[derive(Debug)]
pub enum EntryRangeError {
    NeedTwoDates,
    NeedTwoWeights,
}

impl Display for EntryRangeError {
    #[allow(clippy::absolute_paths)]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl Error for EntryRangeError {}

use crate::prelude::*;

impl EntryCollection {
    #[allow(
        clippy::as_conversions,
        clippy::cast_possible_wrap,
        clippy::cast_precision_loss
    )]
    pub fn calculate_simple_moving_average(&self, date: NaiveDate, before: usize) -> Option<f32> {
        if date - Duration::days(before as i64) < self.range.min_date {
            return None;
        }
        let mut entries = vec![self.entries.get(&date).cloned()];
        entries.append(&mut self.get_entries_before(date, before));
        calculate_simple_moving_average(entries)
    }

    #[allow(
        clippy::as_conversions,
        clippy::cast_possible_wrap,
        clippy::cast_precision_loss
    )]
    pub fn calculate_simple_moving_average_centered(
        &self,
        date: NaiveDate,
        before: usize,
        after: usize,
    ) -> Option<f32> {
        if date - Duration::days(before as i64) < self.range.min_date {
            return None;
        }
        if date + Duration::days(after as i64) > self.range.max_date {
            return None;
        }
        let mut entries = self.get_entries_before(date, before);
        entries.push(self.entries.get(&date).cloned());
        entries.append(&mut self.get_entries_after(date, after));
        calculate_simple_moving_average(entries)
    }
}

#[allow(
    clippy::as_conversions,
    clippy::cast_possible_wrap,
    clippy::cast_precision_loss
)]
fn calculate_simple_moving_average(entries: Vec<Option<Entry>>) -> Option<f32> {
    let values = entries
        .into_iter()
        .flatten()
        .filter_map(|entry| entry.weight)
        .collect::<Vec<f32>>();
    if values.is_empty() {
        return None;
    }
    let sum: f32 = values.iter().sum();
    Some(sum / values.len() as f32)
}

use crate::prelude::*;

impl EntryCollection {
    #[allow(
        clippy::as_conversions,
        clippy::cast_possible_wrap,
        clippy::cast_precision_loss,
        clippy::cast_sign_loss,
        clippy::comparison_chain
    )]
    pub fn get_trend<F: Fn(&WeightStatistics) -> Option<f32>>(
        &self,
        period: usize,
        predicate: F,
    ) -> Option<Vec<(usize, f32)>> {
        let period = Duration::days(period as i64);
        let mut entries: Vec<_> = self
            .entries
            .iter()
            .filter_map(|(date, entry)| Some((entry, predicate(self.statistics.get(date)?)?)))
            .collect();
        entries.reverse();
        let mut values = Vec::new();
        let start = entries.first()?;
        values.push((start.0.date, start.1));
        let mut target = start.0.date - period;
        for pair in entries.windows(2) {
            let previous = pair.first().expect("should be two values");
            let current = pair.get(1).expect("should be two values");
            if current.0.date == target {
                values.push((current.0.date, current.1));
                target -= period;
            } else if current.0.date < target {
                let previous_delta = (previous.0.date - target).num_days() as f32;
                let current_delta = (previous.0.date - current.0.date).num_days() as f32;
                let modifier = previous_delta / current_delta;
                let diff = previous.1 - current.1;
                let value = previous.1 - diff * modifier;
                values.push((target, value));
                target -= period;
            }
        }
        let values = values
            .into_iter()
            .map(|(date, value)| (self.range.get_day(date), value))
            .collect();
        Some(values)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn _get_trend() {
        // Arrange
        let collection = EntryCollection::get_sample();
        // Act
        let trend = collection.get_trend(7, |x| x.sma1c).unwrap_or_default();
        // Assert
        let expect = Expect::new()
            .values(&trend)
            .expect("Expect should not fail");
        assert!(expect);
    }
}

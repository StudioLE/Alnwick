use crate::prelude::*;

pub struct ChartFactory {
    collection: EntryCollection,
    pub x_scale: f32,
    pub y_scale: f32,
}

impl ChartFactory {
    #[allow(clippy::as_conversions, clippy::cast_precision_loss)]
    pub fn new(collection: EntryCollection) -> Self {
        let days = collection.range.get_total_days() as f32;
        let weight_span = collection.range.max_weight - collection.range.min_weight;
        Self {
            collection,
            x_scale: 1.0,
            y_scale: days / weight_span,
        }
    }

    pub fn get_weight_scatter(&self) -> Vec<ScatterData> {
        self.collection
            .entries
            .values()
            .filter_map(|entry| {
                Some(ScatterData {
                    point: Point::new(
                        self.x_from_date(entry.date),
                        self.y_from_weight(entry.weight?),
                    ),
                    class: get_shot_class(entry.shot.as_ref()),
                    size: if entry.shot.is_some() { 0.75 } else { 0.50 },
                    descender: self.get_statistics(entry).sma1c.map(|descender| {
                        Point::new(self.x_from_date(entry.date), self.y_from_weight(descender))
                    }),
                })
            })
            .collect()
    }

    pub fn get_trend_points(&self) -> Vec<Point> {
        self.collection
            .get_trend(7, |x| x.sma1c)
            .unwrap_or_default()
            .into_iter()
            .map(|(day, value)| Point::new(self.x_from_day(day), self.y_from_weight(value)))
            .collect()
    }

    pub fn get_points_for<F: Fn(&WeightStatistics) -> Option<f32>>(
        &self,
        predicate: F,
    ) -> Vec<Point> {
        self.collection
            .entries
            .values()
            .filter_map(|entry| {
                Some(Point::new(
                    self.x_from_date(entry.date),
                    self.y_from_weight(predicate(&self.get_statistics(entry))?),
                ))
            })
            .collect()
    }

    pub fn get_statistics(&self, entry: &Entry) -> WeightStatistics {
        self.collection
            .statistics
            .get(&entry.date)
            .cloned()
            .unwrap_or_default()
    }

    #[allow(
        clippy::as_conversions,
        clippy::cast_precision_loss,
        clippy::cast_sign_loss
    )]
    fn x_from_date(&self, date: NaiveDate) -> f32 {
        self.collection.range.get_day(date) as f32 * self.x_scale
    }

    #[allow(
        clippy::as_conversions,
        clippy::cast_precision_loss,
        clippy::cast_sign_loss
    )]
    fn x_from_day(&self, day: usize) -> f32 {
        day as f32 * self.x_scale
    }

    fn y_from_weight(&self, weight: f32) -> f32 {
        (self.collection.range.max_weight - weight) * self.y_scale
    }
}

fn get_shot_class(shot: Option<&Shot>) -> Option<String> {
    let shot = shot?;
    Some(format!("shot d{}", shot.dose * 10.0))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_weight_scatter() {
        // Arrange
        let collection = EntryCollection::get_sample();
        let factory = ChartFactory::new(collection);
        // Act
        let data = factory.get_weight_scatter();
        // Assert
        let expect = Expect::new().values(&data).expect("Expect should not fail");
        assert!(expect);
    }

    #[test]
    fn get_trend_points() {
        // Arrange
        let collection = EntryCollection::get_sample();
        let factory = ChartFactory::new(collection);
        // Act
        let points = factory.get_trend_points();
        // Assert
        let expect = Expect::new()
            .values(&points)
            .expect("Expect should not fail");
        assert!(expect);
    }

    #[test]
    fn get_points_for_sma1c() {
        // Arrange
        let collection = EntryCollection::get_sample();
        let factory = ChartFactory::new(collection);
        // Act
        let points = factory.get_points_for(|statistics| statistics.sma1c);
        // Assert
        let expect = Expect::new()
            .values(&points)
            .expect("Expect should not fail");
        assert!(expect);
    }
}

use crate::prelude::*;

#[allow(clippy::integer_division, clippy::identity_op)]
const SMA1: usize = 1 * 7 - 1;
#[allow(clippy::integer_division, clippy::identity_op)]
const SMA1C: usize = (1 * 7) / 2;
#[allow(clippy::integer_division)]
const SMA2: usize = 2 * 7 - 1;
#[allow(clippy::integer_division)]
const SMA2C: usize = (2 * 7) / 2;
#[allow(clippy::integer_division)]
const SMA4: usize = 4 * 7 - 1;
#[allow(clippy::integer_division)]
const SMA4C: usize = (4 * 7) / 2;
#[allow(clippy::integer_division)]
const SMA8: usize = 8 * 7 - 1;
#[allow(clippy::integer_division)]
const SMA8C: usize = (8 * 7) / 2;

#[allow(dead_code)]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct WeightStatistics {
    /// Simple moving average over the past week
    pub sma1: Option<f32>,
    /// Simple moving average centered to 1 week
    pub sma1c: Option<f32>,
    /// Simple moving average over the past 2 weeks
    pub sma2: Option<f32>,
    /// Simple moving average centered over 2 weeks
    pub sma2c: Option<f32>,
    /// Simple moving average over the past 4 weeks
    pub sma4: Option<f32>,
    /// Simple moving average centered over 4 weeks
    pub sma4c: Option<f32>,
    /// Simple moving average over the past 8 weeks
    pub sma8: Option<f32>,
    /// Simple moving average centered over 8 weeks
    pub sma8c: Option<f32>,
}

impl EntryCollection {
    pub fn calculate_weight_statistics(&self, date: NaiveDate) -> WeightStatistics {
        WeightStatistics {
            sma1: self.calculate_simple_moving_average(date, SMA1),
            sma1c: self.calculate_simple_moving_average_centered(date, SMA1C, SMA1C),
            sma2: self.calculate_simple_moving_average(date, SMA2),
            sma2c: self.calculate_simple_moving_average_centered(date, SMA2C, SMA2C),
            sma4: self.calculate_simple_moving_average(date, SMA4),
            sma4c: self.calculate_simple_moving_average_centered(date, SMA4C, SMA4C),
            sma8: self.calculate_simple_moving_average(date, SMA8),
            sma8c: self.calculate_simple_moving_average_centered(date, SMA8C, SMA8C),
        }
    }
}

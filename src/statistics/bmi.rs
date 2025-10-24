pub struct BodyMassIndex;

impl BodyMassIndex {
    #[allow(dead_code)]
    pub const UNDERWEIGHT_C3: f32 = 16.0;
    #[allow(dead_code)]
    pub const UNDERWEIGHT_C2: f32 = 17.0;
    #[allow(dead_code)]
    pub const UNDERWEIGHT_C1: f32 = 18.5;
    pub const CENTER: f32 = 21.75;
    pub const HEALTHY: f32 = 25.0;
    pub const OVERWEIGHT: f32 = 30.0;
    #[allow(dead_code)]
    pub const OBESE_C1: f32 = 35.0;
    #[allow(dead_code)]
    pub const OBESE_C2: f32 = 40.0;

    #[allow(dead_code)]
    pub fn from_height_weight(height: f32, weight: f32) -> f32 {
        weight / height.powf(2.0)
    }

    pub fn weight_from_height_bmi(height: f32, bmi: f32) -> f32 {
        bmi * height.powf(2.0)
    }
}

#[cfg(test)]
mod tests {
    use super::BodyMassIndex;
    use float_eq::assert_float_eq;

    const EPSILON: f32 = 0.0001;

    #[test]
    fn from_height_weight() {
        assert_float_eq!(
            BodyMassIndex::from_height_weight(1.75, 68.0),
            22.204_08,
            abs <= EPSILON
        );
        assert_float_eq!(
            BodyMassIndex::from_height_weight(1.62, 90.0),
            34.293_552,
            abs <= EPSILON
        );
    }

    #[test]
    fn weight_from_height_bmi() {
        assert_float_eq!(
            BodyMassIndex::weight_from_height_bmi(1.62, 19.0),
            49.863_598,
            abs <= EPSILON
        );
        assert_float_eq!(
            BodyMassIndex::weight_from_height_bmi(1.62, 25.0),
            65.61,
            abs <= EPSILON
        );
        assert_float_eq!(
            BodyMassIndex::weight_from_height_bmi(1.62, 30.0),
            78.731_995,
            abs <= EPSILON
        );
        assert_float_eq!(
            BodyMassIndex::weight_from_height_bmi(1.62, 40.0),
            104.976,
            abs <= EPSILON
        );
    }
}

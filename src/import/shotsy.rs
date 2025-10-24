use crate::prelude::*;

const LB_TO_KG: f32 = 0.453_592_37;

#[allow(dead_code)]
#[derive(Clone, Debug, Deserialize)]
pub(crate) struct ShotsyData {
    #[serde(rename = "Date (UTC)")]
    pub date: Option<NaiveDate>,

    #[serde(rename = "Shot")]
    pub shot: Option<String>,

    /// Time of shot
    #[serde(rename = "Time (UTC)")]
    pub time: Option<NaiveTime>,

    #[serde(rename = "Site")]
    pub site: Option<String>,

    #[serde(rename = "Shot Notes")]
    pub shot_notes: Option<String>,

    /// Weight recorded in lb
    #[serde(rename = "Recorded Weight")]
    pub weight: Option<f32>,

    #[serde(rename = "Calories")]
    pub calories: Option<String>,

    #[serde(rename = "Protein")]
    pub protein: Option<String>,

    #[serde(rename = "Water")]
    pub water: Option<String>,

    #[serde(rename = "Day Notes")]
    pub day_notes: Option<String>,

    #[serde(rename = "Nausea")]
    pub nausea: Option<String>,

    #[serde(rename = "Heartburn")]
    pub heartburn: Option<String>,

    #[serde(rename = "Food Noise")]
    pub food_noise: Option<String>,

    #[serde(rename = "Suppressed Appetite")]
    pub suppressed_appetite: Option<String>,

    #[serde(rename = "Constipation")]
    pub constipation: Option<String>,

    #[serde(rename = "Diarrhea")]
    pub diarrhea: Option<String>,

    #[serde(rename = "Belching")]
    pub belching: Option<String>,

    #[serde(rename = "Injection Site Reaction")]
    pub injection_site_reaction: Option<String>,

    #[serde(rename = "Mood Swings")]
    pub mood_swings: Option<String>,

    #[serde(rename = "Indigestion")]
    pub indigestion: Option<String>,

    #[serde(rename = "Metallic Taste")]
    pub metallic_taste: Option<String>,

    #[serde(rename = "Stomach Pain")]
    pub stomach_pain: Option<String>,

    #[serde(rename = "Hair Loss")]
    pub hair_loss: Option<String>,

    #[serde(rename = "Fatigue")]
    pub fatigue: Option<String>,

    #[serde(rename = "Migraine")]
    pub migraine: Option<String>,
}

impl ShotsyData {
    pub(crate) fn from_csv(content: &str) -> Vec<ShotsyData> {
        let mut reader = csv::Reader::from_reader(content.as_bytes());
        reader
            .deserialize::<ShotsyData>()
            .enumerate()
            .filter_map(|(i, result)| {
                result.handle_error(|e| warn!("Failed to read line {}: {e}", i + 1))
            })
            .collect()
    }

    #[allow(clippy::wrong_self_convention)]
    pub(crate) fn to_entries(data: Vec<ShotsyData>) -> Vec<Entry> {
        data.into_iter()
            .filter_map(|shotsy| {
                let entry = shotsy.clone().to_entry();
                if entry.is_none() {
                    warn!("Entry did not contain shot or weight data: {shotsy:?}");
                }
                entry
            })
            .collect()
    }

    fn to_entry(self) -> Option<Entry> {
        Some(Entry {
            date: self.date?,
            weight: self.weight.map(|x| x * LB_TO_KG),
            shot: self.to_shot(),
        })
    }

    fn to_shot(self) -> Option<Shot> {
        Some(Shot {
            time: self.time?,
            medication: get_medication(self.shot.clone())?,
            dose: get_dose(self.shot)?,
            site: self.site,
            notes: self.shot_notes,
        })
    }
}

fn get_medication(input: Option<String>) -> Option<Medication> {
    input?
        .starts_with("Mounjaro")
        .then_some(Medication::Mounjaro)
}

fn get_dose(input: Option<String>) -> Option<f32> {
    let regex = Regex::new(r" (\d+\.\d+) mg$").expect("Regex should be valid");
    let input = input?;
    let value = regex.captures(&input)?.get(1)?.as_str();
    value.parse().ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn import() {
        // Arrange
        let csv = include_str!("../../samples/shotsy.csv");
        // Act
        let data = ShotsyData::from_csv(csv);
        // Assert
        assert_eq!(data.len(), 144);
        // Act
        let entries = ShotsyData::to_entries(data);
        // Assert
        let expect = Expect::new()
            .values(&entries)
            .expect("Expect should not fail");
        assert!(expect);
    }

    #[test]
    fn _get_dose() {
        assert_eq!(get_dose(Some("Mounjaro® 2.5 mg".to_owned())), Some(2.5));
        assert_eq!(get_dose(Some("Mounjaro® 2.9 mg".to_owned())), Some(2.9));
        assert_eq!(
            get_dose(Some("Mounjaro® 3.33333 mg".to_owned())),
            Some(3.33333)
        );
        assert_eq!(get_dose(Some("Mounjaro® 5.0 mg".to_owned())), Some(5.0));
        assert_eq!(get_dose(Some("Mounjaro® 7.5 mg".to_owned())), Some(7.5));
        assert_eq!(get_dose(Some("Mounjaro® 10.0 mg".to_owned())), Some(10.0));
        assert_eq!(get_dose(Some("Mounjaro® 12.5 mg".to_owned())), Some(12.5));
        assert_eq!(get_dose(Some("Mounjaro® 15.0 mg".to_owned())), Some(15.0));
    }
}

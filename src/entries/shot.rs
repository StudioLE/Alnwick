use crate::prelude::*;

pub const SHOT_25: &str = NEUTRAL_600;
pub const SHOT_50: &str = PURPLE_900;
pub const SHOT_75: &str = TEAL_700;
pub const SHOT_100: &str = PINK_600;
pub const SHOT_125: &str = SKY_500;
pub const SHOT_150: &str = RED_400;
pub const SHOT_UNKNOWN: &str = SLATE_500;
pub const SHOT_NONE: &str = GRAY_600;

#[allow(dead_code)]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Shot {
    pub time: NaiveTime,
    pub medication: Medication,
    pub dose: f32,
    pub site: Option<String>,
    pub notes: Option<String>,
}

impl Shot {
    #[allow(dead_code, clippy::float_cmp)]
    pub fn get_color(shot: Option<&Shot>) -> String {
        let color = if let Some(shot) = shot {
            if shot.dose == 2.5 {
                SHOT_25
            } else if shot.dose == 5.0 {
                SHOT_50
            } else if shot.dose == 7.5 {
                SHOT_75
            } else if shot.dose == 10.0 {
                SHOT_100
            } else if shot.dose == 12.5 {
                SHOT_125
            } else if shot.dose == 15.0 {
                SHOT_150
            } else {
                SHOT_UNKNOWN
            }
        } else {
            SHOT_NONE
        };
        color.to_owned()
    }
}

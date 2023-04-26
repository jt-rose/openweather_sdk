use serde::{ Serialize, Deserialize};
use std::fmt;

#[derive(Debug, Serialize, Deserialize, PartialOrd, PartialEq, Default, Copy, Clone)]
pub struct Snow {
    #[serde(alias = "3h")]
    pub volume_over_three_hours: Option<f64>,
    #[serde(alias = "1h")]
    pub volume_over_last_hour: Option<f64>
}

impl fmt::Display for Snow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        let volume_over_three_hours = match self.volume_over_three_hours {
            Some(volume) => volume.to_string(),
            None => "None".to_string()
        };

        let volume_over_last_hour = match self.volume_over_last_hour {
            Some(volume) => volume.to_string(),
            None => "None".to_string()
        };

        write!(
            f,
            "Snow: (volume_over_three_hours: {}, volume_over_last_hour: {})",
            volume_over_three_hours,
            volume_over_last_hour,
        )
    }
}
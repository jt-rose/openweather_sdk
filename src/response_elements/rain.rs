use serde::{ Serialize, Deserialize};
use std::fmt;

#[derive(Debug, Serialize, Deserialize, PartialOrd, PartialEq, Default, Copy, Clone)]
pub struct Rain {
    #[serde(alias = "3h")]
    pub volume_over_three_hours: f64
}

impl fmt::Display for Rain {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Rain: (volume_over_three_hours: {})",
            self.volume_over_three_hours,
        )
    }
}
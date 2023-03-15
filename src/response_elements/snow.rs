use serde::{ Serialize, Deserialize};
use std::fmt;

#[derive(Debug, Serialize, Deserialize, PartialOrd, PartialEq, Default, Copy, Clone)]
pub struct Snow {
    #[serde(alias = "1h")]
    pub volume_over_last_hour: f64
}

impl fmt::Display for Snow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Snow: (volume_over_last_hour: {})",
            self.volume_over_last_hour,
        )
    }
}
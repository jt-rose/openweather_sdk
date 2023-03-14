use serde::{ Serialize, Deserialize};
use std::fmt;

#[derive(Debug, Serialize, Deserialize, PartialOrd, PartialEq, Default, Copy, Clone)]
pub struct Wind {
    pub speed: f64,
    pub deg: i64,
    pub gust: f64
}

impl fmt::Display for Wind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Wind: (speed: {}, degree: {}, gust: {})",
            self.speed,
            self.deg,
            self.gust
        )
    }
}
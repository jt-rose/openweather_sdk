use serde::{ Serialize, Deserialize};
use std::fmt;

#[derive(Debug, Serialize, Deserialize)]
pub struct Temp {
    pub day: f64,
    pub min: f64,
    pub max: f64,
    pub night: f64,
    pub eve: f64,
    pub morn: f64
}

impl fmt::Display for Temp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Temp: (day: {}, min: {}, max: {}, night: {}, eve: {}, morn: {})",
            self.day,
            self.min,
            self.max,
            self.night,
            self.eve,
            self.morn
        )
    }
}
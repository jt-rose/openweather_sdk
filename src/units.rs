use std::fmt;
use serde::{ Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq, Hash, Copy, Clone)]
pub enum Units {
    Standard,
    Metric,
    Imperial,
}

impl fmt::Display for Units {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Standard => "standard",
                Self::Metric => "metric",
                Self::Imperial => "imperial"
            }
        )
    }
}

impl Default for Units {
    fn default() -> Self {
        Self::Standard
    }
}
use std::fmt;

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
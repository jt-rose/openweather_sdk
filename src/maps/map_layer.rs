use std::fmt;
use serde::{ Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, PartialOrd, PartialEq, Copy, Clone)]
pub enum MapLayer {
    Clouds,
    Precipitation,
    Pressure,
    Wind,
    Temperature,
}

impl fmt::Display for MapLayer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                MapLayer::Clouds => "clouds_new",
                MapLayer::Precipitation => "precipitation_new",
                MapLayer::Pressure => "pressure_new",
                MapLayer::Wind => "wind_new",
                MapLayer::Temperature => "temp_new",
            }
        )
    }
}

impl Default for MapLayer {
    fn default() -> Self {
        MapLayer::Temperature
    }
}
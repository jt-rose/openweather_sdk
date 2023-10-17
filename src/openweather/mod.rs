pub mod air_pollution;
pub mod forecast;
pub mod one_call;
pub mod geocoding;
pub mod maps;

pub use air_pollution::AirPollution;
pub use forecast::Forecast;
pub use one_call::{OneCall, Fields};
pub use geocoding::Geocoding;
pub use maps::Maps;

use std::fmt;
use serde::{ Serialize, Deserialize};
use crate::units::Units;
use crate::languages::Language;

#[derive(Debug, Serialize, Deserialize, PartialOrd, PartialEq, Default, Clone)]
pub struct OpenWeather {
    pub one_call: OneCall,
    pub forecast: Forecast,
    pub maps: Maps,
    pub air_pollution: AirPollution,
    pub geocoding: Geocoding
}

impl fmt::Display for OpenWeather {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "OpenWeather: (one_call: {}, forecast: {}, maps: {}, air_pollution: {}, geocoding: {})",
            self.one_call,
            self.forecast,
            self.maps,
            self.air_pollution,
            self.geocoding
        )
    }
}

impl OpenWeather {
    pub fn new(api_key: String, units: Units, language: Language) -> Self {
        Self {
            one_call: OneCall::new(api_key.clone(), units, language),
            forecast: Forecast::new(api_key.clone(), units, language),
            maps: Maps::new(api_key.clone()),
            air_pollution: AirPollution::new(api_key.clone()),
            geocoding: Geocoding::new(api_key.clone())
        }
    }
}

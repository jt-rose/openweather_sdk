use std::fmt;
use crate::air_pollution::air_pollution::AirPollution;
use crate::units::Units;
use crate::languages::Language;
use crate::one_call::one_call::OneCall;
use crate::forecast::forecast::Forecast;
use crate::geocoding::geocoding::Geocoding;
use crate::maps::maps::Maps;

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
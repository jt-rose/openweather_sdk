use crate::air_pollution::air_pollution::AirPollution;
use crate::units::Units;
use crate::languages::Language;
use crate::onecall::onecall::OneCall;
use crate::forecast::forecast::Forecast;
use crate::geocoding::geocoding::Geocoding;
use crate::maps::maps::Maps;

pub struct OpenWeather {
    pub onecall: OneCall,
    pub forecast: Forecast,
    pub maps: Maps,
    pub air_pollution: AirPollution,
    pub geocoding: Geocoding
}

impl OpenWeather {
    pub fn new(api_key: String, units: Units, language: Language) -> Self {
        Self {
            onecall: OneCall::new(api_key.clone(), units, language),
            forecast: Forecast::new(api_key.clone(), units, language),
            maps: Maps::new(api_key.clone()),
            air_pollution: AirPollution::new(api_key.clone()),
            geocoding: Geocoding::new(api_key.clone())
        }
    }
}
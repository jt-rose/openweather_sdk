use crate::units::Units;
use crate::languages::Language;
use crate::onecall::onecall::OneCall;
use crate::forecast::forecast::Forecast;
use crate::maps::maps::Maps;

pub struct OpenWeather {
    api_key: &'static str,
    units: Units,
    language: Language,
    pub onecall: OneCall,
    pub forecast: Forecast,
    pub maps: Maps
}

impl OpenWeather {
    pub fn new(api_key: &'static str, units: Units, language: Language) -> Self {
        Self {
            api_key,
            units,
            language,
            onecall: OneCall::new(api_key, units, language),
            forecast: Forecast::new(api_key, units, language),
            maps: Maps::new(api_key, units, language)
        }
    }
}
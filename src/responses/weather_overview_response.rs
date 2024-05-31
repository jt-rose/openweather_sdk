use serde::{ Serialize, Deserialize };
use std::fmt;

#[derive(Debug, Serialize, Deserialize, PartialOrd, PartialEq, Default, Clone)]
pub struct WeatherOverviewResponse {
    pub lat: f64,
    pub lon: f64,
    pub tz: String,
    pub date: String,
    pub units: String,
    pub weather_overview: String,
}

impl fmt::Display for WeatherOverviewResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "WeatherOverviewResponse: (lat: {}, lon: {}, tz: {}, date: {}, units: {}, weather_overview: {})",
            self.lat,
            self.lon,
            self.tz,
            self.date,
            self.units,
            self.weather_overview
        )
    }
}

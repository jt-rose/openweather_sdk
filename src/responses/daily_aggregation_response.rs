use serde::{ Serialize, Deserialize };
use std::fmt;

#[derive(Debug, Serialize, Deserialize, PartialOrd, PartialEq, Default, Clone)]
pub struct Afternoon {
    afternoon: f64
}

impl fmt::Display for Afternoon {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "(afternoon: {})",
            self.afternoon,
        )
    }
}

#[derive(Debug, Serialize, Deserialize, PartialOrd, PartialEq, Default, Clone)]
pub struct Precipitation {
    total: f64
}

impl fmt::Display for Precipitation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "(total: {})",
            self.total,
        )
    }
}

#[derive(Debug, Serialize, Deserialize, PartialOrd, PartialEq, Default, Clone)]
pub struct Temperature {
    min: f64,
    max: f64,
    afternoon: f64,
    night: f64,
    evening: f64,
    morning: f64
}

impl fmt::Display for Temperature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "(min: {}, max: {}, afternoon: {}, night: {}, evening: {}, morning: {})",
            self.min,
            self.max,
            self.afternoon,
            self.night,
            self.evening,
            self.morning
        )
    }
}

#[derive(Debug, Serialize, Deserialize, PartialOrd, PartialEq, Default, Clone)]
pub struct Max {
    speed: f64,
    direction: f64
}

impl fmt::Display for Max {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "(speed: {}, direction: {})",
            self.speed,
            self.direction,
        )
    }
}

#[derive(Debug, Serialize, Deserialize, PartialOrd, PartialEq, Default, Clone)]
pub struct Wind {
    max: Max
}

impl fmt::Display for Wind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "({})",
            self.max
        )
    }
}

#[derive(Debug, Serialize, Deserialize, PartialOrd, PartialEq, Default, Clone)]
pub struct DailyAggregationResponse {
    pub lat: f64,
    pub lon: f64,
    pub tz: String,
    pub date: String,
    pub units: String,
    pub cloud_cover: Afternoon,
    pub humidity: Afternoon,
    pub precipitation: Precipitation,
    pub temperature: Temperature,
    pub pressure: Afternoon,
    pub wind: Wind
}

impl fmt::Display for DailyAggregationResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "DailyAggregationResponse: (lat: {}, lon: {}, tz: {}, date: {}, units: {}, cloud_cover: {}, humidity: {}, precipitation: {}, temperature: {}, pressure: {}, wind: {})",
            self.lat,
            self.lon,
            self.tz,
            self.date,
            self.units,
            self.cloud_cover,
            self.humidity,
            self.precipitation,
            self.temperature,
            self.pressure,
            self.wind
        )
    }
}

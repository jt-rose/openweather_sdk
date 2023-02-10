use serde::{ Serialize, Deserialize };
use super::weather::Weather;

#[derive(Debug, Serialize, Deserialize)]
pub struct HistoricalData {
    #[serde(alias = "dt")]
    pub datetime: i64,
    pub sunrise: i64,
    pub sunset: i64,
    pub temp: f64,
    pub feels_like: f64,
    pub pressure: i64,
    pub humidity: i64,
    pub dew_point: f64,
    pub uvi: Option<f64>,
    pub clouds: i64,
    pub visibility: Option<i64>,
    pub wind_speed: f64,
    pub wind_deg: i64,
    pub weather: Vec<Weather>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HistoricalResponse {
    pub lat: f64,
    pub lon: f64,
    pub timezone: String,
    pub timezone_offset: i64,
    pub data: Vec<HistoricalData>,
}
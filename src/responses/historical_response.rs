use serde::Deserialize;
use super::weather::Weather;

#[derive(Debug, Deserialize)]
struct HistoricalData {
    #[serde(alias = "dt")]
    datetime: i64,
    sunrise: i64,
    sunset: i64,
    temp: f64,
    feels_like: f64,
    pressure: i64,
    humidity: i64,
    dew_point: f64,
    uvi: Option<f64>,
    clouds: i64,
    visibility: Option<i64>,
    wind_speed: f64,
    wind_deg: i64,
    weather: Vec<Weather>
}

#[derive(Debug, Deserialize)]
pub struct HistoricalResponse {
    lat: f64,
    lon: f64,
    timezone: String,
    timezone_offset: i64,
    data: Vec<HistoricalData>,
}
use serde::{ Serialize, Deserialize };
use super::weather::Weather;

#[derive(Debug, Serialize, Deserialize)]
pub struct Rain {
    #[serde(alias = "1h")]
    pub volume_over_last_hour: f64
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Snow {
    #[serde(alias = "1h")]
    pub volume_over_last_hour: f64
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Current {
    pub dt: i64,
    pub sunrise: i64,
    pub sunset: i64,
    pub temp: f64,
    pub feels_like: f64,
    pub pressure: u64,
    pub humidity: u64,
    pub dew_point: f64,
    pub uvi: f64,
    pub clouds: u64,
    pub visibility: i64,
    pub wind_speed: f64,
    pub wind_deg: u64,
    pub wind_gust: Option<f64>,
    pub rain: Option<Rain>,
    pub snow: Option<Snow>,
    pub weather: Vec<Weather>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Temp {
    pub day: f64,
    pub min: f64,
    pub max: f64,
    pub night: f64,
    pub eve: f64,
    pub morn: f64
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeelsLike {
    pub day: f64,
    pub night: f64,
    pub eve: f64,
    pub morn: f64
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Daily {
    #[serde(alias = "dt")]
    pub datetime: i64,
    pub sunrise: i64,
    pub sunset: i64,
    pub moonrise: i64,
    pub moonset: i64,
    pub moon_phase: f64,
    pub temp: Temp,
    pub feels_like: FeelsLike,
    pub pressure: i64,
    pub humidity: u64,
    pub dew_point: f64,
    pub wind_speed: f64,
    pub wind_deg: i64,
    pub wind_gust: Option<f64>,
    pub weather: Vec<Weather>,
    pub clouds: i64,
    pub pop: f64,
    pub rain: Option<f64>,
    pub snow: Option<f64>,
    pub uvi: f64
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Hourly {
    #[serde(alias = "dt")]
    pub datetime: i64,
    pub temp: f64,
    pub feels_like: f64,
    pub pressure: u64,
    pub humidity: u64,
    pub dew_point: f64,
    pub uvi: f64,
    pub clouds: u64,
    pub visibility: i64,
    pub wind_speed: f64,
    pub wind_deg: u64,
    pub wind_gust: Option<f64>,
    pub rain: Option<Rain>,
    pub snow: Option<Snow>,
    pub weather: Vec<Weather>,
    pub pop: f64
    }

#[derive(Debug, Serialize, Deserialize)]
pub struct Minutely {
    #[serde(alias = "dt")]
    pub datetime: i64,
    pub precipitation: u64
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Alert {
    pub sender_name: String,
    pub event: String,
    pub start: i64,
    pub end: i64,
    pub description: String,
    pub tags: Vec<String>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OneCallResponse {
    pub lat: f64,
    pub lon: f64,
    pub timezone: String,
    pub timezone_offset: isize,
    pub current: Option<Current>,
    pub minutely: Option<Vec<Minutely>>,
    pub hourly: Option<Vec<Hourly>>,
    pub daily: Option<Vec<Daily>>,
    pub alerts: Option<Vec<Alert>>
}
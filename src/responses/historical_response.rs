use serde::{ Serialize, Deserialize };
use super::weather::Weather;
use crate::utils::display_option;
use std::fmt;

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

impl fmt::Display for HistoricalData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut weather_list = String::new();
        weather_list.push_str("[");
        for weather in &self.weather {
            weather_list.push_str(&format!("{}, ", weather));
        }
        weather_list.push_str("]");

        write!(
            f,
            "HistoricalData: (datetime: {}, sunrise: {}, sunset: {}, temp: {}, feels_like: {}, pressure: {}, humidity: {}, dew_point: {}, uvi: {}, clouds: {}, visibility: {}, wind_speed: {}, wind_deg: {}, weather: {})",
            self.datetime,
            self.sunrise,
            self.sunset,
            self.temp,
            self.feels_like,
            self.pressure,
            self.humidity,
            self.dew_point,
            display_option(&self.uvi),
            self.clouds,
            display_option(&self.visibility),
            self.wind_speed,
            self.wind_deg,
            weather_list
        )
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HistoricalResponse {
    pub lat: f64,
    pub lon: f64,
    pub timezone: String,
    pub timezone_offset: i64,
    pub data: Vec<HistoricalData>,
}

impl fmt::Display for HistoricalResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut data_list = String::new();
        data_list.push_str("[");
        for data in &self.data {
            data_list.push_str(&format!("{}, ", data));
        }
        data_list.push_str("]");

        write!(
            f,
            "HistoricalResponse: (lat: {}, lon: {}, timezone: {}, timezone_offset: {}, data: {})",
            self.lat,
            self.lon,
            self.timezone,
            self.timezone_offset,
            data_list
        )
    }
}
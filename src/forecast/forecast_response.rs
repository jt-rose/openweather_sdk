use serde::{ Serialize, Deserialize};
use crate::responses::weather::Weather;

#[derive(Debug, Serialize, Deserialize)]
pub struct Wind {
    pub speed: f64,
    pub deg: i64,
    pub gust: f64
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Clouds {
    pub all: i64
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Rain {
    #[serde(alias = "3h")]
    pub volume_over_three_hours: f64
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Sys {
    pub pod: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Main {
    pub temp: f64,
    pub feels_like: f64,
    pub temp_min: f64,
    pub temp_max: f64,
    pub pressure: u64,
    pub sea_level: i64,
    #[serde(alias = "grnd_level")]
    pub ground_level: i64,
    pub humidity: u64,
    pub temp_kf: f64
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForecastDescription {
    #[serde(alias = "dt")]
    pub datetime: u64,
    pub main: Main,
    pub weather: Vec<Weather>,
    pub clouds: Clouds,
    pub wind: Wind,
    pub visibility: i64,
    pub pop: f64,
    pub rain: Option<Rain>,
    pub sys: Sys,
    pub dt_txt: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct City {
    pub id: u64,
    pub name: String,
    pub coord: Coord,
    pub country: String,
    pub population: u64,
    pub timezone: i64,
    pub sunrise: u64,
    pub sunset: u64
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Coord {
    pub lat: f64,
    pub lon: f64
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForecastResponse {
    pub cod: String,
    pub message: i64,
    pub cnt: i64,
    pub list: Vec<ForecastDescription>,
    pub city: City,
}
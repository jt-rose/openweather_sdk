use serde::Deserialize;
use crate::responses::weather::Weather;

#[derive(Debug, Deserialize)]
struct Wind {
    speed: f64,
    deg: i64,
    gust: f64
}

#[derive(Debug, Deserialize)]
struct Clouds {
    all: i64
}

#[derive(Debug, Deserialize)]
struct Rain {
    #[serde(alias = "3h")]
    volume_over_three_hours: f64
}

#[derive(Debug, Deserialize)]
struct Sys {
    pod: String
}

#[derive(Debug, Deserialize)]
struct Main {
    temp: f64,
    feels_like: f64,
    temp_min: f64,
    temp_max: f64,
    pressure: u64,
    sea_level: i64,
    #[serde(alias = "grnd_level")]
    ground_level: i64,
    humidity: u64,
    temp_kf: f64
}

#[derive(Debug, Deserialize)]
struct ForecastDescription {
    #[serde(alias = "dt")]
    datetime: u64,
    main: Main,
    weather: Vec<Weather>,
    clouds: Clouds,
    wind: Wind,
    visibility: i64,
    pop: f64,
    rain: Option<Rain>,
    sys: Sys,
    dt_txt: String
}

#[derive(Debug, Deserialize)]
struct City {
    id: u64,
    name: String,
    coord: Coord,
    country: String,
    population: u64,
    timezone: i64,
    sunrise: u64,
    sunset: u64
}

#[derive(Debug, Deserialize)]
struct Coord {
    lat: f64,
    lon: f64
}

#[derive(Debug, Deserialize)]
pub struct ForecastResponse {
    cod: String,
    message: i64,
    cnt: i64,
    list: Vec<ForecastDescription>,
    city: City,
}
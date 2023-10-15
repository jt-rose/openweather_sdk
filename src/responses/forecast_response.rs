use serde::{ Serialize, Deserialize};
use crate::responses::response_elements::Weather;
use std::fmt;
use std::fmt::Formatter;
use crate::responses::response_elements::Clouds;
use crate::responses::response_elements::Rain;
use crate::responses::response_elements::Wind;
use crate::responses::response_elements::Coord;

#[derive(Debug, Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq, Hash, Default, Clone)]
pub struct Sys {
    pub pod: String
}

impl fmt::Display for Sys {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "System: (pod: {})",
            self.pod,
        )
    }
}

#[derive(Debug, Serialize, Deserialize, PartialOrd, PartialEq,  Default, Copy, Clone)]
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

impl fmt::Display for Main {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Main: (temp: {}, feels_like: {}, temp_min: {}, temp_max: {}, pressure: {}, sea_level: {}, ground_level: {}, humidity: {}, temp_kf: {})",
            self.temp,
            self.feels_like,
            self.temp_min,
            self.temp_max,
            self.pressure,
            self.sea_level,
            self.ground_level,
            self.humidity,
            self.temp_kf
        )
    }
}

#[derive(Debug, Serialize, Deserialize, PartialOrd, PartialEq, Default, Clone)]
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

impl fmt::Display for ForecastDescription {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut weather_list = String::new();
        weather_list.push_str("[ ");
        for weather in &self.weather {
            weather_list.push_str(&format!("{}", weather));
        }
        weather_list.push_str(" ]");

        let rain_string = match &self.rain {
            Some(rain) => format!("{}", rain),
            None => "None".to_string()
        };

        write!(
            f,
            "ForecastDescription: (datetime: {}, main: {}, weather: {}, clouds: {}, wind: {}, visibility: {}, pop: {}, rain: {}, sys: {}, dt_txt: {})",
            self.datetime,
            self.main,
            weather_list,
            self.clouds,
            self.wind,
            self.visibility,
            self.pop,
            rain_string,
            self.sys,
            self.dt_txt
        )
    }
}

#[derive(Debug, Serialize, Deserialize, PartialOrd, PartialEq, Default, Clone)]
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

impl fmt::Display for City {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "City: (country: {}, name: {}, id: {}, coord: {}, population: {}, timezone: {}, sunrise: {}, sunset: {})",
            self.country,
            self.name,
            self.id,
            self.coord,
            self.population,
            self.timezone,
            self.sunrise,
            self.sunset
        )
    }
}

#[derive(Debug, Serialize, Deserialize, PartialOrd, PartialEq, Default, Clone)]
pub struct ForecastResponse {
    pub cod: String,
    pub message: i64,
    pub cnt: i64,
    pub list: Vec<ForecastDescription>,
    pub city: City,
}

impl fmt::Display for ForecastResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut list_items = String::new();
        list_items.push('[');
        for item in &self.list {
            list_items.push_str(&format!("{}, ", item));
        }
        list_items.push(']');
        write!(
            f,
            "ForecastResponse: (cod: {}, message: {}, cnt: {}, list: {}, city: {})",
            self.cod,
            self.message,
            self.cnt,
            list_items,
            self.city
        )
    }
}

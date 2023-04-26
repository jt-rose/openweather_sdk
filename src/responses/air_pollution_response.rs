use std::fmt;
use std::fmt::Formatter;
use serde::{Serialize, Deserialize};
use crate::responses::response_elements::Coord;

#[derive(Debug, Serialize, Deserialize, PartialOrd, PartialEq, Default, Copy, Clone)]
pub struct Components {
    pub co: f64,
    pub no: f64,
    pub no2: f64,
    pub o3: f64,
    pub so2: f64,
    pub pm2_5: f64,
    pub pm10: f64,
    pub nh3: f64
}

impl fmt::Display for Components {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "(co: {}, no: {}, no2: {}, o3: {}, so2: {}, pm2_5: {}, pm10: {}, nh3: {})",
            self.co,
            self.no,
            self.no2,
            self.o3,
            self.so2,
            self.pm2_5,
            self.pm10,
            self.nh3
        )
    }
}

#[derive(Debug, Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq, Default, Copy, Clone)]
pub struct Main {
    pub aqi: u64
}

impl fmt::Display for Main {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "aqi: {}",
            self.aqi
        )
    }
}

#[derive(Debug, Serialize, Deserialize, PartialOrd, PartialEq, Default, Copy, Clone)]
pub struct ListItem {
    #[serde(alias = "dt")]
    pub datetime: u64,
    pub main: Main,
    pub components: Components
}

impl fmt::Display for ListItem {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "datetime: {}, main: {}, components: {}",
            self.datetime,
            self.main,
            self.components
        )
    }
}

#[derive(Debug, Serialize, Deserialize, PartialOrd, PartialEq, Default, Clone)]
pub struct AirPollutionResponse {
    pub coord: Coord,
    pub list: Vec<ListItem>
}

impl fmt::Display for AirPollutionResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut list_items = String::new();
        list_items.push('[');
        for item in &self.list {
            list_items.push_str(&format!("{}, ", item));
            list_items.push_str(", ");
        }
        list_items.push(']');

        write!(
            f,
            "coord: {}, list: {}",
            self.coord,
            list_items
        )
    }
}
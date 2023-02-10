use serde::{ Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Coord {
    pub lat: f64,
    pub lon: f64
}

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
pub struct Main {
    pub aqi: u64
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListItem {
    #[serde(alias = "dt")]
    pub datetime: u64,
    pub main: Main,
    pub components: Components
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AirPollutionResponse {
    pub coord: Coord,
    pub list: Vec<ListItem>
}
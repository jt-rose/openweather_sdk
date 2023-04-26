use std::fmt;
use serde::{Serialize, Deserialize};
use crate::responses::AirPollutionResponse;

#[derive(Debug, Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq, Hash, Default, Clone)]
pub struct AirPollution {
    api_key: String,
}

impl fmt::Display for AirPollution {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "AirPollution: (api_key: {}, methods: [new, get_current_air_pollution, get_forecast_air_pollution, get_historical_air_pollution])",
            self.api_key
        )
    }
}

impl AirPollution {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
        }
    }

    fn format_query(&self, lat: f64, lon: f64, forecast_qualifier: &str) -> String {
        format!("https://api.openweathermap.org/data/2.5/air_pollution{}?lat={}&lon={}&appid={}",
                forecast_qualifier,
                lat,
                lon,
                self.api_key)
    }

    fn format_historical_query(&self, lat: f64, lon: f64, start: u64, end: u64) -> String {
        format!("https://api.openweathermap.org/data/2.5/air_pollution/history?lat={}&lon={}&start={}&end={}&appid={}",
            lat,
            lon,
            start,
            end,
            self.api_key)
    }

    pub async fn get_current_air_pollution(&self, lat: f64, lon: f64) -> Result<AirPollutionResponse, Box<dyn std::error::Error>> {
        let url = self.format_query(lat, lon, "");
        let resp = reqwest::get(url)
            .await?
            .json::<AirPollutionResponse>()
            .await?;

        Ok(resp)

    }

    pub async fn get_forecast_air_pollution(&self, lat: f64, lon: f64) -> Result<AirPollutionResponse, Box<dyn std::error::Error>> {
        let url = self.format_query(lat, lon, "/forecast");
        let resp = reqwest::get(url)
            .await?
            .json::<AirPollutionResponse>()
            .await?;

        Ok(resp)
    }

    pub async fn get_historical_air_pollution(&self, lat: f64, lon: f64, start: u64, end: u64) -> Result<AirPollutionResponse, Box<dyn std::error::Error>> {
        let url = self.format_historical_query(lat, lon, start, end);
        let resp = reqwest::get(url)
            .await?
            .json::<AirPollutionResponse>()
            .await?;

        Ok(resp)
    }
}
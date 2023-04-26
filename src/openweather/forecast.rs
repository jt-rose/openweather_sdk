use crate::responses::ForecastResponse;
use crate::units::Units;
use crate::languages::Language;
use std::fmt;
use serde::{ Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq, Hash, Default, Clone)]
pub struct Forecast {
    api_key: String,
    units: Units,
    language: Language,
}

impl fmt::Display for Forecast {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Forecast: (api_key: {}, units: {}, language: {}, methods: [new, get_forecast])",
            self.api_key,
            self.units,
            self.language
        )
    }
}

impl Forecast {
    pub fn new(api_key: String, units: Units, language: Language) -> Self {
        Self {
            api_key,
            units,
            language
        }
    }

    fn format_query(&self, lat: f64, lon: f64, forecast_type: &str, count: u8) -> String {
        format!(
            "https://api.openweathermap.org/data/2.5/forecast{}?lat={}&lon={}&cnt={}&units={}&lang={}&appid={}",
            forecast_type,
            lat,
            lon,
            count,
            self.units,
            self.language,
            self.api_key)
    }

    pub async fn call(&self, lat: f64, lon: f64, count: u8) -> Result<ForecastResponse, Box<dyn std::error::Error>> {
        let url = self.format_query(lat, lon, "", count);
        let resp = reqwest::get(url)
            .await?
            .json::<ForecastResponse>()
            .await?;

        Ok(resp)
    }

    // TODO: confirm response type for hourly, daily, and climate
    // pub async fn get_hourly_forecast(&self, lat: f64,lon: f64, count: u8) -> Result<ForecastResponse, Box<dyn std::error::Error>> {
    //     let url = self.format_query(lat, lon, "/hourly", count);
    //     let resp = reqwest::get(url)
    //         .await?
    //         .json::<ForecastResponse>()
    //         .await?;
    //
    //     Ok(resp)
    // }
    //
    // pub async fn get_daily_forecast(&self, lat: f64,lon: f64, count: u8) -> Result<ForecastResponse, Box<dyn std::error::Error>> {
    //     let url = self.format_query(lat, lon, "/daily", count);
    //     let resp = reqwest::get(url)
    //         .await?
    //         .json::<ForecastResponse>()
    //         .await?;
    //
    //     Ok(resp)
    // }
    //
    // pub async fn get_climate_forecast(&self, lat: f64,lon: f64, count: u8) -> Result<ForecastResponse, Box<dyn std::error::Error>> {
    //     let url = self.format_query(lat, lon, "/climate", count);
    //     let resp = reqwest::get(url)
    //         .await?
    //         .json::<ForecastResponse>()
    //         .await?;
    //
    //     Ok(resp)
    // }
}
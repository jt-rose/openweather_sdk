use reqwest;
use super::forecast_response::ForecastResponse;
use crate::units::Units;
use crate::languages::Language;

pub struct Forecast {
    api_key: String,
    units: Units,
    language: Language,
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

    pub async fn get_forecast(&self, lat: f64,lon: f64, count: u8) -> Result<ForecastResponse, Box<dyn std::error::Error>> {
        // self.query(lat, lon, "").await?
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
use crate::responses::{GeocodingResponse, response_handler, ZipCodeResponse};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq, Hash, Default, Clone)]
pub struct Geocoding {
    api_key: String,
}

impl std::fmt::Display for Geocoding {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Geocoding: (api_key: {}, methods: [new, get_geocoding, get_geocoding_by_zip_code, get_location_data])",
            self.api_key
        )
    }
}

impl Geocoding {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key
        }
    }

    pub async fn get_geocoding(&self, city: &str, state_code: Option<&str>, country_code: Option<&str>, limit: u8)
                               -> Result<Vec<GeocodingResponse>, Box<dyn std::error::Error>> {
        let mut q = city.to_string();
        if let Some(sc) = state_code {
            q.push(',');
            q.push_str(sc);
        }

        if let Some(cc) = country_code {
            q.push(',');
            q.push_str(cc);
        }

        let url = format!(
            "https://api.openweathermap.org/geo/1.0/direct?q={}&limit={}&appid={}",
            q,
            limit,
            self.api_key
        );
        let resp = reqwest::get(url)
            .await?;
        response_handler::<Vec<GeocodingResponse>>(resp).await
    }

    pub async fn get_geocoding_by_zip_code(&self, zip_code: &str, country_code: Option<&str>) -> Result<ZipCodeResponse, Box<dyn std::error::Error>> {
        let mut country = "".to_string();
        if let Some(cc) = country_code {
            country.push(',');
            country.push_str(cc);
        }

        let url = format!(
            "https://api.openweathermap.org/geo/1.0/zip?zip={}{}&appid={}",
            zip_code,
            country,
            self.api_key
        );

        let resp = reqwest::get(url)
            .await?;
        response_handler::<ZipCodeResponse>(resp).await
    }

    pub async fn get_location_data(&self, lat: f64, lon: f64, limit: u8) -> Result<Vec<GeocodingResponse>, Box<dyn std::error::Error>> {
        let url = format!(
            "https://api.openweathermap.org/geo/1.0/reverse?lat={}&lon={}&limit={}&appid={}",
            lat,
            lon,
            limit,
            self.api_key
        );

        let resp = reqwest::get(url)
            .await?;
        response_handler::<Vec<GeocodingResponse>>(resp).await
    }
}
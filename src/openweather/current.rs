use crate::responses::{CurrentResponse, response_handler};
use crate::units::Units;
use crate::languages::Language;
use std::fmt;
use serde::{ Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq, Hash, Default, Clone)]
pub struct Current {
    api_key: String,
    units: Units,
    language: Language,
}

impl fmt::Display for Current {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Current: (api_key: {}, units: {}, language: {}, methods: [new, get_current])",
            self.api_key,
            self.units,
            self.language
        )
    }
}

impl Current {
    pub fn new(api_key: String, units: Units, language: Language) -> Self {
        Self {
            api_key,
            units,
            language
        }
    }

    fn format_query(&self, lat: f64, lon: f64) -> String {
        format!(
            "https://api.openweathermap.org/data/2.5/weather?lat={}&lon={}&units={}&lang={}&appid={}",
            lat,
            lon,
            self.units,
            self.language,
            self.api_key)
    }

    pub async fn call(&self, lat: f64, lon: f64) -> Result<CurrentResponse, Box<dyn std::error::Error>> {
        let url = self.format_query(lat, lon);
        dbg!(&url);
        let resp = reqwest::get(url)
            .await?;
        response_handler::<CurrentResponse>(resp).await
    }
}
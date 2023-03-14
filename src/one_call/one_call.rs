use std::fmt;
use crate::languages::Language;
use crate::units::Units;
use crate::responses::one_call_response::OneCallResponse;
use crate::responses::historical_response::HistoricalResponse;

pub struct Fields {
    pub current: bool,
    pub minutely: bool,
    pub hourly: bool,
    pub daily: bool,
    pub alerts: bool
}

impl fmt::Display for Fields {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Fields: current: {}, minutely: {}, hourly: {}, daily: {}, alerts: {}",
            self.current,
            self.minutely,
            self.hourly,
            self.daily,
            self.alerts
        )
    }
}

pub struct OneCall {
    api_key: String,
    units: Units,
    language: Language,
    // fields are used to specify which should be included,
    // defaulting to true for all
    pub fields: Fields
}

impl fmt::Display for OneCall {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "OneCall: (api_key: {}, units: {}, language: {}, fields: {}, methods: [new, get_onecall, get_historical])",
            self.api_key,
            self.units,
            self.language,
            self.fields
        )
    }
}

impl OneCall {
    pub fn new(api_key: String, units: Units, language: Language) -> Self {
        Self{
            api_key,
            units,
            language,
            fields: Fields {
                current: true,
                minutely: true,
                hourly: true,
                daily: true,
                alerts: true
            }
        }
    }

    fn format_url_query(&self, lat: f64, lon: f64) -> String {
        format!(
            "https://api.openweathermap.org/data/3.0/onecall?lat={}&lon={}&units={}&lang={}&appid={}{}",
            lat,
            lon,
            self.units,
            self.language,
            self.api_key,
            self.format_excluded_fields()
        )
    }

    fn format_historical_query(&self, lat: f64, lon: f64, datetime: i64) -> String {
        format!(
            "https://api.openweathermap.org/data/3.0/onecall/timemachine?dt={}&lat={}&lon={}&units={}&lang={}&appid={}",
            datetime,
            lat,
            lon,
            self.units,
            self.language,
            self.api_key
        )
    }

    fn format_excluded_fields(&self) -> String {
        let mut excluded_fields = Vec::new();

        if self.fields.current == false {
            excluded_fields.push("current")
        }
        if self.fields.minutely == false {
            excluded_fields.push("minutely")
        }
        if self.fields.hourly == false {
            excluded_fields.push("hourly")
        }
        if self.fields.daily == false {
            excluded_fields.push("daily")
        }
        if self.fields.alerts == false {
            excluded_fields.push("alerts")
        }

        if excluded_fields.is_empty() {
            "".to_string()
        } else {
            let mut excluded = "&exclude=".to_string();
            excluded.push_str(&excluded_fields.join(","));
            excluded
        }
    }

    pub async fn call(&self, lat: f64, lon: f64) -> Result<OneCallResponse, Box<dyn std::error::Error>> {
        let resp = reqwest::get(self.format_url_query(lat, lon))
            .await?
            .json::<OneCallResponse>()
            .await?;

        Ok(resp)
    }

    pub async fn call_historical_data(&self, lat: f64, lon: f64, datetime: i64) -> Result<HistoricalResponse, Box<dyn std::error::Error>> {
        let resp = reqwest::get(self.format_historical_query(lat, lon, datetime))
            .await?
            .json::<HistoricalResponse>()
            .await?;

        Ok(resp)
    }
}
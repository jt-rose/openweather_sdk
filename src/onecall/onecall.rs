use crate::languages::Language;
use crate::units::Units;
use crate::responses::onecall_response::OneCallResponse;
use crate::responses::historical_response::HistoricalResponse;

pub struct Fields {
    pub current: bool,
    pub minutely: bool,
    pub hourly: bool,
    pub daily: bool,
    pub alerts: bool
}

pub struct OneCall {
    api_key: String,
    lat: f64,
    long: f64,
    units: Units,
    language: Language,
    // fields are used to specify which should be included,
    // defaulting to true for all
    pub fields: Fields
}

impl OneCall {
    pub fn new(api_key: &str, lat: f64, long: f64) -> Self {
        Self{
            api_key: api_key.to_string(),
            lat,
            long,
            units: Units::Standard,
            language: Language::English,
            fields: Fields {
                current: true,
                minutely: true,
                hourly: true,
                daily: true,
                alerts: true
            }
        }
    }

    pub fn set_lat_and_long(&mut self, lat: f64, long: f64) {
        self.lat = lat;
        self.long = long;
    }

    pub fn set_units(&mut self, units: Units) {
        self.units = units;
    }

    pub fn set_language(&mut self, language: Language) {
        self.language = language;
    }

    fn format_url_query(&self) -> String {
        format!(
            "https://api.openweathermap.org/data/3.0/onecall?lat={}&lon={}&units={}&lang={}&appid={}{}",
            self.lat,
            self.long,
            self.units,
            self.language,
            self.api_key,
            self.format_excluded_fields()
        )
    }

    fn format_historical_query(&self, datetime: i64) -> String {
        format!(
            "https://api.openweathermap.org/data/3.0/onecall/timemachine?dt={}&lat={}&lon={}&units={}&lang={}&appid={}",
            datetime,
            self.lat,
            self.long,
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

    pub async fn call(&self) -> Result<OneCallResponse, Box<dyn std::error::Error>> {
        println!("{}", self.format_excluded_fields());
        println!("{}", self.format_url_query());
        let resp = reqwest::get(self.format_url_query())
            .await?
            .json::<OneCallResponse>()
            .await?;

        Ok(resp)
    }

    pub async fn call_historical_data(&self, datetime: i64) -> Result<HistoricalResponse, Box<dyn std::error::Error>> {
        println!("{}", self.format_historical_query(datetime));
        let resp = reqwest::get(self.format_historical_query(datetime))
            .await?
            .json::<HistoricalResponse>()
            .await?;

        Ok(resp)
    }
}
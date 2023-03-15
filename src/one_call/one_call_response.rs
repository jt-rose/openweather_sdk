use serde::{ Serialize, Deserialize };
use crate::response_elements::weather::Weather;
use crate::response_elements::rain::Rain;
use crate::response_elements::snow::Snow;
use crate::response_elements::temp::Temp;
use std::fmt;
use crate::utils::display_option;

#[derive(Debug, Serialize, Deserialize, PartialOrd, PartialEq, Default, Clone)]
pub struct Current {
    pub dt: i64,
    pub sunrise: i64,
    pub sunset: i64,
    pub temp: f64,
    pub feels_like: f64,
    pub pressure: u64,
    pub humidity: u64,
    pub dew_point: f64,
    pub uvi: f64,
    pub clouds: u64,
    pub visibility: i64,
    pub wind_speed: f64,
    pub wind_deg: u64,
    pub wind_gust: Option<f64>,
    pub rain: Option<Rain>,
    pub snow: Option<Snow>,
    pub weather: Vec<Weather>
}

impl fmt::Display for Current {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut weather_list = String::new();
        weather_list.push_str("[");
        for weather in &self.weather {
            weather_list.push_str(&format!("{}, ", weather));
        }
        weather_list.push_str("]");

        write!(
            f,
            "Current: (dt: {}, sunrise: {}, sunset: {}, temp: {}, feels_like: {}, pressure: {}, humidity: {}, dew_point: {}, uvi: {}, clouds: {}, visibility: {}, wind_speed: {}, wind_deg: {}, wind_gust: {}, rain: {}, snow: {}, weather: {})",
            self.dt,
            self.sunrise,
            self.sunset,
            self.temp,
            self.feels_like,
            self.pressure,
            self.humidity,
            self.dew_point,
            self.uvi,
            self.clouds,
            self.visibility,
            self.wind_speed,
            self.wind_deg,
            display_option(&self.wind_gust),
            display_option(&self.rain),
            display_option(&self.snow),
            weather_list
        )
    }
}

#[derive(Debug, Serialize, Deserialize, PartialOrd, PartialEq, Default, Clone)]
pub struct FeelsLike {
    pub day: f64,
    pub night: f64,
    pub eve: f64,
    pub morn: f64
}

impl fmt::Display for FeelsLike {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "FeelsLike: (day: {}, night: {}, eve: {}, morn: {})",
            self.day,
            self.night,
            self.eve,
            self.morn
        )
    }
}

#[derive(Debug, Serialize, Deserialize, PartialOrd, PartialEq, Default, Clone)]
pub struct Daily {
    #[serde(alias = "dt")]
    pub datetime: i64,
    pub sunrise: i64,
    pub sunset: i64,
    pub moonrise: i64,
    pub moonset: i64,
    pub moon_phase: f64,
    pub temp: Temp,
    pub feels_like: FeelsLike,
    pub pressure: i64,
    pub humidity: u64,
    pub dew_point: f64,
    pub wind_speed: f64,
    pub wind_deg: i64,
    pub wind_gust: Option<f64>,
    pub weather: Vec<Weather>,
    pub clouds: i64,
    pub pop: f64,
    pub rain: Option<f64>,
    pub snow: Option<f64>,
    pub uvi: f64
}

impl fmt::Display for Daily {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut weather_list = String::new();
        weather_list.push_str("[");
        for weather in &self.weather {
            weather_list.push_str(&format!("{}, ", weather));
        }
        weather_list.push_str("]");

        write!(
            f,
            "Daily: (datetime: {}, sunrise: {}, sunset: {}, moonrise: {}, moonset: {}, moon_phase: {}, temp: {}, feels_like: {}, pressure: {}, humidity: {}, dew_point: {}, wind_speed: {}, wind_deg: {}, wind_gust: {}, weather: {}, clouds: {}, pop: {}, rain: {}, snow: {}, uvi: {})",
            self.datetime,
            self.sunrise,
            self.sunset,
            self.moonrise,
            self.moonset,
            self.moon_phase,
            self.temp,
            self.feels_like,
            self.pressure,
            self.humidity,
            self.dew_point,
            self.wind_speed,
            self.wind_deg,
            display_option(&self.wind_gust),
            weather_list,
            self.clouds,
            self.pop,
            display_option(&self.rain),
            display_option(&self.snow),
            self.uvi
        )
    }
}

#[derive(Debug, Serialize, Deserialize, PartialOrd, PartialEq, Default, Clone)]
pub struct Hourly {
    #[serde(alias = "dt")]
    pub datetime: i64,
    pub temp: f64,
    pub feels_like: f64,
    pub pressure: u64,
    pub humidity: u64,
    pub dew_point: f64,
    pub uvi: f64,
    pub clouds: u64,
    pub visibility: i64,
    pub wind_speed: f64,
    pub wind_deg: u64,
    pub wind_gust: Option<f64>,
    pub rain: Option<Rain>,
    pub snow: Option<Snow>,
    pub weather: Vec<Weather>,
    pub pop: f64
    }

impl fmt::Display for Hourly {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut weather_list = String::new();
        weather_list.push_str("[");
        for weather in &self.weather {
            weather_list.push_str(&format!("{}, ", weather));
        }
        weather_list.push_str("]");

        write!(
            f,
            "Hourly: (datetime: {}, temp: {}, feels_like: {}, pressure: {}, humidity: {}, dew_point: {}, uvi: {}, clouds: {}, visibility: {}, wind_speed: {}, wind_deg: {}, wind_gust: {}, rain: {}, snow: {}, weather: {}, pop: {})",
            self.datetime,
            self.temp,
            self.feels_like,
            self.pressure,
            self.humidity,
            self.dew_point,
            self.uvi,
            self.clouds,
            self.visibility,
            self.wind_speed,
            self.wind_deg,
            display_option(&self.wind_gust),
            display_option(&self.rain),
            display_option(&self.snow),
            weather_list,
            self.pop
        )
    }
}

#[derive(Debug, Serialize, Deserialize, PartialOrd, PartialEq, Default, Clone)]
pub struct Minutely {
    #[serde(alias = "dt")]
    pub datetime: i64,
    pub precipitation: u64
}

impl fmt::Display for Minutely {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Minutely: (datetime: {}, precipitation: {})",
            self.datetime,
            self.precipitation
        )
    }
}

#[derive(Debug, Serialize, Deserialize, PartialOrd, PartialEq, Default, Clone)]
pub struct Alert {
    pub sender_name: String,
    pub event: String,
    pub start: i64,
    pub end: i64,
    pub description: String,
    pub tags: Vec<String>
}

impl fmt::Display for Alert {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut tags_list = String::new();
        tags_list.push_str("[");
        for tag in &self.tags {
            tags_list.push_str(&format!("{}, ", tag));
        }
        tags_list.push_str("]");

        write!(
            f,
            "Alert: (sender_name: {}, event: {}, start: {}, end: {}, description: {}, tags: {})",
            self.sender_name,
            self.event,
            self.start,
            self.end,
            self.description,
            tags_list
        )
    }
}

#[derive(Debug, Serialize, Deserialize, PartialOrd, PartialEq, Default, Clone)]
pub struct OneCallResponse {
    pub lat: f64,
    pub lon: f64,
    pub timezone: String,
    pub timezone_offset: isize,
    pub current: Option<Current>,
    pub minutely: Option<Vec<Minutely>>,
    pub hourly: Option<Vec<Hourly>>,
    pub daily: Option<Vec<Daily>>,
    pub alerts: Option<Vec<Alert>>
}

impl fmt::Display for OneCallResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut minutely_list = String::new();
        minutely_list.push_str("[");
        if let Some(minutely) = &self.minutely {
            for minute in minutely {
                minutely_list.push_str(&format!("{}, ", minute));
            }
        }
        minutely_list.push_str("]");

        let mut hourly_list = String::new();
        hourly_list.push_str("[");
        if let Some(hourly) = &self.hourly {
            for hour in hourly {
                hourly_list.push_str(&format!("{}, ", hour));
            }
        }
        hourly_list.push_str("]");

        let mut daily_list = String::new();
        daily_list.push_str("[");
        if let Some(daily) = &self.daily {
            for day in daily {
                daily_list.push_str(&format!("{}, ", day));
            }
        }
        daily_list.push_str("]");

        let mut alert_list = String::new();
        alert_list.push_str("[");
        if let Some(alerts) = &self.alerts {
            for alert in alerts {
                alert_list.push_str(&format!("{}, ", alert));
            }
        }
        alert_list.push_str("]");

        write!(
            f,
            "OneCallResponse: (lat: {}, lon: {}, timezone: {}, timezone_offset: {}, current: {}, minutely: {}, hourly: {}, daily: {}, alerts: {})",
            self.lat,
            self.lon,
            self.timezone,
            self.timezone_offset,
            display_option(&self.current),
            minutely_list,
            hourly_list,
            daily_list,
            alert_list
        )
    }
}
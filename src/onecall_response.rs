use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Weather {
    id: u64,
    main: String,
    description: String,
    icon: String
}

#[derive(Debug, Deserialize)]
struct Rain {
    #[serde(alias = "1h")]
    volume_over_last_hour: f64
}

#[derive(Debug, Deserialize)]
struct Snow {
    #[serde(alias = "1h")]
    volume_over_last_hour: f64
}

#[derive(Debug, Deserialize)]
struct Current {
    dt: i64,
    sunrise: i64,
    sunset: i64,
    temp: f64,
    feels_like: f64,
    pressure: u64,
    humidity: u64,
    dew_point: f64,
    uvi: f64,
    clouds: u64,
    visibility: i64,
    wind_speed: f64,
    wind_deg: u64,
    wind_gust: Option<f64>,
    rain: Option<Rain>,
    snow: Option<Snow>,
    weather: Vec<Weather>
}

#[derive(Debug, Deserialize)]
struct Temp {
    day: f64,
    min: f64,
    max: f64,
    night: f64,
    eve: f64,
    morn: f64
}

#[derive(Debug, Deserialize)]
struct FeelsLike {
    day: f64,
    night: f64,
    eve: f64,
    morn: f64
}

#[derive(Debug, Deserialize)]
struct Daily {
    #[serde(alias = "dt")]
    datetime: i64,
    sunrise: i64,
    sunset: i64,
    moonrise: i64,
    moonset: i64,
    moon_phase: f64,
    temp: Temp,
    feels_like: FeelsLike,
    pressure: i64,
    humidity: u64,
    dew_point: f64,
    wind_speed: f64,
    wind_deg: i64,
    wind_gust: Option<f64>,
    weather: Vec<Weather>,
    clouds: i64,
    pop: f64,
    rain: Option<f64>,
    snow: Option<f64>,
    uvi: f64
}

#[derive(Debug, Deserialize)]
struct Hourly {
    #[serde(alias = "dt")]
    datetime: i64,
    temp: f64,
    feels_like: f64,
    pressure: u64,
    humidity: u64,
    dew_point: f64,
    uvi: f64,
    clouds: u64,
    visibility: i64,
    wind_speed: f64,
    wind_deg: u64,
    wind_gust: Option<f64>,
    rain: Option<Rain>,
    snow: Option<Snow>,
    weather: Vec<Weather>,
    pop: f64
    }

#[derive(Debug, Deserialize)]
struct Minutely {
    #[serde(alias = "dt")]
    datetime: i64,
    precipitation: u64
}

#[derive(Debug, Deserialize)]
struct Alert {
    sender_name: String,
    event: String,
    start: i64,
    end: i64,
    description: String,
    tags: Vec<String>
}

#[derive(Debug, Deserialize)]
pub struct OneCallResponse {
    lat: f64,
    lon: f64,
    timezone: String,
    timezone_offset: isize,
    current: Option<Current>,
    minutely: Option<Vec<Minutely>>,
    hourly: Option<Vec<Hourly>>,
    daily: Option<Vec<Daily>>,
    alerts: Option<Vec<Alert>>
}
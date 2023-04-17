//! # OpenWeather SDK
//!
//! TODO: Add description
//!
//! ## Example
//!
//! ```rust
//! use openweather_sdk::prelude::{ OpenWeather, Units, Language };
//!
//! let openweather = OpenWeather::new(
//!     "MY_PRIVATE_API_KEY".to_string(),
//!     Units::Imperial,
//!     Language::English
//! );
//! ```

//! ### One Call
//!
//! ```rust
//! let lat = 38.795021;
//! let lon = -77.273300;
//!
//! let res = openweather.one_call.call(lat, lon).await;
//! let res2 = openweather.one_call.historical(lat, lon, 1606223802).await;
//!
//! // Select fields
//! openweather.one_call.fields.minutely = false;
//! openweather.one_call.fields.hourly = false;
//! let res4 = openweather.one_call.call(lat, lon).await;
//! ```
//!
//! ### Forecast
//!
//! ```rust
//! let lat = 38.795021;
//! let lon = -77.273300;
//!
//! let res = openweather.forecast.call(lat, lon).await;
//! ```
//!
//! ### Maps
//!
//! ```rust
//! let lat = 38.795021;
//! let lon = -77.273300;
//! let zoom = 1;
//! let x_tiles = 1;
//! let y_tiles = 1;
//!
//! let res = openweather.maps.get_cloud_map(zoom, x_tiles, y_tiles).await;
//! let res2 = openweather.maps.get_precipitation_map(zoom, x_tiles, y_tiles).await;
//! let res3 = openweather.maps.get_temperature_map(zoom, x_tiles, y_tiles).await;
//! let res4 = openweather.maps.get_wind_speed_map(zoom, x_tiles, y_tiles).await;
//! let res5 = openweather.maps.get_pressure_map(zoom, x_tiles, y_tiles).await;
//!
//! ```
//!
//! ### Air Pollution
//!
//! ```rust
//! let lat = 38.795021;
//! let lon = -77.273300;
//!
//! let res = openweather.air_pollution.get_current_air_pollution(lat, lon).await;
//! let res2 = openweather.air_pollution.get_forecast_air_pollution(lat, lon).await;
//!
//! let start = 1606223802;
//! let end = 1606482999;
//! let res2 = openweather.air_pollution.get_historical_air_pollution(lat, lon, start, end).await;
//!
//! ```
//!
//! ### Geocoding
//!
//! ```rust
//! let lat = 38.795021;
//! let lon = -77.273300;
//!
//! let city = "Washington";
//! let state = "DC";
//! let country = "US";
//! let limit = 5;
//!
//! let res = openweather.geocoding.get_geocoding(city, Some(state), Some(country), limit).await;
//! let res2 = openweather.geocoding.get_geocoding_by_zip_code("20001", None).await;
//! let res3 = openweather.geocoding.get_location_data(lat, lon, limit).await;
//!
//! ```


pub mod languages;
pub mod units;
pub mod one_call;
pub mod forecast;
// pub mod settings;
pub mod openweather;
pub mod maps;
pub mod air_pollution;
pub mod geocoding;
pub mod response_elements;
pub mod utils;

pub mod responses {
    pub use crate::response_elements;
    pub use crate::one_call::one_call_response::OneCallResponse;
    pub use crate::one_call::historical_response::HistoricalResponse;
    pub use crate::forecast::forecast_response::ForecastResponse;
    pub use crate::maps::map_layer::MapLayer;
    pub use crate::air_pollution::air_pollution_response::AirPollutionResponse;
    pub use crate::geocoding::geocoding_response::GeocodingResponse;
}

pub mod prelude {
    pub use crate::openweather::OpenWeather;
    pub use crate::languages::Language;
    pub use crate::units::Units;
    pub use crate::responses;
}

#[cfg(test)]
mod tests {
    use crate::languages::Language;
    use crate::openweather::OpenWeather;
    use crate::units::Units;

    extern crate dotenv;

    use dotenv::dotenv;
    use std::env;

    struct Setup {
        lat: f64,
        lon: f64,
        start: u64,
        end: u64
    }

    impl Setup {
        pub fn new() -> Self {
            Self {
                lat: 38.795021,
                lon: -77.273300,
                start: 1606223802,
                end: 1606482999,
            }
        }
    }

    fn create_client() -> OpenWeather {
        dotenv().ok();

        let mut api_key = "".to_string();
        for v in env::vars() {
            if v.0 == "API_KEY" {
                api_key = v.1;
            }
        }

        OpenWeather::new(api_key, Units::Imperial, Language::English)
    }

    // one_call
    #[tokio::test]
    async fn get_one_call() {
        let client = create_client();
        let setup = Setup::new();
        let result = client.one_call.call(setup.lat, setup.lon).await;

        assert_eq!(result.is_ok(), true);
    }

    #[tokio::test]
    async fn get_one_call_with_select_fields() {
        let mut client = create_client();
        client.one_call.fields.minutely = false;
        client.one_call.fields.hourly = false;
        let setup = Setup::new();
        let result = client.one_call.call(setup.lat, setup.lon).await;

        assert_eq!(result.is_ok(), true);
        // TODO: add check for removed fields as null
    }

    #[tokio::test]
    async fn get_historical_one_call() {
        let client = create_client();
        let setup = Setup::new();
        let result = client.one_call.call_historical_data(setup.lat, setup.lon, 643803200).await;

        assert_eq!(result.is_ok(), true);
    }

    // forecast
    #[tokio::test]
    async fn get_forecast() {
        let client = create_client();
        let setup = Setup::new();
        let result = client.forecast.call(setup.lat, setup.lon, 5).await;

        assert_eq!(result.is_ok(), true);
    }

    // #[tokio::test]
    // async fn get_hourly_forecast() {
    //     let client = create_client();
    //     let setup = Setup::new();
    //     let result = client.forecast.get_hourly_forecast(setup.lat, setup.lon,5).await;
    //
    //     assert_eq!(result.is_ok(), true);
    // }
    //
    // #[tokio::test]
    // async fn get_daily_forecast() {
    //     let client = create_client();
    //     let setup = Setup::new();
    //     let result = client.forecast.get_daily_forecast(setup.lat, setup.lon,5).await;
    //
    //     assert_eq!(result.is_ok(), true);
    // }
    //
    // #[tokio::test]
    // async fn get_climate_forecast() {
    //     let client = create_client();
    //     let setup = Setup::new();
    //     let result = client.forecast.get_climate_forecast(setup.lat, setup.lon,5).await;
    //
    //     assert_eq!(result.is_ok(), true);
    // }

    // maps
    #[tokio::test]
    async fn get_temperature_map() {
        let client = create_client();
        let result = client.maps.get_temperature_map(1, 1, 1).await;

        assert_eq!(result.is_ok(), true);
    }

    #[tokio::test]
    async fn get_wind_speed_map() {
        let client = create_client();
        let result = client.maps.get_wind_speed_map(1, 1, 1).await;

        assert_eq!(result.is_ok(), true);
    }

    #[tokio::test]
    async fn get_pressure_map() {
        let client = create_client();
        let result = client.maps.get_pressure_map(1, 1, 1).await;

        assert_eq!(result.is_ok(), true);
    }

    #[tokio::test]
    async fn get_precipitation_map() {
        let client = create_client();
        let result = client.maps.get_precipitation_map(1, 1, 1).await;

        assert_eq!(result.is_ok(), true);
    }

    #[tokio::test]
    async fn get_cloud_map() {
        let client = create_client();
        let result = client.maps.get_cloud_map(1, 1, 1).await;

        assert_eq!(result.is_ok(), true);
    }

    // air pollution
    #[tokio::test]
    async fn get_air_pollution() {
        let client = create_client();
        let setup = Setup::new();
        let result = client.air_pollution.get_current_air_pollution(setup.lat, setup.lon).await;

        assert_eq!(result.is_ok(), true);
    }

    #[tokio::test]
    async fn get_historical_air_pollution() {
        let client = create_client();
        let setup = Setup::new();
        let result = client.air_pollution.get_historical_air_pollution(setup.lat, setup.lon, setup.start, setup.end).await;

        assert_eq!(result.is_ok(), true);
    }

    #[tokio::test]
    async fn get_forecast_air_pollution() {
        let client = create_client();
        let setup = Setup::new();
        let result = client.air_pollution.get_forecast_air_pollution(setup.lat, setup.lon).await;

        assert_eq!(result.is_ok(), true);
    }

    // geocoding
    #[tokio::test]
    async fn get_geocoding() {
        let client = create_client();
        let result = client.geocoding.get_geocoding("Burke", Some("Virginia"), Some("US"), 5).await;

        assert_eq!(result.is_ok(), true);
    }

    #[tokio::test]
    async fn get_geocoding_by_zip_code() {
        let client = create_client();
        let result = client.geocoding.get_geocoding_by_zip_code("22015", None).await;

        assert_eq!(result.is_ok(), true);
    }

    #[tokio::test]
    async fn get_reverse_geocoding() {
        let client = create_client();
        let setup = Setup::new();
        let result = client.geocoding.get_location_data(setup.lat, setup.lon, 5).await;

        assert_eq!(result.is_ok(), true);
    }
}
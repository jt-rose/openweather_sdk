//! # OpenWeather SDK
//!
//! This library is a small rust wrapper for making requests to the [OpenWeather API]. This library includes:
//!
//! - query constructor with full coverage of free-tier API calls
//! - type-safe enums for accurate settings
//! - type-safe [serde] deserialization of responses
//! - async requests made with the [Reqwest] library
//!
//! ### Query Types Supported
//! - [x] [OneCall]
//! - [x] [Time Machine]
//! - [x] [Daily Aggregation]
//! - [x] [Weather Overview]
//! - [x] [Forecast]
//! - [x] [Current]
//! - [x] [Maps]
//! - [x] [Air Pollution]
//! - [x] [Geocoding]
//!
//! ## Examples
//!
//! For detailed examples of how to use the [OpenWeather API], please reference the official documentation.
//!
//! ```rust
//! use openweather_sdk::{OpenWeather, Units, Language};
//!
//! let openweather = OpenWeather::new(
//!     "MY_PRIVATE_API_KEY".to_string(),
//!     Units::Imperial,
//!     Language::English
//! );
//!
//! let lat = 38.795021;
//! let lon = -77.273300;
//! let count = 10;
//!
//! let forecast_response = openweather.forecast.call(lat, lon, count).await;
//! ```

//! ### One Call
//!
//! ```rust
//! let lat = 38.795021;
//! let lon = -77.273300;
//! let historical_date = 1606223802;
//! let date = "2024-05-31" // YYYY-MM-DD format
//!
//! // get one call data for current weather
//! let res = openweather.one_call.call(lat, lon).await;
//!
//! // get one call data for historical weather
//! let res2 = openweather.one_call.call_historical_data(lat, lon, historical_date).await;
//!
//! // get one call data for daily aggregation
// let res3 = openweather.one_call.call_daily_aggregation(lat, lon, date, None).await;
//!
//! // get one call data for weather overview
// let res4 = openweather.one_call.call_weather_overview(lat, lon, date).await;
//!
//! // customize response fields
//! openweather.one_call.fields.minutely = false;
//! openweather.one_call.fields.hourly = false;
//! let res5 = openweather.one_call.call(lat, lon).await;
//! ```
//!
//! ### Forecast
//!
//! ```rust
//! let lat = 38.795021;
//! let lon = -77.273300;
//! let count = 10;
//!
//! // get forecast data with specified number of timestamps
//! let res = openweather.forecast.call(lat, lon, count).await;
//! ```
//!
//! ### Current
//!
//! ```rust
//! let lat = 38.795021;
//! let lon = -77.273300;
//!
//! // get forecast data with specified number of timestamps
//! let res = openweather.current.call(lat, lon).await;
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
//! // get various types of map data
//! let cloud_map = openweather.maps.get_cloud_map(zoom, x_tiles, y_tiles).await;
//! let precip_map = openweather.maps.get_precipitation_map(zoom, x_tiles, y_tiles).await;
//! let temp_map = openweather.maps.get_temperature_map(zoom, x_tiles, y_tiles).await;
//! let wind_spd_map = openweather.maps.get_wind_speed_map(zoom, x_tiles, y_tiles).await;
//! let pressure_map = openweather.maps.get_pressure_map(zoom, x_tiles, y_tiles).await;
//!
//! ```
//!
//! ### Air Pollution
//!
//! ```rust
//! let lat = 38.795021;
//! let lon = -77.273300;
//!
//! // get current and forecast air pollution data
//! let res = openweather.air_pollution.get_current_air_pollution(lat, lon).await;
//! let res2 = openweather.air_pollution.get_forecast_air_pollution(lat, lon).await;
//!
//! // get historical air pollution data with start and end timestamps
//! let start = 1606223802;
//! let end = 1606482999;
//! let res3 = openweather.air_pollution.get_historical_air_pollution(lat, lon, start, end).await;
//! ```
//!
//! ### Geocoding
//!
//! ```rust
//! let lat = 38.795021;
//! let lon = -77.273300;
//!
//! let city = "New York";
//! let state = "NY";
//! let country = "US";
//! let limit = 5;
//!
//! // get geocoding data by city name, zip code, or coordinates
//! let res = openweather.geocoding.get_geocoding(city, Some(state), Some(country), limit).await;
//! let res2 = openweather.geocoding.get_geocoding_by_zip_code("20001", None).await;
//! let res3 = openweather.geocoding.get_location_data(lat, lon, limit).await;
//! ```
//!
//! [OpenWeather API]: https://openweathermap.org/api
//! [OneCall]: https://openweathermap.org/api/one-call-3
//! [Time Machine]: https://openweathermap.org/api/one-call-3#history
//! [Daily Aggregation]: https://openweathermap.org/api/one-call-3#history_daily_aggregation
//! [Weather Overview]: https://docs.openweather.co.uk/api/one-call-3#weather_overview
//! [Forecast]: https://openweathermap.org/forecast5
//! [Maps]: https://openweathermap.org/api/weathermaps
//! [Air Pollution]: https://openweathermap.org/api/air-pollution
//! [Geocoding]: https://openweathermap.org/api/geocoding-api
//! [serde]: https://serde.rs/
//! [Reqwest]: https://docs.rs/reqwest/0.11.3/reqwest/
//! [Jeff Rose]: www.github.com/jt-rose

mod openweather;
mod languages;
mod units;
mod utils;
pub mod responses;

pub use crate::languages::Language;
pub use crate::units::Units;
pub use crate::openweather::{ OpenWeather, OneCall, Fields, Forecast, Maps, AirPollution, Geocoding};

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
        pub fn new(lat: f64, lon: f64) -> Self {
            Self {
                lat: lat,
                lon: lon,
                start: 1606223802,
                end: 1606482999,
            }
        }
    }

    impl Default for Setup {
        fn default() -> Self {
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
        let setup = Setup::default();
        let result = client.one_call.call(setup.lat, setup.lon).await;

        assert_eq!(result.is_ok(), true);
    }

    #[tokio::test]
    async fn get_one_call_with_select_fields() {
        let mut client = create_client();
        client.one_call.fields.minutely = false;
        client.one_call.fields.hourly = false;
        let setup = Setup::default();
        let result = client.one_call.call(setup.lat, setup.lon).await;

        assert_eq!(result.is_ok(), true);

        let result = result.unwrap();

        assert_eq!(result.minutely, None);
        assert_eq!(result.hourly, None);
    }

    #[tokio::test]
    async fn get_one_call_with_missing_visibility() {
        let client = create_client();
        let setup = Setup::new(48.210033, 16.363449);
        let result = client.one_call.call(setup.lat, setup.lon).await;

        assert_eq!(result.is_ok(), true);
    }

    #[tokio::test]
    async fn get_one_call_with_missing_sunset_sunrise() {
        let client = create_client();
        let setup = Setup::new(69.702819, 170.274688);
        let result = client.one_call.call(setup.lat, setup.lon).await;

        assert_eq!(result.is_ok(), true);
    }

    #[tokio::test]
    async fn get_historical_one_call() {
        let client = create_client();
        let setup = Setup::default();
        let result = client.one_call.call_historical_data(setup.lat, setup.lon, 643803200).await;

        assert_eq!(result.is_ok(), true);
    }

    #[tokio::test]
    async fn get_daily_aggregation_one_call() {
        let client = create_client();
        let setup = Setup::default();
        let date = "2024-06-01";
        let result = client.one_call.call_daily_aggregation(setup.lat, setup.lon, date, None).await;

        assert_eq!(result.is_ok(), true);
    }

    #[tokio::test]
    async fn get_daily_aggregation_one_call_with_tz() {
        let client = create_client();
        let setup = Setup::default();
        let date = "2024-06-01";
        let tz_offset = Some("-05:00");
        let result = client.one_call.call_daily_aggregation(setup.lat, setup.lon, date, tz_offset).await;

        assert_eq!(result.is_ok(), true);
    }

    #[tokio::test]
    async fn get_weather_overview_one_call() {
        let client = create_client();
        let setup = Setup::default();
        let result = client.one_call.call_weather_overview(setup.lat, setup.lon, None).await;

        assert_eq!(result.is_ok(), true);
    }

    #[tokio::test]
    async fn get_weather_overview_one_call_with_date() {
        let client = create_client();
        let setup = Setup::default();
        // NOTE: weather_overview currently only supports today or tomorrow date requests
        // NOTE: in a YYYY-MM-DD format, so update this as needed for testing
        let tomorrow = Some("2024-06-01");
        let result = client.one_call.call_weather_overview(setup.lat, setup.lon, tomorrow).await;

        assert_eq!(result.is_ok(), true);
    }

    // forecast
    #[tokio::test]
    async fn get_forecast() {
        let client = create_client();
        let setup = Setup::default();
        let result = client.forecast.call(setup.lat, setup.lon, 5).await;

        assert_eq!(result.is_ok(), true);
    }

    // #[tokio::test]
    // async fn get_hourly_forecast() {
    //     let client = create_client();
    //     let setup = Setup::default();
    //     let result = client.forecast.get_hourly_forecast(setup.lat, setup.lon,5).await;
    //
    //     assert_eq!(result.is_ok(), true);
    // }
    //
    // #[tokio::test]
    // async fn get_daily_forecast() {
    //     let client = create_client();
    //     let setup = Setup::default();
    //     let result = client.forecast.get_daily_forecast(setup.lat, setup.lon,5).await;
    //
    //     assert_eq!(result.is_ok(), true);
    // }
    //
    // #[tokio::test]
    // async fn get_climate_forecast() {
    //     let client = create_client();
    //     let setup = Setup::default();
    //     let result = client.forecast.get_climate_forecast(setup.lat, setup.lon,5).await;
    //
    //     assert_eq!(result.is_ok(), true);
    // }

     // current
     #[tokio::test]
     async fn get_current() {
         let client = create_client();
         let setup = Setup::default();
         let result = client.current.call(setup.lat, setup.lon).await;
 
         assert_eq!(result.is_ok(), true);
     }


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
        let setup = Setup::default();
        let result = client.air_pollution.get_current_air_pollution(setup.lat, setup.lon).await;

        assert_eq!(result.is_ok(), true);
    }

    #[tokio::test]
    async fn get_historical_air_pollution() {
        let client = create_client();
        let setup = Setup::default();
        let result = client.air_pollution.get_historical_air_pollution(setup.lat, setup.lon, setup.start, setup.end).await;

        assert_eq!(result.is_ok(), true);
    }

    #[tokio::test]
    async fn get_forecast_air_pollution() {
        let client = create_client();
        let setup = Setup::default();
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
        let setup = Setup::default();
        let result = client.geocoding.get_location_data(setup.lat, setup.lon, 5).await;

        assert_eq!(result.is_ok(), true);
    }
}

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
        let result = client.forecast.get_forecast(setup.lat, setup.lon,5).await;

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
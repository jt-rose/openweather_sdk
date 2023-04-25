# OpenWeather_SDK

This library is a small rust wrapper for making requests to the [OpenWeather API]. This library includes:

- query constructor with full coverage of free-tier API calls
- type-safe enums for accurate settings
- type-safe [serde] deserialization of responses
- async requests made with the [Reqwest] library

### Query Types Supported
- [x] [OneCall]
- [x] [TimeMachine]
- [x] [Forecast]
- [x] [Maps]
- [x] [Air Pollution]
- [x] [Geocoding]

### Getting Started

#### Initialize the library
```rust
use openweather_sdk::prelude::{ OpenWeather, Units, Language };

let openweather = OpenWeather::new(
    "MY_PRIVATE_API_KEY".to_string(),
    Units::Imperial,
    Language::English
);
```

#### OneCall Query
```rust
let lat = 38.795021;
let lon = -77.273300;

let res = openweather.one_call.call(lat, lon).await;
let res2 = openweather.one_call.historical(lat, lon, 1606223802).await;

// Select fields
openweather.one_call.fields.minutely = false;
openweather.one_call.fields.hourly = false;
let res4 = openweather.one_call.call(lat, lon).await;
```

#### Forecast Query
```rust
let lat = 38.795021;
let lon = -77.273300;

let res = openweather.forecast.call(lat, lon).await;
```

#### Maps Query
```rust
let lat = 38.795021;
let lon = -77.273300;
let zoom = 1;
let x_tiles = 1;
let y_tiles = 1;

let res = openweather.maps.get_cloud_map(zoom, x_tiles, y_tiles).await;
let res2 = openweather.maps.get_precipitation_map(zoom, x_tiles, y_tiles).await;
let res3 = openweather.maps.get_temperature_map(zoom, x_tiles, y_tiles).await;
let res4 = openweather.maps.get_wind_speed_map(zoom, x_tiles, y_tiles).await;
let res5 = openweather.maps.get_pressure_map(zoom, x_tiles, y_tiles).await;
```

#### Air Pollution Query
```rust
let lat = 38.795021;
let lon = -77.273300;

let res = openweather.air_pollution.get_current_air_pollution(lat, lon).await;
let res2 = openweather.air_pollution.get_forecast_air_pollution(lat, lon).await;```

let start = 1606223802;
let end = 1606482999;
let res2 = openweather.air_pollution.get_historical_air_pollution(lat, lon, start, end).await;
```

#### Geocoding Query
```rust
let lat = 38.795021;
let lon = -77.273300;

let city = "Washington";
let state = "DC";
let country = "US";
let limit = 5;

let res = openweather.geocoding.get_geocoding(city, Some(state), Some(country), limit).await;
let res2 = openweather.geocoding.get_geocoding_by_zip_code("20001", None).await;
let res3 = openweather.geocoding.get_location_data(lat, lon, limit).await;
```

### Dependencies
- reqwest ^0.11
- tokio ^1.0
- serde ^1.0.152

### License
- MIT

### Contributing
- Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.
- Please make sure to update tests and documentation as appropriate.

### Author
- [Jeff Rose]

[OpenWeather API]: https://openweathermap.org/api
[OneCall]: https://openweathermap.org/api/one-call-3
[TimeMachine]: https://openweathermap.org/api/one-call-3#history
[Forecast]: https://openweathermap.org/forecast5
[Maps]: https://openweathermap.org/api/weathermaps
[Air Pollution]: https://openweathermap.org/api/air-pollution
[Geocoding]: https://openweathermap.org/api/geocoding-api
[serde]: https://serde.rs/
[Reqwest]: https://docs.rs/reqwest/0.11.4/reqwest/
[Jeff Rose]: www.github.com/jt-rose
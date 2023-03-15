# OpenWeather_SDK

This library is a small rust wrapper for making requests to the [OpenWeather API]. This library includes:

- query constructor with full coverage of free-tier API calls
- type-safe enums for accurate settings
- type-safe serde deserialization of responses
- async requests made with the Reqwest library

### Query Types Supported
- [x] [OneCall]
- [x] [TimeMachine]
- [x] [Forecast]
- [x] [Maps]
- [x] [Air Pollution]
- [x] [Geocoding]

### Getting Started

- coming soon!

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
[Jeff Rose]: www.github.com/jt-rose
use std::fmt;
use reqwest::Response;
use crate::languages::Language;
use crate::units::Units;
use super::map_layer::MapLayer;


pub struct Maps {
    api_key: &'static str,
    units: Units,
    language: Language,
}

impl Maps {
    pub fn new(api_key: &'static str, units: Units, language: Language) -> Self {
        Self {
            api_key,
            units,
            language
        }
    }

    fn format_query(&self, layer: MapLayer, zoom: u8, x_tiles: u8, y_tiles: u8) -> String {
        format!(
            "https://tile.openweathermap.org/map/{}/{}/{}/{}.png?appid={}",
            layer,
            zoom,
            x_tiles,
            y_tiles,
            self.api_key)
    }

    async fn get_map(&self, layer: MapLayer, zoom: u8, x_tiles: u8, y_tiles: u8) -> Result<Response, Box<dyn std::error::Error>> {
        let url = self.format_query(layer, zoom, x_tiles, y_tiles);
        let resp = reqwest::get(url)
            .await?;

        Ok(resp)
    }

    pub async fn get_cloud_map(&self, zoom: u8, x_tiles: u8, y_tiles: u8) -> Result<Response, Box<dyn std::error::Error>> {
        self.get_map(MapLayer::Clouds, zoom, x_tiles, y_tiles).await
    }

    pub async fn get_precipitation_map(&self, zoom: u8, x_tiles: u8, y_tiles: u8) -> Result<Response, Box<dyn std::error::Error>> {
        self.get_map(MapLayer::Precipitation, zoom, x_tiles, y_tiles).await
    }

    pub async fn get_pressure_map(&self, zoom: u8, x_tiles: u8, y_tiles: u8) -> Result<Response, Box<dyn std::error::Error>> {
        self.get_map(MapLayer::Pressure, zoom, x_tiles, y_tiles).await
    }

    pub async fn get_wind_speed_map(&self, zoom: u8, x_tiles: u8, y_tiles: u8) -> Result<Response, Box<dyn std::error::Error>> {
        self.get_map(MapLayer::Wind, zoom, x_tiles, y_tiles).await
    }

    pub async fn get_temperature_map(&self, zoom: u8, x_tiles: u8, y_tiles: u8) -> Result<Response, Box<dyn std::error::Error>> {
        self.get_map(MapLayer::Temperature, zoom, x_tiles, y_tiles).await
    }
}
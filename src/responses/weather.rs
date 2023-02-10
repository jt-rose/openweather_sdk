use serde::{ Serialize, Deserialize };

#[derive(Debug, Serialize, Deserialize)]
pub struct Weather {
    pub id: u64,
    pub main: String,
    pub description: String,
    pub icon: String
}
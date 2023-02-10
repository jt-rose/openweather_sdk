use serde::{ Serialize, Deserialize };

#[derive(Debug, Serialize, Deserialize)]
pub struct LocalNames {
    pub af: Option<String>,
    pub ar: Option<String>,
    pub ascii: Option<String>,
    pub az: Option<String>,
    pub bg: Option<String>,
    pub ca: Option<String>,
    pub da: Option<String>,
    pub de: Option<String>,
    pub el: Option<String>,
    pub en: Option<String>,
    pub eu: Option<String>,
    pub fa: Option<String>,
    pub feature_name: Option<String>,
    pub fi: Option<String>,
    pub fr: Option<String>,
    pub gl: Option<String>,
    pub he: Option<String>,
    pub hi: Option<String>,
    pub hr: Option<String>,
    pub hu: Option<String>,
    pub id: Option<String>,
    pub it: Option<String>,
    pub ja: Option<String>,
    pub la: Option<String>,
    pub lt: Option<String>,
    pub mk: Option<String>,
    pub nl: Option<String>,
    pub no: Option<String>,
    pub pl: Option<String>,
    pub pt: Option<String>,
    pub ro: Option<String>,
    pub ru: Option<String>,
    pub sk: Option<String>,
    pub sl: Option<String>,
    pub sr: Option<String>,
    pub th: Option<String>,
    pub tr: Option<String>,
    pub vi: Option<String>,
    pub zu: Option<String>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GeocodingResponse {
    pub name: String,
    pub local_names: Option<LocalNames>,
    pub lat: f64,
    pub lon: f64,
    pub country: String,
    pub state: Option<String>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ZipCodeResponse {
    pub zip: String,
    pub name: String,
    pub lat: f64,
    pub lon: f64,
    pub country: String
}
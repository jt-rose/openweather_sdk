use std::error::Error;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug, Clone, serde::Deserialize)]
pub struct ErrorResponse {
    pub cod: u16,
    pub message: String,
    pub parameters: Option<Vec<String>>,
}

impl ErrorResponse {
    pub async fn new(resp: reqwest::Response) -> Result<Self, Box<dyn Error>> {
        let err = resp.json::<Self>().await?;
        Ok(err)
    }
}

impl Display for ErrorResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut parameters = String::new();
        parameters.push('[');
        if let Some(params) = &self.parameters {
            for param in params {
                parameters.push_str(&format!("{}, ", param));
            }
        }
        parameters.push(']');
        write!(
            f,
            "ErrorResponse: (cod: {}, message: {}, parameters: {})",
            self.cod,
            self.message,
            parameters
        )
    }
}

impl std::error::Error for ErrorResponse {
    fn description(&self) -> &str {
        &self.message
    }
    fn cause(&self) -> Option<&dyn Error> {
        Some(self)
    }
}

#[cfg(test)]
mod tests {
    use crate::responses::error_response::ErrorResponse;

    #[test]
    fn test_error_response() {
        let json = r#"{
            "cod": 401,
            "message": "Invalid API key. Please see http://openweathermap.org/faq#error401 for more info.",
            "parameters": [
                "appid",
                "q"
            ]
        }"#;
        let err: ErrorResponse = serde_json::from_str(json).unwrap();
        assert_eq!(err.cod, 401);
        assert_eq!(err.message, "Invalid API key. Please see http://openweathermap.org/faq#error401 for more info.");
        assert_eq!(err.parameters.is_some(), true);
        let params = err.parameters.unwrap();
        assert_eq!(params[0], "appid");
        assert_eq!(params[1], "q");
    }
}

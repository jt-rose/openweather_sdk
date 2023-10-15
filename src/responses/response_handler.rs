use reqwest::StatusCode;
use crate::responses::ErrorResponse;

pub async fn response_handler<T>(resp: reqwest::Response) -> Result<T, Box<dyn std::error::Error>>
where
    T: serde::de::DeserializeOwned,
{
    return match resp.status() {
        StatusCode::OK => {
            let res = resp
                .json::<T>()
                .await?;
            Ok(res)
        },
        _ => {
            let err = resp
                .json::<ErrorResponse>()
                .await?;
            Err(Box::new(err))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::responses::{OneCallResponse, GeocodingResponse, response_handler};

    #[tokio::test]
    async fn test_response_handler_one_call() {
        let resp = reqwest::get("https://api.openweathermap.org/data/3.0/onecall?lat=33.44&lon=-94.04&exclude=hourly,daily&appid=bad_key")
            .await.expect("Request should have succeeded");
        let err_resp = response_handler::<OneCallResponse>(resp).await.expect_err("Response should have been an error");
        assert_eq!(err_resp.to_string(), "ErrorResponse: (cod: 401, message: Please note that using One Call 3.0 requires a separate subscription to the One Call by Call plan. Learn more here https://openweathermap.org/price. If you have a valid subscription to the One Call by Call plan, but still receive this error, then please see https://openweathermap.org/faq#error401 for more info., parameters: [])");
    }

    #[tokio::test]
    async fn test_response_handler_geocoding() {
        let resp = reqwest::get("https://api.openweathermap.org/geo/1.0/direct?q=Chicago&limit=5&appid=bad_key")
            .await.expect("Request should have succeeded");
        let err_resp = response_handler::<Vec<GeocodingResponse>>(resp).await.expect_err("Response should have been an error");
        assert_eq!(err_resp.to_string(), "ErrorResponse: (cod: 401, message: Invalid API key. Please see https://openweathermap.org/faq#error401 for more info., parameters: [])");
    }
}
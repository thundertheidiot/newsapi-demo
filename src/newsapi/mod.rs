pub mod article;
pub mod response;

use thiserror::Error;

const BASE_URL: &'static str = "https://newsapi.org";

#[derive(Debug, Error)]
pub enum NewsAPIError {
    #[error("HTTP request failed: {0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("Failed to parse JSON response: {0}")]
    Serde(#[from] serde_json::Error),
    #[error("Invalid Header Value")]
    HeaderValue(#[from] reqwest::header::InvalidHeaderValue),
    #[error("API returned an error: {message} (code {code})")]
    Api { code: String, message: String },
}

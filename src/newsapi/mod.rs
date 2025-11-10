pub mod article;
pub mod response;

use crate::newsapi::article::Article;
use reqwest::Client;
use serde::Deserialize;
use thiserror::Error;

/// Catchall error type for all the possible errors that can happen.
///
///
#[derive(Debug, Error)]
pub enum NewsAPIError {
    /// Error returned by NewsAPI, e.g.
    #[error("API returned an error: {message} (code {code})")]
    Api {
        /// Error code returned by the API.
        code: String,
        /// Human-readable error message.
        message: String,
    },
    /// Error while making the HTTP request.
    #[error("HTTP request failed: {0}")]
    Reqwest(#[from] reqwest::Error),
    /// Error while parsing the JSON response body.
    #[error("Failed to parse JSON response: {0}")]
    Serde(#[from] serde_json::Error),
    #[error("IO Error: {0:?}")]
    IO(#[from] std::io::Error),
    #[error("Invalid Header Value")]
    HeaderValue(#[from] reqwest::header::InvalidHeaderValue),
}

#[derive(Debug, Deserialize, Clone)]
pub struct NewsAPISuccess {
    pub status: String,
    #[serde(rename(deserialize = "totalResults"))]
    pub total_results: i32,
    pub articles: Vec<Article>,
}

#[derive(Debug, Deserialize)]
pub struct NewsAPIFail {
    pub status: String,
    pub code: String,
    pub message: String,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum NewsAPIResponse {
    Success(NewsAPISuccess),
    Fail(NewsAPIFail),
}

impl Into<Result<NewsAPISuccess, NewsAPIError>> for NewsAPIResponse {
    fn into(self) -> Result<NewsAPISuccess, NewsAPIError> {
        match self {
            NewsAPIResponse::Success(v) => Ok(v),
            NewsAPIResponse::Fail(e) => Err(NewsAPIError::Api {
                code: e.code,
                message: e.message,
            }),
        }
    }
}

pub async fn fetch_top(client: &Client) -> Result<NewsAPISuccess, NewsAPIError> {
    client
        .get("https://newsapi.org/v2/top-headlines")
        .query(&[("category", "general")])
        .send()
        .await?
        .json::<NewsAPIResponse>()
        .await?
        .into()
}

pub async fn search(client: &Client, query: &str) -> Result<NewsAPISuccess, NewsAPIError> {
    client
        .get("https://newsapi.org/v2/top-headlines")
        .query(&[("q", query)])
        .send()
        .await?
        .json::<NewsAPIResponse>()
        .await?
        .into()
}

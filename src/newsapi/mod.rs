pub mod article;
pub mod source;

use crate::newsapi::article::Article;
use crate::newsapi::source::Source;
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
    #[error("Invalid Image {0}")]
    Image(#[from] image::ImageError),
    #[error("Invalid Header Value")]
    HeaderValue(#[from] reqwest::header::InvalidHeaderValue),
}

// the top-headlines and everything endpoints share this response format
#[derive(Debug, Deserialize, Clone)]
pub struct NewsAPIArticlesSuccess {
    pub status: String,
    #[serde(rename(deserialize = "totalResults"))]
    pub total_results: i32,
    #[serde(default)]
    pub articles: Vec<Article>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct NewsAPISourcesSuccess {
    pub status: String,
    #[serde(default)]
    pub sources: Vec<Source>,
}

#[derive(Debug, Deserialize)]
pub struct NewsAPIFail {
    pub status: String,
    pub code: String,
    pub message: String,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum NewsAPIArticleResponse {
    Success(NewsAPIArticlesSuccess),
    Fail(NewsAPIFail),
}

impl From<NewsAPIArticleResponse> for Result<NewsAPIArticlesSuccess, NewsAPIError> {
    fn from(val: NewsAPIArticleResponse) -> Self {
        match val {
            NewsAPIArticleResponse::Success(v) => Ok(v),
            NewsAPIArticleResponse::Fail(e) => Err(NewsAPIError::Api {
                code: e.code,
                message: e.message,
            }),
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum NewsAPISourceResponse {
    Success(NewsAPISourcesSuccess),
    Fail(NewsAPIFail),
}

impl From<NewsAPISourceResponse> for Result<NewsAPISourcesSuccess, NewsAPIError> {
    fn from(val: NewsAPISourceResponse) -> Self {
        match val {
            NewsAPISourceResponse::Success(v) => Ok(v),
            NewsAPISourceResponse::Fail(e) => Err(NewsAPIError::Api {
                code: e.code,
                message: e.message,
            }),
        }
    }
}

pub async fn fetch_top(
    client: &Client,
    sources: Option<String>,
) -> Result<NewsAPIArticlesSuccess, NewsAPIError> {
    let request = client.get("https://newsapi.org/v2/top-headlines");

    let request = match sources.as_deref() {
        None => request.query(&[("category", "general")]),
        Some("") => request.query(&[("category", "general")]),
        Some(s) => request.query(&[("sources", s)]),
    };

    request
        .send()
        .await?
        .json::<NewsAPIArticleResponse>()
        .await?
        .into()
}

pub async fn search_articles(
    client: &Client,
    query: &str,
    sources: Option<String>,
) -> Result<NewsAPIArticlesSuccess, NewsAPIError> {
    let request = client.get("https://newsapi.org/v2/everything");

    let request = match sources.as_deref() {
        None => request.query(&[("q", query)]),
        Some("") => request.query(&[("q", query)]),
        Some(s) => request.query(&[("q", query), ("sources", s)]),
    };

    request
        .send()
        .await?
        .json::<NewsAPIArticleResponse>()
        .await?
        .into()
}

pub async fn fetch_sources(client: &Client) -> Result<NewsAPISourcesSuccess, NewsAPIError> {
    client
        .get("https://newsapi.org/v2/top-headlines/sources")
        .send()
        .await?
        .json::<NewsAPISourceResponse>()
        .await?
        .into()
}

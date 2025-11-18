pub mod article;
pub mod source;

use crate::newsapi::article::Article;
use crate::newsapi::source::Source;
use reqwest::Client;
use serde::Deserialize;
use thiserror::Error;

/// Catchall error type for operations involving the NewsAPI client, HTTP requests,
/// JSON parsing, filesystem IO and image handling.
#[derive(Debug, Error)]
pub enum NewsAPIError {
    /// Error returned by the NewsAPI service when the API responds with an
    /// error payload (for example invalid parameters, rate limiting, etc).
    ///
    /// Fields:
    /// - `code`: machine-readable error code returned by the API (string).
    /// - `message`: human-readable explanation of the error.
    #[error("API returned an error: {message} (code {code})")]
    Api {
        /// Error code returned by the API.
        code: String,
        /// Human-readable error message.
        message: String,
    },
    /// Error from the HTTP client / transport layer (reqwest).
    ///
    /// This covers network failures, timeouts, TLS errors, invalid responses,
    /// and other problems originating from the HTTP stack.
    #[error("HTTP request failed: {0:?}")]
    Reqwest(#[from] reqwest::Error),
    /// Error while parsing the JSON response body.
    ///
    /// Occurs when the HTTP response body cannot be deserialized into the
    /// expected struct (malformed JSON, missing fields, type errors, etc).
    #[error("Failed to parse JSON response: {0:?}")]
    Serde(#[from] serde_json::Error),
    /// Filesystem or other IO error.
    ///
    /// Used for errors encountered when reading from or writing to the local
    /// cache (or other I/O operations).
    #[error("IO Error: {0:?}")]
    IO(#[from] std::io::Error),
    /// Image decoding/format error.
    ///
    /// Produced when image data cannot be decoded or when the image crate
    /// reports a problem (unsupported format, truncated data, etc).
    #[error("Invalid Image {0}")]
    Image(#[from] image::ImageError),
    /// Invalid HTTP header value (used when constructing the API key header).
    ///
    /// Returned when the provided API token cannot be converted into a valid
    /// `HeaderValue`.
    #[error("Invalid Header Value")]
    HeaderValue(#[from] reqwest::header::InvalidHeaderValue),
}

/// Response returned by the articles endpoints (top-headlines and everything).
///
/// This struct models the successful JSON payload.
#[derive(Debug, Deserialize, Clone)]
pub struct NewsAPIArticlesSuccess {
    /// API status string, either "ok" or "error", should be "ok" here, but this is unused.
    pub status: String,
    /// Total number of results reported by the API.
    /// Deserialized from the JSON key "totalResults".
    #[serde(rename(deserialize = "totalResults"))]
    pub total_results: i32,
    /// Articles returned for this page.
    /// Defaults to an empty vector when the JSON field is missing or null.
    #[serde(default)]
    pub articles: Vec<Article>,
}
/// Response returned by the sources endpoint.
///
/// This struct models the successful JSON payload.
#[derive(Debug, Deserialize, Clone)]
pub struct NewsAPISourcesSuccess {
    /// API status string, either "ok" or "error", should be "ok" here, but this is unused.
    pub status: String,
    /// List of available sources returned by the API.
    /// Defaults to an empty vector when the JSON field is missing or null.
    #[serde(default)]
    pub sources: Vec<Source>,
}

/// Error payload returned by the NewsAPI on failed requests.
#[derive(Debug, Deserialize)]
pub struct NewsAPIFail {
    /// API status string, either "ok" or "error", should be "error" here, but this is unused.
    pub status: String,
    /// Error code returned by the API.
    pub code: String,
    /// Human-readable error message explaining the failure.
    pub message: String,
}

/// Response from the articles endpoints; either a successful payload or an API error.
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

/// Response from the source endpoint; either a successful payload or an API error.
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

/// Fetch top headlines from NewsAPI using the `v2/top-headlines` endpoint.
///
/// Parameters:
/// - `client`: Reqwest client configured with the NewsAPI API key
/// - `sources`: optional comma-separated source ids, empty string is treated as omitted
///
/// Expectations:
/// - when sources is Some it should contain a valid comma-separated list of source ids accepted by the API
pub async fn fetch_top(
    client: &Client,
    sources: Option<String>,
) -> Result<NewsAPIArticlesSuccess, NewsAPIError> {
    let request = client.get("https://newsapi.org/v2/top-headlines");

    let request = match sources.as_deref() {
        None | Some("") => request.query(&[("category", "general")]),
        Some(s) => request.query(&[("sources", s)]),
    };

    request
        .send()
        .await?
        .json::<NewsAPIArticleResponse>()
        .await?
        .into()
}

/// Search for articles using the `v2/everything` endpoint.
///
/// Parameters:
/// - `client`: Reqwest client configured with the NewsAPI API key
/// - `query`: search query string
/// - `sources`: optional comma-separated source ids, empty string is treated as omitted
///
/// Expectations:
/// - when sources is Some it should contain a valid comma-separated list of source ids accepted by the API
pub async fn search_articles(
    client: &Client,
    query: &str,
    sources: Option<String>,
) -> Result<NewsAPIArticlesSuccess, NewsAPIError> {
    let request = client.get("https://newsapi.org/v2/everything");

    let request = match sources.as_deref() {
        None | Some("") => request.query(&[("q", query)]),
        Some(s) => request.query(&[("q", query), ("sources", s)]),
    };

    request
        .send()
        .await?
        .json::<NewsAPIArticleResponse>()
        .await?
        .into()
}

/// Fetch available sources from the NewsAPI `top-headlines/sources` endpoint.
///
/// Parameters:
/// - `client`: Reqwest client configured with the NewsAPI API key
pub async fn fetch_sources(client: &Client) -> Result<NewsAPISourcesSuccess, NewsAPIError> {
    client
        .get("https://newsapi.org/v2/top-headlines/sources")
        .send()
        .await?
        .json::<NewsAPISourceResponse>()
        .await?
        .into()
}

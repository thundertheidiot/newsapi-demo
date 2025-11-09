use crate::newsapi::article::Article;
use reqwest::RequestBuilder;
use serde::Deserialize;

/// Response of the `top-headlines` endpoint
#[derive(Debug, Deserialize, Clone)]
pub struct TopHeadlinesResponse {
    pub status: Option<String>,
    #[serde(rename(deserialize = "totalResults"))]
    pub total_results: Option<i32>,
    pub articles: Vec<Article>,
}

/// Response of the `everything` endpoint
#[derive(Debug, Deserialize, Clone)]
pub struct EverythingResponse {
    pub status: String,
    #[serde(rename(deserialize = "totalResults"))]
    pub total_results: Option<i32>,
    pub articles: Vec<Article>,
}

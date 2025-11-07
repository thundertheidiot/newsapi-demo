use serde::Deserialize;

/// A representation of the `source` object in the article responses
#[derive(Debug, Deserialize)]
pub struct ArticleSource {
    /// Id for the source of the article
    pub id: Option<String>,
    /// Display name for the source of the article
    pub name: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Article {
    pub source: ArticleSource,
    pub author: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub url: Option<String>,
    #[serde(rename(deserialize = "urlToImage"))]
    pub url_to_image: Option<String>,
    #[serde(rename(deserialize = "publishedAt"))]
    pub published_at: Option<String>,
    pub content: Option<String>,
}

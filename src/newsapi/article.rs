use serde::Deserialize;

/// Source object for an article response.
/// The article data seems particularly flaky and the documentation is rather poor
/// Most fields here are marked as Option to be cautious
#[derive(Debug, Deserialize, Clone)]
pub struct ArticleSource {
    /// Id for the source of the article
    pub id: Option<String>,
    /// Display name for the source of the article
    pub name: Option<String>,
}

/// A single article returned by NewsAPI.
/// The article data seems particularly flaky and the documentation is rather poor
/// Most fields here are marked as Option to be cautious
#[derive(Debug, Deserialize, Clone)]
pub struct Article {
    /// Source metadata
    pub source: ArticleSource,
    /// Author
    pub author: Option<String>,
    /// Article title
    pub title: String,
    /// Article description
    pub description: Option<String>,
    /// Article url
    pub url: Option<String>,
    /// Image url
    #[serde(rename(deserialize = "urlToImage"))]
    pub url_to_image: Option<String>,
    /// Timestamp of publication, in ISO 8601 format
    #[serde(rename(deserialize = "publishedAt"))]
    pub published_at: Option<String>,
    /// Truncated portion of the content of the article
    /// This may contain html formatting
    pub content: Option<String>,
}

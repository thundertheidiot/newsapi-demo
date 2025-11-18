use serde::Deserialize;

/// Source data retrieved from the
#[derive(Debug, Deserialize, Clone)]
pub struct Source {
    /// Id of the source, used for filtering by source with the other endpoints
    pub id: String,
    /// Display name
    pub name: String,
    /// Longer description
    pub description: String,
    /// Link to the source
    pub url: String,
    /// Category of the source
    pub category: String,
    /// Language
    pub language: String,
    /// Country
    pub country: String,
}

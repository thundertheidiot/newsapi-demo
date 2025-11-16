use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Source {
    pub id: String,
    pub name: String,
    pub description: String,
    pub url: String,
    pub category: String,
    pub language: String,
    pub country: String,
}

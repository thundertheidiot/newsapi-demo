use crate::newsapi::response::EverythingResponse;
use crate::ui::App;
use reqwest::header::HeaderValue;
use reqwest::header::HeaderMap;

use crate::newsapi::response::TopHeadlinesResponse;
use crate::newsapi::NewsAPIError;
use std::env;

mod newsapi;
mod ui;

fn main() -> iced::Result {
    iced::application("NewsAPI Demo", App::update, App::view).run_with(App::new)
}

// #[tokio::main]
async fn demo() -> Result<(), NewsAPIError> {
    let token = env::var("NEWS_API_TOKEN").expect("$NEWS_API_TOKEN should be set.");

    let mut headers = HeaderMap::new();
    headers.insert("X-Api-Key", HeaderValue::from_str(&token)?);

    let client = reqwest::ClientBuilder::new()
        .user_agent("NewsAPI Demo Application")
        .default_headers(headers)
        .build()?;

    let res1 = client.get("https://newsapi.org/v2/top-headlines")
        .query(&[("category", "general")])
        .send()
        .await?
	.json::<TopHeadlinesResponse>()
	.await?;

    let res2 = client.get("https://newsapi.org/v2/everything")
        .query(&[("q", "bitcoin AND ethereum")])
        .send()
        .await?
	.json::<EverythingResponse>()
	.await?;

    println!("res1 saatana");
    println!("{:#?}", res1);
    println!("res2 saatana");
    println!("{:#?}", res2);

    Ok(())
}

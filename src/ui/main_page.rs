use crate::TopHeadlinesResponse;
use crate::newsapi::article::Article;
use crate::ui::article::article_to_element;
use iced::Alignment;
use iced::widget::Row;
use std::time::Duration;

use crate::newsapi::NewsAPIError;
use crate::newsapi::response::EverythingResponse;
use crate::ui::Action;
use crate::ui::Message;
use crate::ui::Page;
use iced::Element;
use iced::Length;
use iced::Task;
use iced::widget::Column;
use iced::widget::Space;
use iced::widget::button;
use iced::widget::scrollable;
use iced::widget::text_input;
use iced::widget::{column, row, text};
use reqwest::Client;
use reqwest::header::HeaderMap;
use reqwest::header::HeaderValue;

pub struct MainPage {
    client: Client,
    search_query: String,
    text: Option<String>,
    articles: Vec<Article>,
}

#[derive(Debug, Clone)]
pub enum MainPageMessage {
    SearchBarOnInput(String),
    SearchSubmit,
    SearchComplete(Result<TopHeadlinesResponse, String>),
}

impl MainPage {
    pub fn new(token: String) -> Result<Self, NewsAPIError> {
        let mut headers = HeaderMap::new();
        headers.insert("X-Api-Key", HeaderValue::from_str(&token)?);

        let client = reqwest::ClientBuilder::new()
            .user_agent("NewsAPI Demo Application")
            .default_headers(headers)
            .build()?;

        Ok(Self {
            client: client,
            search_query: String::new(),
            text: None,
            articles: Vec::new(),
        })
    }
}

impl Page for MainPage {
    fn view(&self) -> Element<'_, Message> {
        use MainPageMessage::*;
        use Message::MainPage as M;

        column![
            row![
                text_input("Search: ", &self.search_query)
                    .on_input(|s| M(SearchBarOnInput(s)))
                    .on_submit(M(SearchSubmit))
                    .size(24),
                button("Submit").on_press(M(SearchSubmit)).padding(10)
            ]
            .spacing(5)
            .padding(15),
            scrollable(
                Column::with_children(self.articles.chunks(3).map(|chunk| {
                    Into::<Element<'_, Message>>::into(
                        Row::with_children(chunk.into_iter().map(article_to_element))
                            .spacing(10)
                            .align_y(Alignment::Center),
                    )
                }))
                .spacing(15)
                .padding(10)
                .align_x(Alignment::Center)
            )
        ]
        .spacing(10)
        .into()
    }

    fn update(&mut self, message: Message) -> Action {
        use MainPageMessage::*;
        use Message::MainPage as M;

        if let Message::MainPage(message) = message {
            match message {
                SearchBarOnInput(s) => self.search_query = s,
                SearchSubmit => {
                    let client = self.client.clone();
                    let query = self.search_query.clone();

                    return Action::Task(Task::perform(
                        async move {
                            /// This function exists to group up all the possible errors, so they can be handled at once
                            async fn fallible(
                                client: &Client,
                                query: &str,
                            ) -> Result<TopHeadlinesResponse, NewsAPIError>
                            {
                                Ok(match query {
                                    "" => client
                                        .get("https://newsapi.org/v2/top-headlines")
                                        .query(&[("category", "general")])
                                        .send(),
                                    s => client
                                        .get("https://newsapi.org/v2/top-headlines")
                                        .query(&[("q", s)])
                                        .send(),
                                }
                                .await?
                                .json::<TopHeadlinesResponse>()
                                .await?)
                            }

                            fallible(&client, &query).await.map_err(|e| e.to_string())
                        },
                        |v| M(SearchComplete(v)),
                    ));
                }
                SearchComplete(v) => match v {
                    Ok(v) => {
                        println!("yea");
                        self.articles = v.articles;
                    }
                    Err(e) => self.text = Some(e),
                },
            }
        }

        Action::None
    }
}

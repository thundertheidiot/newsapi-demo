use crate::TopHeadlinesResponse;
use crate::fetch_top;
use crate::newsapi::NewsAPISuccess;
use crate::newsapi::article::Article;
use crate::newsapi::search;
use crate::ui::article::article_to_element;
use iced::Alignment;
use iced::color;
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
    search_result: Option<Result<NewsAPISuccess, String>>,
    articles: Vec<Article>,
}

#[derive(Debug, Clone)]
pub enum MainPageMessage {
    SearchBarOnInput(String),
    SearchSubmit,
    SearchComplete(Result<NewsAPISuccess, String>),
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
            search_result: None,
        })
    }
}

impl Page for MainPage {
    fn view(&self) -> Element<'_, Message> {
        use MainPageMessage::*;
        use Message::MainPage as M;

        let article_view: Element<'_, Message> = match &self.search_result {
            Some(v) => match v {
                Ok(resp) => Column::with_children(resp.articles.chunks(3).map(|chunk| {
                    Into::<Element<'_, Message>>::into(
                        Row::with_children(chunk.iter().map(article_to_element))
                            .spacing(10)
                            .align_y(Alignment::Center),
                    )
                }))
                .into(),
                Err(e) => text(e).color(color!(0xff0000)).into(),
            },
            None => Space::with_width(0).into(),
        };

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
            scrollable(article_view)
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
                            match query.as_str() {
                                "" => fetch_top(&client).await,
                                query => search(&client, query).await,
                            }
                            .map_err(|e| e.to_string())
                        },
                        |v| M(SearchComplete(v)),
                    ));
                }
                SearchComplete(v) => self.search_result = Some(v),
            }
        }

        Action::None
    }
}

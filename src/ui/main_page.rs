use crate::TopHeadlinesResponse;
use crate::fetch_top;
use crate::newsapi::NewsAPISuccess;
use crate::newsapi::article::Article;
use crate::newsapi::search;
use crate::ui::article::article_to_card;
use crate::ui::article::get_image_from_url;
use iced::Alignment;
use iced::color;
use iced::futures::SinkExt;
use iced::widget::Row;
use iced::widget::Stack;
use iced::widget::container;
use iced::widget::image;
use iced::widget::image::Handle;
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
    search_result: Option<Result<NewsAPISuccess, String>>,
    images_loaded: Vec<Option<Handle>>,
    active_article: Option<usize>,
}

#[derive(Debug, Clone)]
pub enum MainPageMessage {
    SearchBarOnInput(String),
    SearchSubmit,
    SearchComplete(Result<NewsAPISuccess, String>),
    // Handle is a reference to bytes, doesn't own the data
    ImageLoaded(Option<(usize, Handle)>),
    ActiveArticle(usize),
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
            search_result: None,
            active_article: None,
            images_loaded: Vec::new(),
        })
    }
}

impl Page for MainPage {
    fn view(&self) -> Element<'_, Message> {
        use MainPageMessage::*;
        use Message::MainPage as M;

        let mut items: Vec<Element<'_, Message>> = Vec::new();

        let article_view: Element<'_, Message> = match &self.search_result {
            Some(v) => match v {
                Ok(resp) => Column::with_children(
                    resp.articles
                        .iter()
                        .enumerate()
                        .collect::<Vec<(usize, &Article)>>()
                        .chunks(3)
                        .map(|chunk| {
                            Into::<Element<'_, Message>>::into(
                                Row::with_children(
                                    // TODO: optimize this
                                    // usize gets copied through the dereference
                                    chunk.into_iter().map(|(i, a)| {
                                        article_to_card(*i, a, &self.images_loaded[*i])
                                    }),
                                )
                                .spacing(10)
                                .align_y(Alignment::Center),
                            )
                        }),
                )
                .into(),
                Err(e) => container(text(e).color(color!(0xff0000)).size(24))
                    .padding(15)
                    .into(),
            },
            None => Space::with_width(0).into(),
        };

        items.push(scrollable(article_view).into());

        if let (Some(index), Some(Ok(data))) = (self.active_article, &self.search_result) {
            match &data.articles[index].content {
                Some(v) => items.push(text(v).into()),
                None => items.push(text("meow :3").into()),
            }
        }

        column![
            row![
                text_input("Search: ", &self.search_query)
                    .on_input(|s| M(SearchBarOnInput(s)))
                    .on_submit(M(SearchSubmit))
                    .size(24),
                button("Submit").on_press(M(SearchSubmit)).padding(10),
            ]
            .spacing(5)
            .padding(15),
            Stack::from_vec(items)
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
                SearchComplete(v) => {
                    self.active_article = None;
                    let mut tasks: Task<Message> = Task::none();

                    self.images_loaded = Vec::new();

                    if let Ok(data) = &v {
                        self.images_loaded.resize(data.articles.len(), None);

                        tasks =
                            Task::batch(data.articles.iter().enumerate().map(|(i, article)| {
                                match &article.url_to_image {
                                    Some(url) => {
                                        // gets passed to separate task
                                        let url = url.to_owned();
                                        Task::perform(
                                            async move {
                                                match get_image_from_url(&url).await {
                                                    Ok(bytes) => {
                                                        Some((i, Handle::from_bytes(bytes)))
                                                    }
                                                    Err(e) => {
                                                        eprintln!("Error getting image: {e:#?}");
                                                        None
                                                    }
                                                }
                                            },
                                            |data| M(ImageLoaded(data)),
                                        )
                                    }
                                    None => Task::none(),
                                }
                            }));
                    }

                    self.search_result = Some(v);
                    return Action::Task(tasks);
                }
                ImageLoaded(data) => {
                    if let Some((i, handle)) = data {
                        // images_loaded is resized to article amount above, this should be safe
                        self.images_loaded[i] = Some(handle);
                    }
                }
                ActiveArticle(index) => {
                    self.active_article = Some(index);
                    println!("clicked on {index}");
                }
                _ => (),
            }
        }

        Action::None
    }
}

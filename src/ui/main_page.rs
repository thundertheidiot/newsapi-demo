use crate::fetch_top;
use crate::newsapi::NewsAPIArticlesSuccess;
use crate::newsapi::NewsAPISourcesSuccess;
use crate::newsapi::article::Article;
use crate::newsapi::search_articles;
use crate::ui::SEARCH_BAR_ID;
use crate::ui::TOKEN_INPUT_ID;
use crate::ui::article::article_to_card;
use crate::ui::article::article_view;
use crate::ui::article::get_image_from_url;
use crate::ui::source::source_toggle;
use crate::ui::style::LIST_ICON;
use crate::ui::style::SEARCH_ICON;
use crate::ui::style::button_style;
use crate::ui::style::text_input_style;
use crate::ui::token_page::TokenPage;
use iced::Alignment;
use iced::Background;
use iced::Border;
use iced::Theme;
use iced::color;
use iced::widget::Row;
use iced::widget::Space;
use iced::widget::Stack;
use iced::widget::container;
use iced::widget::image::Handle;
use iced::widget::mouse_area;
use iced::widget::svg;
use iced::widget::text_input::focus;
use iced::widget::tooltip;
use std::collections::HashMap;

use crate::newsapi::NewsAPIError;
use crate::ui::Action;
use crate::ui::Message;
use crate::ui::Page;
use iced::Element;
use iced::Length;
use iced::Task;
use iced::widget::Column;
use iced::widget::button;
use iced::widget::scrollable;
use iced::widget::text_input;
use iced::widget::{column, row, text};
use reqwest::Client;
use reqwest::header::HeaderMap;
use reqwest::header::HeaderValue;

pub struct MainPage {
    pub client: Client,
    search_query: String,
    search_result: Option<Result<NewsAPIArticlesSuccess, String>>,
    images_loaded: Vec<Option<Handle>>,
    active_article: Option<usize>,
    source_data: Option<Result<NewsAPISourcesSuccess, String>>,
    enabled_sources: HashMap<String, bool>,
    source_page: bool,
}

#[derive(Debug, Clone)]
pub enum MainPageMessage {
    SearchBarOnInput(String),
    SearchSubmit,
    SearchComplete(Result<NewsAPIArticlesSuccess, String>),
    SourcesFetched(Result<NewsAPISourcesSuccess, String>),
    SourceToggled(String, bool),
    // Handle is a reference to bytes, doesn't own the data
    ImageLoaded(Option<(usize, Handle)>),
    ActiveArticle(Option<usize>),
    ToggleSourcePage,
    BackToApiKeyPage,
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
            client,
            search_query: String::new(),
            search_result: None,
            active_article: None,
            images_loaded: Vec::new(),
            source_data: None,
            source_page: false,
            enabled_sources: HashMap::new(),
        })
    }
}

impl Page for MainPage {
    fn view(&self, size: (f32, f32)) -> Element<'_, Message> {
        use MainPageMessage::*;
        use Message::MainPage as M;

        let w = size.0;
        let mut chunks = (w / 400.0).floor() as usize;
        if chunks < 1 {
            chunks = 1;
        }

        Stack::with_capacity(3)
            .push(
                Column::with_capacity(2) // allocate biggest possible capacity
                    .push(
                        // search bar component
                        row![
                            text_input("Search for articles", &self.search_query)
                                .on_input(|s| M(SearchBarOnInput(s)))
                                .on_submit(M(SearchSubmit))
                                .id(SEARCH_BAR_ID) // id for focus task
                                .style(text_input_style)
                                .width(Length::FillPortion(19))
                                .size(24),
                            button(svg(iced::advanced::svg::Handle::from_memory(SEARCH_ICON)))
                                .on_press(M(SearchSubmit))
                                .padding(10)
                                .width(Length::FillPortion(1))
                                .height(Length::Fill)
                                .style(button_style),
                            tooltip(
                                button(row![
                                    svg(iced::advanced::svg::Handle::from_memory(LIST_ICON)),
                                    text(self.enabled_sources.values().filter(|v| **v).count())
                                ])
                                .on_press(M(ToggleSourcePage))
                                .padding(10)
                                .width(Length::FillPortion(1))
                                .height(Length::Fill)
                                .style(button_style),
                                container(text("Sources"))
                                    .padding(5)
                                    .style(|theme: &Theme| {
                                        container::Style {
                                            background: Some(Background::Color(
                                                theme.palette().background,
                                            )),
                                            border: Border::default()
                                                .color(theme.palette().primary)
                                                .rounded(5),
                                            ..Default::default()
                                        }
                                    }),
                                tooltip::Position::Bottom,
                            )
                        ]
                        .height(Length::Fixed(72.0))
                        .spacing(5)
                        .padding(15),
                    )
                    // only show list of article cards if search result exists
                    // it will be None at the start
                    .push_maybe(match &self.search_result {
                        // scrollable list of article cards
                        Some(Ok(data)) => Some::<Element<'_, Message>>(
                            scrollable(
                                Column::with_children(
                                    data.articles
                                        .iter()
                                        .enumerate()
                                        .collect::<Vec<(usize, &Article)>>()
                                        .chunks(chunks)
                                        .map(|chunk| {
                                            Into::<Element<'_, Message>>::into(
                                                Row::with_children(chunk.iter().map(|(i, a)| {
                                                    article_to_card(*i, a, &self.images_loaded[*i])
                                                }))
                                                .spacing(10)
                                                .align_y(Alignment::Center),
                                            )
                                        }),
                                )
                                .spacing(5)
                                .padding(5),
                            )
                            .spacing(5)
                            .height(Length::Fill)
                            .into(),
                        ),
                        // Error message
                        Some(Err(error)) => Some(
                            container(column![
                                text(error).color(color!(0xff0000)).size(32),
                                button("Back to API key page")
                                    .style(button_style)
                                    .on_press(M(BackToApiKeyPage))
                            ])
                            .padding(15)
                            .into(),
                        ),
                        _ => None,
                    }),
            )
            .push_maybe(match (self.active_article, &self.search_result) {
                (Some(index), Some(Ok(data))) => Some(
                    mouse_area(
                        container(article_view(
                            &data.articles[index],
                            &self.images_loaded[index],
                        ))
                        .padding(20)
                        .width(Length::Fill)
                        .height(Length::Fill)
                        .center(Length::Fill)
                        .align_x(Alignment::Center)
                        .align_y(Alignment::Center)
                        .style(|_theme| container::Style {
                            background: None,
                            ..Default::default()
                        }),
                    )
                    .interaction(iced::mouse::Interaction::Idle)
                    .on_right_press(M(ActiveArticle(None)))
                    .on_press(M(ActiveArticle(None))),
                ),
                _ => None,
            })
            .push_maybe(match &self.source_page {
                true => Some(column![
                    (match &self.source_data {
                        Some(Ok(data)) =>
                            Column::with_children(data.sources.iter().map(|source| {
                                source_toggle(
                                    source,
                                    self.enabled_sources.get(&source.id).unwrap_or(&false),
                                )
                            }))
                            .into(),
                        Some(Err(error)) => container(column![
                            text(error).color(color!(0xff0000)).size(32),
                            button("Back to API key page")
                                .style(button_style)
                                .on_press(M(BackToApiKeyPage))
                        ])
                        .padding(15)
                        .into(),
                        _ => Into::<Element<'_, Message>>::into(Space::with_width(1)),
                    })
                ]),
                false => None,
            })
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

                    let sources: Option<String> = Some(
                        self.enabled_sources
                            .iter()
                            .filter(|(_, v)| **v)
                            .map(|(k, _)| k.as_ref())
                            .collect::<Vec<&str>>()
                            .join(","),
                    );

                    return Action::Task(Task::perform(
                        async move {
                            match query.as_str() {
                                "" => fetch_top(&client, sources).await,
                                query => search_articles(&client, query, sources).await,
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

                        tasks = Task::batch(data.articles.iter().enumerate().map(image_task));
                    }

                    self.search_result = Some(v);
                    return Action::Task(tasks);
                }
                SourcesFetched(v) => {
                    if let Ok(data) = &v {
                        self.enabled_sources.clear();

                        for s in &data.sources {
                            self.enabled_sources.insert(s.id.to_owned(), false);
                        }
                    }

                    self.source_data = Some(v);
                }
                SourceToggled(id, state) => {
                    self.enabled_sources.insert(id, state);
                    ()
                }
                ToggleSourcePage => {
                    self.source_page = !self.source_page;
                }
                ImageLoaded(data) => {
                    if let Some((i, handle)) = data {
                        // images_loaded is resized to article amount above, this should be safe
                        self.images_loaded[i] = Some(handle);
                    }
                }
                ActiveArticle(index) => {
                    self.active_article = index;
                }
                BackToApiKeyPage => {
                    return Action::SwitchPage((Box::new(TokenPage::new()), focus(TOKEN_INPUT_ID)));
                }
            }
        }

        Action::None
    }
}

fn image_task(input: (usize, &Article)) -> Task<Message> {
    let (index, article) = input;

    match &article.url_to_image {
        Some(url) => {
            let url = url.to_owned();

            Task::perform(
                async move {
                    match get_image_from_url(&url).await {
                        Ok(bytes) => Some((index, Handle::from_bytes(bytes))),
                        Err(e) => {
                            eprintln!("Error getting image: {e:#?}");
                            None
                        }
                    }
                },
                |data| Message::MainPage(MainPageMessage::ImageLoaded(data)),
            )
        }
        None => Task::none(),
    }
}

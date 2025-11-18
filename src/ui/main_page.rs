use crate::fetch_top;
use crate::newsapi::NewsAPIArticlesSuccess;
use crate::newsapi::NewsAPISourcesSuccess;
use crate::newsapi::article::Article;
use crate::newsapi::search_articles;
use crate::ui::SEARCH_BAR_ID;
use crate::ui::TOKEN_INPUT_ID;
use crate::ui::article::article_cards;
use crate::ui::article::article_page;
use crate::ui::article::get_image_from_url;
use crate::ui::source::source_page;
use crate::ui::style::LIST_ICON;
use crate::ui::style::SEARCH_ICON;
use crate::ui::style::button_style;
use crate::ui::style::text_input_style;
use crate::ui::token_page::TokenPage;
use iced::Background;
use iced::Border;
use iced::Theme;
use iced::color;
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
use iced::widget::text_input;
use iced::widget::{column, row, text};
use reqwest::Client;
use reqwest::header::HeaderMap;
use reqwest::header::HeaderValue;

///  State and UI model for the main page of the application.
pub struct MainPage {
    /// reqwest client used to fetch NewsAPI data
    /// `new` instantiates this with a token
    pub client: Client,
    /// stores the content of the search box
    search_query: String,
    /// search result data
    search_result: Option<Result<NewsAPIArticlesSuccess, String>>,
    /// loaded images by article index
    /// the length is actively managed by any of the fetch events
    images_loaded: Vec<Option<Handle>>,
    /// storing an optionally active article page
    active_article: Option<usize>,
    /// contains the sources fetched from newsapi, used for filtering requests by source
    source_data: Option<Result<NewsAPISourcesSuccess, String>>,
    /// map from source id to enabled state
    enabled_sources: HashMap<String, bool>,
    /// show the source page
    source_page: bool,
    /// stores the content of the source filter box in the source page
    source_filter: String,
}

#[derive(Debug, Clone)]
pub enum MainPageMessage {
    SearchBarOnInput(String),
    SourceFilterOnInput(String),
    SearchSubmit,
    SearchComplete(Result<NewsAPIArticlesSuccess, String>),
    SourcesFetched(Result<NewsAPISourcesSuccess, String>),
    SourceToggled(String, bool),
    // Handle is a reference to bytes, doesn't own the data
    ImageLoaded(Option<(usize, Handle)>),
    ActiveArticle(Option<usize>),
    ToggleSourcePage,
    DisableAllSources,
    BackToApiKeyPage,
}

pub const SOURCE_FILTER_ID: &str = "source_filter_input";

/// Render a compact error view showing a message and a Back to API key page button.
///
/// Parameters:
/// - error: the error text to display
///
/// Returns:
/// - an Element containing the error message and a button that navigates back to the API key page
pub fn error_element(error: &str) -> Element<'_, Message> {
    container(
        column![
            text(error).color(color!(0xff0000)).size(32),
            text("This application depends on NewsAPI, the error may be happening on their end."),
            button("Back to API key page")
                .style(button_style)
                .on_press(Message::MainPage(MainPageMessage::BackToApiKeyPage))
        ]
        .spacing(15),
    )
    .padding(15)
    .into()
}

/// Top bar containing the search input and buttons.
///
/// Parameters:
/// - search_query: current text in the search input
/// - n_sources: number shown on the sources button
///
/// Returns:
/// - an Element containing the search field, search submit button, and a sources toggle/tooltip button
fn top_bar(search_query: &str, n_sources: usize) -> Element<'_, Message> {
    use MainPageMessage::*;
    use Message::MainPage as M;

    row![
        text_input("Search for articles", search_query)
            .on_input(|s| M(SearchBarOnInput(s)))
            .on_submit(M(SearchSubmit))
            .id(SEARCH_BAR_ID) // id for focus task
            .style(text_input_style)
            .width(Length::Fill)
            .size(24),
        button(svg(iced::advanced::svg::Handle::from_memory(SEARCH_ICON)))
            .on_press(M(SearchSubmit))
            .padding(10)
            .width(48)
            .height(Length::Fill)
            .style(button_style),
        tooltip(
            mouse_area(
                button(row![
                    svg(iced::advanced::svg::Handle::from_memory(LIST_ICON)),
                    text(n_sources)
                ])
                .on_press(M(ToggleSourcePage))
                .padding(10)
                .width(64)
                .height(Length::Fill)
                .style(button_style),
            )
            .on_right_press(M(DisableAllSources)),
            container(text("Sources (right click to reset)"))
                .padding(5)
                .style(|theme: &Theme| {
                    container::Style {
                        background: Some(Background::Color(color!(0xeeeeff))),
                        border: Border::default().color(theme.palette().primary).rounded(5),
                        ..Default::default()
                    }
                }),
            tooltip::Position::Bottom,
        )
    ]
    .height(Length::Fixed(72.0))
    .spacing(5)
    .padding(15)
    .into()
}

impl MainPage {
    /// Create a new MainPage with a configured HTTP client and default state.
    ///
    /// Parameters:
    /// - token: the NewsAPI API key used to set the X-Api-Key request header
    ///
    /// Returns:
    /// - Ok(MainPage) with a reqwest client that has the API key header and a default user agent, plus default UI state
    /// - Err(NewsAPIError) if the header value is invalid or the HTTP client cannot be built
    pub fn new(token: &str) -> Result<Self, NewsAPIError> {
        let mut headers = HeaderMap::new();
        headers.insert("X-Api-Key", HeaderValue::from_str(token)?);

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
            source_filter: String::new(),
        })
    }
}

impl Page for MainPage {
    fn view(&self, size: (f32, f32)) -> Element<'_, Message> {
        let w = size.0;
        let mut article_chunks = (w / 400.0).floor();
        if article_chunks < 1.0 {
            article_chunks = 1.0;
        }

        let mut source_chunks = (w / 300.0).floor();
        if source_chunks < 1.0 {
            source_chunks = 1.0;
        }

        let article_chunks = article_chunks as usize;
        let source_chunks = source_chunks as usize;

        Stack::with_capacity(3) // allocate max
            // bottom layer
            // has top bar and article card list
            .push(
                Column::with_capacity(2) // allocate max
                    .push(top_bar(
                        &self.search_query,
                        self.enabled_sources.values().filter(|v| **v).count(),
                    ))
                    .push_maybe(article_cards(
                        self.search_result.as_ref(),
                        article_chunks,
                        &self.images_loaded,
                    )),
            )
            // detailed article page
            .push_maybe(article_page(
                self.active_article.as_ref(),
                self.search_result.as_ref(),
                &self.images_loaded,
            ))
            // detailed source page
            .push_maybe(source_page(
                self.source_page,
                self.source_data.as_ref(),
                &self.enabled_sources,
                source_chunks,
                &self.source_filter,
            ))
            .into()
    }

    fn update(&mut self, message: Message) -> Action {
        use MainPageMessage::*;
        use Message::MainPage as M;

        // handle escape key
        if let Message::Escape = message {
            if self.active_article.is_some() {
                self.active_article = None;
            }

            if self.source_page {
                self.source_page = false;
            }

            return Action::Task(focus(SEARCH_BAR_ID));
        }

        if let Message::MainPage(message) = message {
            match message {
                SearchBarOnInput(s) => self.search_query = s,
                SourceFilterOnInput(s) => self.source_filter = s,
                SearchSubmit => {
                    // need to pass these into the async block
                    // client is a shallow clone
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
                            .map_err(|e| {
                                eprintln!("{e:#?}");
                                e.to_string()
                            })
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
                            self.enabled_sources.insert(s.id.clone(), false);
                        }
                    }

                    self.source_data = Some(v);
                }
                // Toggle specific source
                SourceToggled(id, state) => {
                    self.enabled_sources.insert(id, state);
                    // refocus input box
                    return Action::Task(focus(SOURCE_FILTER_ID));
                }
                // Reset source filter
                DisableAllSources => {
                    for i in self.enabled_sources.values_mut() {
                        *i = false;
                    }
                }
                // Toggle the source filter page
                ToggleSourcePage => {
                    self.source_page = !self.source_page;
                    if self.source_page {
                        return Action::Task(focus(SOURCE_FILTER_ID));
                    }

                    self.source_filter = String::new();
                    return Action::Task(focus(SEARCH_BAR_ID));
                }
                ImageLoaded(data) => {
                    if let Some((i, handle)) = data
                        && i < self.images_loaded.len()
                    {
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

/// Create a Task that loads an article image and sends an ImageLoaded message.
///
/// Parameters:
/// - input: a tuple of (index, article)
///
/// Returns:
/// - a Task that, if the article has an image URL, fetches the image bytes, converts them to a Handle and dispatches Message::MainPage(MainPageMessage::ImageLoaded(Some((index, Handle)))) on success; logs errors and dispatches None on failure. If the article has no image URL returns Task::none().
fn image_task(input: (usize, &Article)) -> Task<Message> {
    let (index, article) = input;

    match &article.url_to_image {
        Some(url) => {
            let url = url.clone();

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

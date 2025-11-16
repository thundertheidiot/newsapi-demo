use crate::fetch_top;
use crate::ui::style::SUBMIT_ICON;
use crate::ui::style::text_input_style;
use iced::Length;
use iced::widget::svg;
use iced::widget::svg::Handle;
use std::env::var;

use crate::newsapi::NewsAPIError;
use crate::ui::Action;
use crate::ui::Message;
use crate::ui::Page;
use crate::ui::SEARCH_BAR_ID;
use crate::ui::TOKEN_INPUT_ID;
use crate::ui::main_page::MainPage;
use crate::ui::style::button_style;
use iced::Element;
use iced::Task;
use iced::futures::task;
use iced::widget::Space;
use iced::widget::button;
use iced::widget::text_input::focus;
use iced::widget::{column, row, text, text_input};

#[derive(Default)]
pub struct TokenPage {
    token: String,
    error: Option<NewsAPIError>,
}

#[derive(Debug, Clone)]
pub enum TokenPageMessage {
    OnInput(String),
    Submit,
}

impl TokenPage {
    pub fn new() -> Self {
        // empty string by default
        let token = var("NEWS_API_TOKEN").unwrap_or_default();

        Self { token, error: None }
    }
}

impl Page for TokenPage {
    fn view(&self, _size: (f32, f32)) -> Element<'_, Message> {
        use crate::ui::Message::TokenPage as T;
        use TokenPageMessage::*;

        let input = row![
            text_input("NewsAPI Token", &self.token)
                .on_input(|s| T(OnInput(s)))
                .on_submit(T(Submit))
                .id(TOKEN_INPUT_ID)
                .style(text_input_style)
                .width(Length::FillPortion(19))
                .size(24),
            button(svg(Handle::from_memory(SUBMIT_ICON)))
                .on_press(T(Submit))
                .padding(10)
                .width(Length::FillPortion(1))
                .height(Length::Fill)
                .style(button_style),
        ]
        .spacing(5)
        .height(Length::Fixed(72.0))
        .padding(15);

        let error_text: Element<'_, Message> = match &self.error {
            None => Space::new(0, 0).into(),
            // TODO good error
            Some(e) => text(e.to_string()).into(),
        };

        column![input, error_text].into()
    }

    fn update(&mut self, msg: Message) -> Action {
        if let Message::TokenPage(message) = msg {
            use TokenPageMessage::*;
            match message {
                OnInput(input) => {
                    self.token = input;
                }
                Submit => match MainPage::new(self.token.clone()) {
                    Ok(page) => {
                        // need to pass this into an async move block
                        let client = page.client.clone();

                        return Action::SwitchPage((
                            Box::new(page),
                            Task::batch(vec![
                                focus(SEARCH_BAR_ID),
                                // begin fetching the top headlines right away
                                Task::perform(
                                    async move { fetch_top(&client).await.map_err(|e| e.to_string()) },
                                    |v| {
                                        Message::MainPage(
                                            crate::ui::main_page::MainPageMessage::SearchComplete(
                                                v,
                                            ),
                                        )
                                    },
                                ),
                            ]),
                        ));
                    }
                    Err(e) => {
                        self.error = Some(e);
                    }
                },
            }
        }

        Action::None
    }
}

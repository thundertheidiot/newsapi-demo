use std::env::var;

use crate::newsapi::NewsAPIError;
use crate::ui::Action;
use crate::ui::Message;
use crate::ui::Page;
use crate::ui::TOKEN_INPUT_ID;
use crate::ui::main_page::MainPage;
use iced::Element;
use iced::widget::Space;
use iced::widget::button;
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
        let token = match var("NEWS_API_TOKEN") {
            Ok(var) => var,
            Err(_) => String::new(),
        };

        Self {
            token: token,
            error: None,
        }
    }
}

impl Page for TokenPage {
    fn view(&self) -> Element<'_, Message> {
        use crate::ui::Message::TokenPage as T;
        use TokenPageMessage::*;

        let input = row![
            text_input("NewsAPI Token", &self.token)
                .on_input(|s| T(OnInput(s)))
                .on_submit(T(Submit))
                .id(TOKEN_INPUT_ID)
                .size(24),
            button("Submit").on_press(T(Submit)).padding(10),
        ]
        .spacing(5)
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
                    Ok(page) => return Action::SwitchPage(Box::new(page)),
                    Err(e) => {
                        self.error = Some(e);
                    }
                },
            }
        }

        Action::None
    }
}

use crate::ui::main_page::MainPageMessage;
use crate::ui::token_page::TokenPage;
use crate::ui::token_page::TokenPageMessage;
use iced::Element;
use iced::Event;
use iced::Subscription;
use iced::Task;
use iced::event;
use iced::widget::text_input::focus;

mod article;
mod main_page;
mod token_page;

trait Page {
    fn update(&mut self, message: Message) -> Action;
    fn view(&self) -> Element<'_, Message>;
}

#[derive(Debug, Clone)]
pub enum Message {
    TokenPage(TokenPageMessage),
    MainPage(MainPageMessage),
    OpenLink(String),
    Event(Event),
}

pub enum Action {
    SwitchPage(Box<dyn Page>),
    Task(Task<Message>),
    None,
}

pub struct App {
    page: Box<dyn Page>,
}

pub const TOKEN_INPUT_ID: &'static str = "token_input_box";

impl App {
    pub fn new() -> (Self, Task<Message>) {
        (
            Self {
                page: Box::new(TokenPage::new()),
            },
            // stating task
            focus(TOKEN_INPUT_ID),
        )
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        use Action::*;

        if let Message::OpenLink(link) = message {
            if let Err(error) = open::that(&link) {
                eprintln!("Error opening link: {error:?}");
            }
            return iced::Task::none();
        }

        match self.page.update(message) {
            SwitchPage(page) => {
                self.page = page;
                iced::Task::none()
            }
            Task(task) => task,
            None => iced::Task::none(),
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        self.page.view()
    }

    pub fn subscription(&self) -> Subscription<Message> {
        event::listen().map(Message::Event)
    }
}

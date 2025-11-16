use crate::DEFAULT_SIZE;
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
mod source;
mod style;
mod token_page;

pub trait Page {
    fn update(&mut self, message: Message) -> Action;
    fn view(&self, size: (f32, f32)) -> Element<'_, Message>;
}

#[derive(Debug, Clone)]
pub enum Message {
    TokenPage(TokenPageMessage),
    MainPage(MainPageMessage),
    OpenLink(String),
    Event(Event),
    NoOp,
}

pub enum Action {
    SwitchPage((Box<dyn Page>, Task<Message>)),
    Task(Task<Message>),
    None,
}

pub struct App {
    page: Box<dyn Page>,
    window_size: (f32, f32),
}

pub const TOKEN_INPUT_ID: &str = "token_input_box";
pub const SEARCH_BAR_ID: &str = "search_box";

impl App {
    pub fn new() -> (Self, Task<Message>) {
        (
            Self {
                page: Box::new(TokenPage::new()),
                window_size: (DEFAULT_SIZE.0, DEFAULT_SIZE.1),
            },
            // starting task
            focus(TOKEN_INPUT_ID),
        )
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        use Action::*;

        if let Message::Event(event) = message {
            match event {
                iced::Event::Window(iced::window::Event::Resized(size)) => {
                    self.window_size = (size.width, size.height);
                }
                _ => (),
            }

            return iced::Task::none();
        }

        if let Message::OpenLink(link) = message {
            if let Err(error) = open::that(&link) {
                eprintln!("Error opening link: {error:?}");
            }
            return iced::Task::none();
        }

        match self.page.update(message) {
            SwitchPage((page, task)) => {
                self.page = page;
                task
            }
            Task(task) => task,
            None => iced::Task::none(),
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        self.page.view(self.window_size)
    }

    pub fn subscription(&self) -> Subscription<Message> {
        event::listen().map(Message::Event)
    }
}

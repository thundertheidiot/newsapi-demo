use crate::ui::main_page::MainPageMessage;
use crate::ui::token_page::TokenPage;
use crate::ui::token_page::TokenPageMessage;
use iced::Element;
use iced::Task;

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
}

pub enum Action {
    SwitchPage(Box<dyn Page>),
    Task(Task<Message>),
    None,
}

pub struct App {
    page: Box<dyn Page>,
}

impl App {
    pub fn new() -> (Self, Task<Message>) {
        (
            Self {
                page: Box::new(TokenPage::new()),
            },
            Task::none(),
        )
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        use Action::*;

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
}

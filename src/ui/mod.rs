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
pub enum ArticleMessage {
    Click(usize),
    ImageLoaded(usize),
}

#[derive(Debug, Clone)]
pub enum Message {
    TokenPage(TokenPageMessage),
    MainPage(MainPageMessage),
    Article(ArticleMessage),
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

// #[derive(Default)]
// pub struct NewsAPI {
//     value: i32,
//     search_query: String,
//     // value: Option<EverythingResponse>,
// }

// #[derive(Debug, Clone)]
// pub enum Message {
//     Increase,
//     Decrease,
//     SubmitSearch,
//     OnInput(String),
// }

// impl NewsAPI {
//     pub fn view(&self) -> Element<'_, Message> {
//         column![
//             text_input("Search", &self.search_query)
//                 .on_input(|s| Message::OnInput(s))
//                 .on_submit(Message::SubmitSearch),
//             text(self.value).size(24),
//             row![
//                 button("Increase").on_press(Message::Increase).padding(50),
//                 vertical_rule(10),
//                 button("Decrease").on_press(Message::Decrease).padding(50),
//             ],
//         ]
//         .into()
//     }

//     pub fn update(&mut self, message: Message) {
//         match message {
//             Message::Increase => self.value += 1,
//             Message::Decrease => self.value -= 1,
//             Message::SubmitSearch => {
//                 println!("ligma");
//             }
//             Message::OnInput(input) => {
//                 self.search_query = input;
//             }
//         }
//     }
// }

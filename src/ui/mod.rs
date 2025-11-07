use iced::Element;

#[derive(Default)]
pub struct NewsAPI {
    // value: Option<EverythingResponse>,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {}

impl NewsAPI {
    pub fn view(&self) -> Element<'_, Message> {
        "hello world".into()
    }

    pub fn update(&mut self, _message: Message) {}
}

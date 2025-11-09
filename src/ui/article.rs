use crate::newsapi::article::Article;
use crate::ui::Message;
use crate::ui::main_page::MainPageMessage;
use iced::widget::Button;
use iced::widget::button::{Status, Style};
use iced::widget::text;
use iced::{Element, widget::column, widget::container};
use iced::{Length, Theme};

pub fn article_to_card<'a>(index: usize, article: &'a Article) -> Element<'a, Message> {
    let description = match &article.description {
        Some(v) => v.as_str(),
        None => "placeholder",
    };

    // println!("{:#?}", article.url_to_image);

    Button::new(
        container(
            column![
                text(&article.title).size(24),
                text(description.to_string()),
                text(index)
            ]
            .spacing(5),
        )
        .width(Length::FillPortion(1))
        .max_height(200),
    )
    .on_press(Message::MainPage(MainPageMessage::ActiveArticle(index)))
    .into()
}

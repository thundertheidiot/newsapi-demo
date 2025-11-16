use crate::newsapi::source::Source;
use crate::ui::Message;
use crate::ui::main_page::MainPageMessage;
use iced::widget::{column, container, mouse_area, row, text, toggler};
use iced::{Color, Element};

pub fn source_toggle<'a>(source: &'a Source, is_enabled: &bool) -> Element<'a, Message> {
    column![
        row![
            text(&source.name).size(24),
            toggler(*is_enabled)
                .on_toggle(|state| Message::MainPage(MainPageMessage::SourceToggled(
                    source.id.to_owned(), // :(
                    state
                )))
                .size(24)
        ],
        text(&source.description),
        mouse_area(container(
            text(&source.url).color(Color::from_rgb(0.0, 0.5, 0.7))
        ))
        .on_press(Message::OpenLink(source.url.clone()))
    ]
    .into()
}

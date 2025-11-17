use crate::newsapi::source::Source;
use crate::ui::Message;
use crate::ui::main_page::MainPageMessage;
use iced::Background;
use iced::Border;
use iced::Theme;
use iced::color;
use iced::widget::text::Shaping::Advanced;
use iced::widget::{column, container, mouse_area, row, text, toggler, tooltip};
use iced::{Color, Element, Length};

pub fn source_toggle<'a>(source: &'a Source, is_enabled: &bool) -> Element<'a, Message> {
    tooltip(
        column![
            row![
                mouse_area(text(&source.name).size(24))
                    .on_press(Message::MainPage(MainPageMessage::SourceToggled(
                        source.id.to_owned(),
                        !*is_enabled
                    )))
                    .interaction(iced::mouse::Interaction::Pointer),
                toggler(*is_enabled)
                    .on_toggle(|state| Message::MainPage(MainPageMessage::SourceToggled(
                        source.id.to_owned(), // :(
                        state
                    )))
                    .size(24)
            ],
            mouse_area(container(
                text(&source.url)
                    .color(Color::from_rgb(0.0, 0.5, 0.7))
                    .shaping(Advanced)
            ))
            .interaction(iced::mouse::Interaction::Pointer)
            .on_press(Message::OpenLink(source.url.clone()))
        ]
        .width(Length::Fill),
        container(text(&source.description).shaping(Advanced))
            .padding(5)
            .max_width(500.0)
            .style(|theme: &Theme| container::Style {
                background: Some(Background::Color(color!(0xeeeeff))),
                border: Border::default().color(theme.palette().primary).rounded(5),
                ..Default::default()
            }),
        tooltip::Position::Bottom,
    )
    .into()
}

use crate::newsapi::NewsAPISourcesSuccess;
use crate::newsapi::source::Source;
use crate::ui::Message;
use crate::ui::main_page::MainPageMessage;
use crate::ui::main_page::SOURCE_FILTER_ID;
use crate::ui::main_page::error_element;
use crate::ui::style::CLOSE_ICON;
use crate::ui::style::close_button_style;
use crate::ui::style::text_input_style;
use iced::Alignment;
use iced::Background;
use iced::Border;
use iced::Theme;
use iced::color;
use iced::widget::Column;
use iced::widget::Row;
use iced::widget::button;
use iced::widget::horizontal_rule;
use iced::widget::scrollable;
use iced::widget::svg;
use iced::widget::text::Shaping::Advanced;
use iced::widget::text_input;
use iced::widget::{column, container, mouse_area, row, text, toggler, tooltip};
use iced::{Color, Element, Length};
use std::collections::HashMap;

/// Build a UI element for a Source: displays name, URL and a description tooltip, with a toggle control.
///
/// Parameters:
/// - `source`: source data (id, name, url, description) used for display and in emitted messages.
/// - `is_enabled`: current enabled state (controls the toggler's state).
///
/// Returns:
/// - `Element<'_, Message>` — a row with the source name and a toggler; clicking the name or toggler sends
///   `Message::MainPage(MainPageMessage::SourceToggled(source.id.clone(), new_state))`, and clicking the URL
///   sends `Message::OpenLink(source.url.clone())`. A tooltip with `source.description` is shown below.
pub fn source_toggle(source: &Source, is_enabled: bool) -> Element<'_, Message> {
    tooltip(
        column![
            row![
                mouse_area(text(&source.name).size(24))
                    .on_press(Message::MainPage(MainPageMessage::SourceToggled(
                        source.id.clone(),
                        !is_enabled
                    )))
                    .interaction(iced::mouse::Interaction::Pointer),
                toggler(is_enabled)
                    .on_toggle(|state| Message::MainPage(MainPageMessage::SourceToggled(
                        source.id.clone(), // :(
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

/// Render the source selection menu when `source_page` is true.
///
/// Parameters:
/// - `source_page`: whether the source page should be shown. If false, returns None.
/// - `source_data`: source list or an error; when Some(Ok(data)) builds the selectable source list, when Some(Err(e)) returns an error element, when None returns None.
/// - `enabled_sources`: map of source.id -> enabled state (used to set each toggle; missing keys are treated as false).
/// - `source_chunks`: number of source items per row when laying out the list.
/// - `source_filter`: filter text applied to source name/description/id; the code matches the filter against a lowercased haystack, so provide a lowercased filter for expected results.
///
/// Returns:
/// - `Some(Element<'a, Message>)` when `source_page` is true and `source_data` is Some(...): a page with a filter input and toggles (or an error text).
/// - `None` when `source_page` is false or `source_data` is None.
pub fn source_page<'a>(
    source_page: bool,
    source_data: Option<&'a Result<NewsAPISourcesSuccess, String>>,
    enabled_sources: &'a HashMap<String, bool>,
    source_chunks: usize,
    source_filter: &'a str,
) -> Option<Element<'a, Message>> {
    use MainPageMessage::*;
    use Message::MainPage as M;

    if !source_page {
        return None;
    }

    match source_data {
        Some(Ok(data)) => Some(
            mouse_area(
                container(
                    mouse_area(
                        container(column![
                            row![
                                text_input("Filter sources", source_filter)
                                    .style(text_input_style)
                                    .width(Length::Fill)
                                    .size(24)
                                    .on_input(|s| M(SourceFilterOnInput(s)))
                                    .on_submit(M(ToggleSourcePage))
                                    .id(SOURCE_FILTER_ID),
                                button(
                                    svg(svg::Handle::from_memory(CLOSE_ICON)).height(Length::Fill)
                                )
                                .width(48)
                                .height(Length::Fill)
                                .style(close_button_style)
                                .on_press(M(ToggleSourcePage)),
                            ]
                            .height(48)
                            .padding(5)
                            .spacing(5),
                            horizontal_rule(6),
                            {
                                // basic filter, this does mean only lowercase works
                                let haystack = |s: &Source| {
                                    format!(
                                        "{} {} {}",
                                        s.name.to_lowercase(),
                                        s.description.to_lowercase(),
                                        s.id.to_lowercase()
                                    )
                                };

                                scrollable(Column::with_children(
                                    data.sources
                                        .iter()
                                        .filter(|s| haystack(s).contains(source_filter))
                                        .collect::<Vec<&Source>>()
                                        .chunks(source_chunks)
                                        .map(|chunk| {
                                            Into::<Element<'_, Message>>::into(
                                                Row::with_children(chunk.iter().map(|source| {
                                                    source_toggle(
                                                        source,
                                                        *enabled_sources
                                                            .get(&source.id)
                                                            .unwrap_or(&false),
                                                    )
                                                }))
                                                .spacing(15),
                                            )
                                        }),
                                ))
                                .spacing(5)
                            }
                        ])
                        .padding([10, 10]) // top/bottom, left/right
                        .width(Length::Fill)
                        .style(|theme| container::Style {
                            background: Some(Background::Color(theme.palette().background)),
                            text_color: Some(theme.palette().text),
                            border: Border::default()
                                .color(theme.palette().primary)
                                .rounded(10)
                                .width(2),
                            ..Default::default()
                        }),
                    )
                    .on_press(Message::NoOp),
                )
                .padding(40)
                .width(Length::Fill)
                .height(Length::Fill)
                .center(Length::Fill)
                .align_x(Alignment::Center)
                .align_y(Alignment::Center)
                .style(|_theme| container::Style {
                    background: None,
                    ..Default::default()
                }),
            )
            .interaction(iced::mouse::Interaction::Idle)
            .on_right_press(M(ToggleSourcePage))
            .on_press(M(ToggleSourcePage))
            .into(),
        ),
        Some(Err(error)) => Some(error_element(error)),
        None => None,
    }
}

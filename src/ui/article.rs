use iced::Alignment;
use iced::Background;
use iced::Border;
use iced::Color;
use iced::Font;
use iced::Length::Shrink;
use iced::Shadow;
use iced::Vector;
use iced::advanced::image::Bytes;
use iced::mouse;
use iced::widget::Column;
use iced::widget::Image;
use iced::widget::Space;
use iced::widget::button;
use iced::widget::horizontal_rule;
use iced::widget::image::Handle;
use iced::widget::mouse_area;
use iced::widget::scrollable;
use iced::widget::svg;
use sha2::Digest;
use sha2::Sha256;
use std::env::temp_dir;
use std::fs::create_dir;
use std::path::PathBuf;

use crate::newsapi::NewsAPIError;
use crate::newsapi::article::Article;
use crate::ui::Message;
use crate::ui::main_page::MainPageMessage;
use crate::ui::style::CLOSE_ICON;
use crate::ui::style::button_style;
use crate::ui::style::card_style;
use crate::ui::style::close_button_style;
use iced::widget::Button;
use iced::widget::button::Status;
use iced::widget::text;
use iced::{Element, widget::container};
use iced::{Length, Theme};

pub fn article_to_card<'a>(
    index: usize,
    article: &'a Article,
    image: &Option<Handle>,
) -> Element<'a, Message> {
    let content: Column<'_, Message> = Column::with_capacity(2)
        .push(
            text(&article.title)
                .size(18)
                .width(Length::Fill)
                .style(|_theme| text::Style {
                    color: Some(Color::from_rgb(0.1, 0.1, 0.1)),
                }),
        )
        .push_maybe(image.as_ref().map(Image::new));

    Button::new(
        container(content.spacing(5)).width(Length::FillPortion(1)), // .max_height(200),
    )
    .on_press(Message::MainPage(MainPageMessage::ActiveArticle(Some(
        index,
    ))))
    .width(Length::Fill)
    .height(300)
    .style(card_style)
    .into()
}

pub fn article_view<'a>(article: &'a Article, image: &Option<Handle>) -> Element<'a, Message> {
    mouse_area(
        container(
            Column::<Message, Theme>::with_capacity(3)
                .push(
                    container(
                        button(svg(svg::Handle::from_memory(CLOSE_ICON)))
                            .width(Length::Fixed(48.0))
                            .style(close_button_style)
                            .on_press(Message::MainPage(MainPageMessage::ActiveArticle(None))),
                    )
                    .align_right(Length::Fill)
                    .padding(10),
                )
                .push(horizontal_rule(6))
                .push(
                    scrollable(
                        Column::<Message, Theme>::with_capacity(9)
                            .push(text(&article.title).size(44))
                            .push_maybe(
                                image.as_ref().map(|image| Image::new(image).height(Shrink)),
                            )
                            .push_maybe(match (&article.author, &article.source.name) {
                                (Some(author), Some(source)) => {
                                    Some(text(format!("{author} - {source}")).size(16))
                                }
                                (None, Some(source)) => Some(text(source.to_string()).size(16)),
                                _ => None,
                            })
                            .push(horizontal_rule(6))
                            .push_maybe(
                                article
                                    .description
                                    .as_ref()
                                    .map(|description| text(description).size(32)),
                            )
                            .push_maybe(
                                article
                                    .content
                                    .as_ref()
                                    .map(|content| text(content).size(20)),
                            )
                            .push_maybe(match (&article.description, &article.content) {
                                (Some(_), _) | (_, Some(_)) => Some(horizontal_rule(6)),
                                _ => None,
                            })
                            .push_maybe(article.url.as_ref().map(|url| {
                                container(
                                    button("Read full article")
                                        .on_press(Message::OpenLink(url.clone()))
                                        .style(button_style)
                                        .width(Length::Fill),
                                )
                                .padding(10)
                            }))
                            .push(Space::with_height(3)),
                    )
                    .spacing(5),
                ),
        )
        .padding([10, 10]) // top/bottom, left/right
        .width(Length::Fill)
        .max_width(1500)
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
    .on_press(Message::NoOp)
    .on_right_press(Message::MainPage(MainPageMessage::ActiveArticle(None)))
    .interaction(mouse::Interaction::Idle)
    .into()
}

fn tmpdir() -> PathBuf {
    temp_dir().join("newsapi_demo")
}

fn url_to_path(url: &str) -> PathBuf {
    let mut hasher = Sha256::new();
    hasher.update(url);
    let hash = hasher.finalize();
    let hex = hex::encode(&hash[..8]);

    tmpdir().join(hex)
}

pub async fn get_image_from_url(url: &str) -> Result<Bytes, NewsAPIError> {
    match create_dir(tmpdir()) {
        Ok(()) => (),
        Err(e) => match e.kind() {
            std::io::ErrorKind::AlreadyExists => (),
            _ => return Err(NewsAPIError::IO(e)),
        },
    };

    let path = url_to_path(url);

    let bytes = match path.exists() {
        true => tokio::fs::read(path).await?.into(),
        false => {
            let bytes = reqwest::get(url).await?.bytes().await?;

            // Shallow clone, bytes does not own the data
            let bytes_clone = bytes.clone();

            // background task to write to disk
            tokio::task::spawn(async move {
                if let Err(e) = tokio::fs::write(path, bytes_clone).await {
                    eprintln!("Failed to cache image: {e:?}");
                }
            });

            bytes
        }
    };

    // very simple image data validation
    // if the format is invalid, this will return an ImageError, otherwise we discard the guessed format
    let _ = image::guess_format(&bytes)?;

    Ok(bytes)
}

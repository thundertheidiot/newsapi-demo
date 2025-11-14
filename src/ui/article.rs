use iced::Alignment;
use iced::Background;
use iced::Border;
use iced::Color;
use iced::Length::FillPortion;
use iced::Length::Shrink;
use iced::advanced::image::Bytes;
use iced::color;
use iced::widget::Column;
use iced::widget::Container;
use iced::widget::Image;
use iced::widget::button;
use iced::widget::image::Handle;
use sha2::Digest;
use sha2::Sha256;
use std::env::temp_dir;
use std::fs::{create_dir, write};
use std::hash::{DefaultHasher, Hash, Hasher};
use std::path::Path;
use std::path::PathBuf;
use std::task;

use crate::newsapi::NewsAPIError;
use crate::newsapi::article::Article;
use crate::ui::Message;
use crate::ui::main_page::MainPageMessage;
use iced::widget::button::{Status, Style};
use iced::widget::text;
use iced::widget::{Button, Space};
use iced::{Element, widget::column, widget::container};
use iced::{Length, Theme};

pub fn article_to_card<'a>(
    index: usize,
    article: &'a Article,
    image: &Option<Handle>,
) -> Element<'a, Message> {
    let content: Column<'_, Message> = Column::with_capacity(2)
        .push(text(&article.title).size(24))
        .push_maybe(match image {
            Some(handle) => Some(Image::new(handle)),
            None => None,
        });
    Button::new(
        container(content.spacing(5)).width(Length::FillPortion(1)), // .max_height(200),
    )
    .on_press(Message::MainPage(MainPageMessage::ActiveArticle(Some(
        index,
    ))))
    .width(Length::Fill)
    .height(300)
    .style(|theme, state| button::Style {
        background: match state {
            Status::Hovered => Some(Background::Color(Color::from_rgb(1.0, 1.0, 1.0))),
            _ => Some(Background::Color(Color::from_rgb(0.8, 0.8, 0.8))),
        },
        border: match state {
            Status::Hovered => Border::default()
                .color(theme.palette().primary)
                .rounded(10)
                .width(3),
            _ => Border::default()
                .color(theme.palette().primary)
                .rounded(10)
                .width(2),
        },
        ..Default::default()
    })
    .into()
}

pub fn article_view<'a>(article: &'a Article, image: &Option<Handle>) -> Element<'a, Message> {
    container(
        Column::<Message, Theme>::with_capacity(6)
            .push(text(&article.title).size(44))
            .push_maybe(match &article.url {
                Some(url) => Some(button("open").on_press(Message::OpenLink(url.clone()))),
                None => None,
            })
            .push_maybe(match &article.description {
                Some(description) => Some(text(description).size(32)),
                None => None,
            })
            .push_maybe(match (&article.author, &article.source.name) {
                (Some(author), Some(source)) => Some(text(format!("{author} - {source}")).size(16)),
                (None, Some(source)) => Some(text(format!("{source}")).size(16)),
                _ => None,
            })
            .push_maybe(match image {
                Some(handle) => Some(Image::new(handle).height(Shrink)),
                None => None,
            })
            .push_maybe(match &article.content {
                Some(content) => Some(text(content).size(20)),
                None => None,
            }),
    )
    .padding([10, 10]) // top/bottom, left/right
    .width(Length::Fill)
    .align_x(Alignment::Center)
    .align_y(Alignment::Center)
    .style(|theme| container::Style {
        background: Some(Background::Color(theme.palette().background)),
        text_color: Some(theme.palette().text),
        border: Border::default()
            .color(theme.palette().primary)
            .rounded(10)
            .width(2),
        ..Default::default()
    })
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

    Ok(match path.exists() {
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

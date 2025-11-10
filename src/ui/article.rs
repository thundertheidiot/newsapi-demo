use iced::widget::Image;
use std::env::temp_dir;
use std::fs::{create_dir, write};
use std::hash::{DefaultHasher, Hash, Hasher};
use std::path::Path;
use std::path::PathBuf;

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
    image_loaded: bool,
) -> Element<'a, Message> {
    let description = match &article.description {
        Some(v) => v.as_str(),
        None => "placeholder",
    };

    let image: Element<'_, Message> = match (&article.url_to_image, image_loaded) {
        (Some(url), true) => Image::new(url).into(),
        _ => Space::with_width(0).into(),
    };

    Button::new(
        container(
            column![
                text(&article.title).size(24),
                text(description),
                text(index),
                image,
            ]
            .spacing(5),
        )
        .width(Length::FillPortion(1))
        .max_height(200),
    )
    .on_press(Message::MainPage(MainPageMessage::ActiveArticle(index)))
    .into()
}

// create_dir(&dir);
fn tmpdir() -> PathBuf {
    temp_dir().join("newsapi_demo")
}

fn url_to_path(url: &str) -> PathBuf {
    let mut hasher = DefaultHasher::new();
    url.hash(&mut hasher);
    tmpdir().join(format!("{}", hasher.finish()))
}

pub async fn download_image(url: &str) -> Result<(), NewsAPIError> {
    create_dir(tmpdir())?;

    println!("download started");

    let bytes = reqwest::get(url).await?.bytes().await?;
    let _ = write(url_to_path(url), bytes)?;

    println!("finished");

    Ok(())
}

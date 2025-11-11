use iced::widget::Column;
use iced::widget::Image;
use iced::widget::image::Handle;
use sha2::Digest;
use sha2::Sha256;
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
    let content: Column<'_, Message> = Column::with_capacity(2)
        .push(text(&article.title).size(24))
        .push_maybe(match (image_loaded, &article.url_to_image) {
            (true, Some(url)) => {
                let path = url_to_path(&url);
                match std::fs::read(path) {
                    Ok(bytes) => {
                        let handle = Handle::from_bytes(bytes);
                        Some(Image::new(handle))
                    }
                    Err(e) => {
                        eprintln!("Error loading image data: {e:#?}");
                        None
                    }
                }
            }
            _ => None,
        });

    Button::new(
        container(content.spacing(5)).width(Length::FillPortion(1)), // .max_height(200),
    )
    .on_press(Message::MainPage(MainPageMessage::ActiveArticle(index)))
    .into()
}

// create_dir(&dir);
fn tmpdir() -> PathBuf {
    temp_dir().join("newsapi_demo")
}

fn url_to_path(url: &str) -> PathBuf {
    let mut hasher = Sha256::new();
    hasher.update(url);
    let hash = hasher.finalize();
    let hex = hex::encode(&hash[..8]);

    println!("{}", &hex);

    tmpdir().join(hex)
}

pub async fn download_image(url: &str) -> Result<(), NewsAPIError> {
    match create_dir(tmpdir()) {
        Ok(()) => (),
        Err(e) => match e.kind() {
            std::io::ErrorKind::AlreadyExists => (),
            _ => return Err(NewsAPIError::IO(e)),
        },
    };

    println!("{url} download started");

    let bytes = reqwest::get(url).await?.bytes().await?;

    let path = url_to_path(url);
    println!("{path:?}");

    let _ = write(path, bytes)?;

    println!("finished");

    Ok(())
}

use crate::newsapi::fetch_top;
use crate::ui::App;
use iced::Size;
use reqwest::header::HeaderMap;
use reqwest::header::HeaderValue;

use crate::newsapi::NewsAPIError;
use std::env;

mod newsapi;
mod ui;

pub const DEFAULT_SIZE: (f32, f32) = (800.0, 600.0);

fn main() -> iced::Result {
    iced::application("NewsAPI Demo", App::update, App::view)
        .subscription(App::subscription)
        .window(iced::window::Settings {
            size: Size::new(DEFAULT_SIZE.0, DEFAULT_SIZE.1),
            ..Default::default()
        })
        .font(include_bytes!(
            "../assets/Roboto-VariableFont_wdth,wght.ttf"
        ))
        .default_font(iced::Font::with_name("Roboto"))
        .run_with(App::new)
}

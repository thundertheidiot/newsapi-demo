use crate::ui::Message;
use iced::Background;
use iced::Border;
use iced::Color;
use iced::Element;
use iced::Gradient;
use iced::Length;
use iced::Shadow;
use iced::Theme;
use iced::Vector;
use iced::widget::button;
use iced::widget::button::Status;
use iced::widget::container;
use iced::widget::svg;
use iced::widget::text_input;
use std::f32::consts::FRAC_PI_3;
use std::f32::consts::FRAC_PI_4;

pub const SEARCH_ICON: &[u8] =
    include_bytes!("../../assets/material-icons/search_24dp_1F1F1F_FILL0_wght400_GRAD0_opsz24.svg");
pub const SUBMIT_ICON: &[u8] = include_bytes!(
    "../../assets/material-icons/prompt_suggestion_24dp_1F1F1F_FILL0_wght400_GRAD0_opsz24.svg"
);
pub const CLOSE_ICON: &[u8] =
    include_bytes!("../../assets/material-icons/close_24dp_1F1F1F_FILL0_wght400_GRAD0_opsz24.svg");
pub const LIST_ICON: &[u8] = include_bytes!(
    "../../assets/material-icons/format_list_bulleted_24dp_1F1F1F_FILL0_wght400_GRAD0_opsz24.svg"
);
pub const NO_IMAGE_ICON: &[u8] = include_bytes!(
    "../../assets/material-icons/hide_image_24dp_E3E3E3_FILL0_wght400_GRAD0_opsz24.svg"
);

pub fn no_image() -> Element<'static, Message> {
    container(
        svg(svg::Handle::from_memory(NO_IMAGE_ICON))
            .width(Length::Fill)
            .height(Length::Fill),
    )
    .center(Length::Fill)
    .style(|_theme| container::Style {
        background: Some(Background::Gradient(Gradient::Linear(
            iced::gradient::Linear::new(FRAC_PI_3)
                .add_stop(0.0, Color::from_rgb(0.90, 0.90, 0.97))
                .add_stop(1.0, Color::from_rgb(0.80, 0.85, 0.95)),
        ))),
        ..Default::default()
    })
    .into()
}

pub fn button_style(theme: &Theme, status: button::Status) -> button::Style {
    let palette = theme.palette();

    button::Style {
        background: Some(Background::Color(match status {
            Status::Hovered | Status::Pressed => Color::from_rgb(0.95, 0.95, 0.95),
            _ => Color::from_rgb(1.0, 1.0, 1.0),
        })),
        border: Border::default()
            .color(palette.primary)
            .rounded(8)
            .width(match status {
                Status::Hovered => 2,
                _ => 1,
            }),
        shadow: match status {
            Status::Hovered | Status::Pressed => Shadow {
                color: Color::from_rgb(0.0, 0.0, 0.0),
                offset: Vector::new(0.0, 2.0),
                blur_radius: 2.0,
            },
            _ => Shadow {
                color: Color::from_rgb(0.0, 0.0, 0.0),
                offset: Vector::new(0.0, 0.0),
                blur_radius: 2.0,
            },
        },
        ..Default::default()
    }
}

pub fn close_button_style(theme: &Theme, status: button::Status) -> button::Style {
    let palette = theme.palette();

    button::Style {
        background: Some(Background::Color(match status {
            Status::Hovered | Status::Pressed => Color::from_rgb(0.95, 0.55, 0.55),
            _ => Color::from_rgb(1.0, 1.0, 1.0),
        })),
        border: Border::default()
            .color(palette.primary)
            .rounded(8)
            .width(match status {
                Status::Hovered => 2,
                _ => 1,
            }),
        shadow: match status {
            Status::Hovered | Status::Pressed => Shadow {
                color: Color::from_rgb(0.0, 0.0, 0.0),
                offset: Vector::new(0.0, 2.0),
                blur_radius: 2.0,
            },
            _ => Shadow {
                color: Color::from_rgb(0.0, 0.0, 0.0),
                offset: Vector::new(0.0, 0.0),
                blur_radius: 2.0,
            },
        },
        ..Default::default()
    }
}

pub fn card_style(theme: &Theme, status: button::Status) -> button::Style {
    let palette = theme.palette();

    let (start, end) = match status {
        button::Status::Hovered => (
            Color::from_rgb(0.80, 0.80, 0.90),
            Color::from_rgb(0.55, 0.75, 0.85),
        ),
        button::Status::Pressed => (
            Color::from_rgb(0.85, 0.85, 0.95),
            Color::from_rgb(0.60, 0.80, 0.90),
        ),
        _ => (
            Color::from_rgb(0.90, 0.90, 1.00),
            Color::from_rgb(0.70, 0.85, 0.95),
        ),
    };

    button::Style {
        background: Some(Background::Gradient(Gradient::Linear(
            iced::gradient::Linear::new(FRAC_PI_4)
                .add_stop(0.0, start)
                .add_stop(1.0, end),
        ))),
        border: Border::default()
            .color(palette.primary)
            .rounded(12)
            .width(match status {
                Status::Hovered | Status::Pressed => 2,
                _ => 1,
            }),
        shadow: match status {
            Status::Hovered | Status::Pressed => Shadow {
                color: Color::from_rgb(0.0, 0.0, 0.2),
                offset: Vector::new(0.0, 3.0),
                blur_radius: 2.0,
            },
            _ => Shadow {
                color: Color::from_rgb(0.0, 0.0, 0.2),
                offset: Vector::new(0.0, 2.0),
                blur_radius: 2.0,
            },
        },
        ..Default::default()
    }
}

pub fn text_input_style(theme: &Theme, status: text_input::Status) -> text_input::Style {
    let palette = theme.palette();

    text_input::Style {
        border: Border::default().color(palette.primary).rounded(8).width(1),
        ..text_input::default(theme, status)
    }
}

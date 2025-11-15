use iced::Background;
use iced::Border;
use iced::Color;
use iced::Gradient;
use iced::Shadow;
use iced::Theme;
use iced::Vector;
use iced::widget::button;
use iced::widget::button::Status;
use iced::widget::text_input;

pub const SEARCH_ICON: &[u8] =
    include_bytes!("../../assets/material-icons/search_24dp_1F1F1F_FILL0_wght400_GRAD0_opsz24.svg");
pub const SUBMIT_ICON: &[u8] = include_bytes!(
    "../../assets/material-icons/prompt_suggestion_24dp_1F1F1F_FILL0_wght400_GRAD0_opsz24.svg"
);
pub const CLOSE_ICON: &[u8] =
    include_bytes!("../../assets/material-icons/close_24dp_1F1F1F_FILL0_wght400_GRAD0_opsz24.svg");

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
            iced::gradient::Linear::new(-0.785398)
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

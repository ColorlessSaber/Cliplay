use iced::{
    Background,
    Border,
    Color,
    Shadow,
    Theme,
    widget::button::{Status, Style}
};

// styles when the button is active
pub fn btn_active_style(_: &Theme, status: Status) -> Style {
    match status {
        Status::Active | Status::Pressed => Style {
            background: Some(Background::Color(Color::from_rgb(0.2, 0.6, 0.2))),
            text_color: Color::WHITE,
            border: Border::default(),
            shadow: Shadow::default(),
            snap: true,
        },
        Status::Hovered => Style {
            background: Some(Background::Color(Color::from_rgb(0.3, 0.7, 0.3))),
            text_color: Color::WHITE,
            border: Border::default(),
            shadow: Shadow::default(),
            snap: true,
        },
        Status::Disabled => Style {
            background: Some(Background::Color(Color::from_rgb(0.5, 0.5, 0.5))),
            text_color: Color::from_rgb(0.7, 0.7, 0.7),
            border: Border::default(),
            shadow: Shadow::default(),
            snap: true,
        },
    }
}

// styles when the button is inactive
pub fn btn_inactive_style(_: &Theme, status: Status) -> Style {
    match status {
        Status::Active | Status::Pressed => Style {
            background: Some(Background::Color(Color::from_rgb(0.94, 0.49, 0.0))),
            text_color: Color::WHITE,
            border: Border::default(),
            shadow: Shadow::default(),
            snap: true,
        },
        Status::Hovered => Style {
            background: Some(Background::Color(Color::from_rgb(0.94, 0.67, 0.0))),
            text_color: Color::WHITE,
            border: Border::default(),
            shadow: Shadow::default(),
            snap: true,
        },
        Status::Disabled => Style {
            background: Some(Background::Color(Color::from_rgb(0.5, 0.5, 0.5))),
            text_color: Color::from_rgb(0.7, 0.7, 0.7),
            border: Border::default(),
            shadow: Shadow::default(),
            snap: true,
        },
    }
}
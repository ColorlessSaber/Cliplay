use iced::{
    Background,
    Border,
    Color,
    Shadow,
    Theme,
    widget::button::{Status, Style},
    border,
};


// style for the player buttons
pub fn player_control_button_style(_: &Theme, status: Status) -> Style {
    let custom_border = Border::default();
    let radius_value = 20.0;
    match status {
        Status::Active | Status::Pressed => Style {
            background: Some(Background::Color(Color::from_rgb(0.2, 0.6, 0.2))),
            text_color: Color::WHITE,
            border: custom_border.rounded(border::radius(radius_value)),
            shadow: Shadow::default(),
            snap: true,
        },
        Status::Hovered => Style {
            background: Some(Background::Color(Color::from_rgb(0.3, 0.7, 0.3))),
            text_color: Color::WHITE,
            border: custom_border.rounded(border::radius(radius_value)),
            shadow: Shadow::default(),
            snap: true,
        },
        Status::Disabled => Style {
            background: Some(Background::Color(Color::from_rgb(0.5, 0.5, 0.5))),
            text_color: Color::from_rgb(0.7, 0.7, 0.7),
            border: custom_border.rounded(border::radius(radius_value)),
            shadow: Shadow::default(),
            snap: true,
        },
    }
}

pub fn active_large_button_style(_: &Theme, status: Status) -> Style {
    let custom_border = Border::default();
    let radius_value = 5.0;
    match status {
        Status::Active | Status::Pressed => Style {
            background: Some(Background::Color(Color::from_rgb(0.2, 0.6, 0.2))),
            text_color: Color::WHITE,
            border: custom_border.rounded(border::radius(radius_value)),
            shadow: Shadow::default(),
            snap: true,
        },
        Status::Hovered => Style {
            background: Some(Background::Color(Color::from_rgb(0.3, 0.7, 0.3))),
            text_color: Color::WHITE,
            border: custom_border.rounded(border::radius(radius_value)),
            shadow: Shadow::default(),
            snap: true,
        },
        Status::Disabled => Style {
            background: Some(Background::Color(Color::from_rgb(0.5, 0.5, 0.5))),
            text_color: Color::from_rgb(0.7, 0.7, 0.7),
            border: custom_border.rounded(border::radius(radius_value)),
            shadow: Shadow::default(),
            snap: true,
        },
    }
}

pub fn inactive_large_button_style(_: &Theme, status: Status) -> Style {
    let custom_border = Border::default();
    let radius_value = 5.0;
    match status {
        Status::Active | Status::Pressed => Style {
            background: Some(Background::Color(Color::from_rgb(0.94, 0.49, 0.0))),
            text_color: Color::WHITE,
            border: custom_border.rounded(border::radius(radius_value)),
            shadow: Shadow::default(),
            snap: true,
        },
        Status::Hovered => Style {
            background: Some(Background::Color(Color::from_rgb(0.94, 0.67, 0.0))),
            text_color: Color::WHITE,
            border: custom_border.rounded(border::radius(radius_value)),
            shadow: Shadow::default(),
            snap: true,
        },
        Status::Disabled => Style {
            background: Some(Background::Color(Color::from_rgb(0.5, 0.5, 0.5))),
            text_color: Color::from_rgb(0.7, 0.7, 0.7),
            border: custom_border.rounded(border::radius(radius_value)),
            shadow: Shadow::default(),
            snap: true,
        },
    }
}

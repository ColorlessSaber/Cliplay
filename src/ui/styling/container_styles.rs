use iced::{
    Background,
    Border,
    Color,
    Theme,
    widget::container::Style
};


pub fn video_playing_style(_: &Theme) -> Style {
    Style {
        background: Some(Background::Color(Color::from_rgb(0.0, 0.0, 0.0))),
        text_color: Some(Color::BLACK),
        ..Style::default()
    }
}

pub fn splash_screen_style(_: &Theme) -> Style {
    Style {
        background: Some(Background::Color(Color::from_rgb(0.0, 0.0, 0.0))),
        text_color: Some(Color::WHITE),
        border: Border::default(),
        ..Style::default()
    }
}

pub fn control_bar_style(_: &Theme) -> Style {
    let custom_border = Border::default();
    Style {
        background: Some(Background::Color(Color::from_rgb(0.502, 0.502, 0.502))),
        text_color: Some(Color::BLACK),
        border: custom_border.width(2).color(Color::from_rgb(0.0, 0.0, 0.0)),
        ..Style::default()
    }
}

pub fn main_section_style(_: &Theme) -> Style {
    let custom_border = Border::default();
    Style {
        background: Some(Background::Color(Color::from_rgb(0.502, 0.502, 0.502))),
        text_color: Some(Color::BLACK),
        border: custom_border.width(2).color(Color::from_rgb(0.0, 0.0, 0.0)),
        ..Style::default()
    }
}
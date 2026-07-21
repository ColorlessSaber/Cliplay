use iced::{
    Background,
    border::radius,
    Theme,
    Color,
};
use iced_aw::widget::labeled_frame::Style;

pub fn settings_label_frame(_: &Theme) -> Style {
    Style {
        color: Background::Color(Color::BLACK),
        radius: radius(0.5),
    }
}
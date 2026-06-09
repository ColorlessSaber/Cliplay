use iced:: {
    Background,
    Border,
    Color,
    Theme,
    widget::slider::{Style, Status, Rail, Handle, HandleShape},
};

pub fn volume_slider_style(_: &Theme, status: Status) -> Style {
    match status {
        Status::Active | Status::Hovered | Status::Dragged => Style {
            rail: Rail {
                backgrounds: (
                    Background::Color(Color::from_rgb(0.0, 0.576, 0.941)), // slider color
                    Background::Color(Color::BLACK) // background of slider color
                ),
                width: 10.0,
                border: Border::default()
            },
            handle: Handle {
                shape: HandleShape::Rectangle {
                    width: 8,
                    border_radius: iced::border::radius(0.0)
                },
                background: Background::Color(Color::from_rgb(0.0, 0.0, 0.0)),
                border_width: 0.5,
                border_color: Color::WHITE,
            }
        }
    }
}
use iced::{
    Theme,
    Color,
    Background,
    Border,
    Shadow,
    widget::button::{Status, Style},
};

pub mod static_images {
    pub static MAIN_MENU_IMAGE: &str = "icons/main_menu.png";
    pub static PLAY_IMAGE: &str = "icons/play.png";
    pub static PAUSE_IMAGE: &str = "icons/pause.png";
    pub static FORWARD_IMAGE: &str = "icons/forward.png";
    pub static BACKWARD_IMAGE: &str = "icons/backward.png";
    pub static LOOP_IMAGE: &str = "icons/loop.png";
    pub static SHUFFLE_IMAGE: &str = "icons/shuffle.png";
    pub static VOLUME_IMAGE: &str = "icons/volume.png";
}

pub fn btn_style(_: &Theme, status: Status) -> Style {
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
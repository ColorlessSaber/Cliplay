use iced::{
    Theme,
    Color,
    Background,
    Border,
    Shadow,
    widget::button::{Status, Style},
};

pub mod static_images {
    pub static MAIN_MENU_CLOSED_IMAGE: &str = "icons/main_menu_closed.png";
    pub static MAIN_MENU_OPEN_IMAGE: &str = "icons/main_menu_open.png";
    pub static PLAY_IMAGE: &str = "icons/play.png";
    pub static PAUSE_IMAGE: &str = "icons/pause.png";
    pub static FORWARD_IMAGE: &str = "icons/forward.png";
    pub static BACKWARD_IMAGE: &str = "icons/backward.png";
    pub static LOOP_OFF_IMAGE: &str = "icons/loop_off.png";
    pub static LOOP_ONE_IMAGE: &str = "icons/loop_single.png";
    pub static LOOP_INFINITE_IMAGE: &str = "icons/loop_infinite.png";
    pub static SHUFFLE_IMAGE: &str = "icons/shuffle.png";
    pub static VOLUME_IMAGE: &str = "icons/volume.png";
}

// Help differentiate the state of the widget; IE, active, inactive, etc.
#[derive(Copy, Clone)]
pub enum StyleState {
    ActiveStyle,
    InactiveStyle,
}

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
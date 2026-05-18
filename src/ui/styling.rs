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

// The different style states for a given widget
#[derive(Debug, PartialEq)]
pub enum StyleState {
    ActiveStyle,
    InactiveStyle,
}

// Holds the different style states for each button
pub struct BtnStyle {
    pub loop_button: StyleState,
    pub shuffle_button: StyleState,
}

impl Default for BtnStyle {
    fn default() -> Self {
        Self {
            loop_button: StyleState::InactiveStyle,
            shuffle_button: StyleState::InactiveStyle,
        }
    }
}

impl BtnStyle {
    pub fn toggle_loop_style(&mut self) {
        match self.loop_button {
            StyleState::InactiveStyle => {
                self.loop_button = StyleState::ActiveStyle;
            }
            StyleState::ActiveStyle => {
                self.loop_button = StyleState::InactiveStyle;
            }
        }
    }
    
    pub fn toggle_shuffle_style(&mut self) {
        match self.shuffle_button {
            StyleState::InactiveStyle => {
                self.shuffle_button = StyleState::ActiveStyle;
            }
            StyleState::ActiveStyle => {
                self.shuffle_button = StyleState::InactiveStyle;
            }
        }
    }
}

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
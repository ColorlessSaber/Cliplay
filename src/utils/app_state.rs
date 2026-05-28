use crate::ui::{buttons::ButtonStruct};
use crate::utils::{
    playlist_manager::PlayListManager,
};
use iced_video_player::Video;

// Holds the state, or information, of the app that can be shared between different views
pub struct AppState {
    pub video: Option<Video>,
    pub btn_struct: ButtonStruct,
    pub playlist_manager: PlayListManager,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            video: None,
            btn_struct: ButtonStruct::default(),
            playlist_manager: PlayListManager::new()
        }
    }
}
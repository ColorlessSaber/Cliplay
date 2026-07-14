use crate::ui::{buttons::ButtonStruct};
use crate::utils::{
    playlist_manager::PlaylistManager,
    app_settings_struct::AppSettings,
};
use iced_video_player::Video;

// Holds the state, or information, of the app that can be shared between different views
pub struct AppState {
    pub video: Option<Video>,
    pub btn_struct: ButtonStruct,
    pub playlist_manager: PlaylistManager,
    pub settings: AppSettings,
}
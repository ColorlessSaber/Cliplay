use crate::ui::{dynamic_buttons::DynamicButtons};
use crate::utils::{
    playlist_manager::PlaylistManager,
    io_utils::app_settings_data_struct::AppSettings,
};
use iced_video_player::Video;

// Holds the state, or information, of the app that can be shared between different views
pub struct AppState {
    pub video: Option<Video>,
    pub btn_struct: DynamicButtons,
    pub playlist_manager: PlaylistManager,
    pub settings: AppSettings,
}
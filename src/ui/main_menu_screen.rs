use iced::{
    Element,
    Length,
    widget::{Button, Column, Image, Space},
};
use crate::ui::styling::btn_active_style;
use crate::ui::styling::static_images::{
    SELECT_SINGLE_VID_IMAGE,
    PLAYLISTS_IMAGE
};
use crate::utils::app_state::AppState;
use crate::utils::functions::load_video_file;

#[derive(Debug, Copy, Clone)]
pub enum MainMenuMessages {
    SelectVideo,
    PlaylistsMenu,
}

pub struct MainMenuScreen {}

impl MainMenuScreen {

    pub fn new() -> Self {
        Self {}
    }

    pub fn update(&self, message: MainMenuMessages, state: &mut AppState) {
        match message {
            MainMenuMessages::SelectVideo => {
                state.playlist_manager.load_playlist();
                let loop_entire_playlist = state.btn_struct.loop_button.is_state_set_to_loop_all();
                let video_file = state.playlist_manager.next_file_in_playlist(loop_entire_playlist);

                match video_file {
                    Some(video_file) => {
                        state.video = Some(load_video_file(&video_file));
                    }
                    None => {
                        println!("reach end of playlist")
                    }
                }

                state.btn_struct.main_menu_button.toggle_state(); // to switch to video view
            }
            MainMenuMessages::PlaylistsMenu => {
                println!("Playlists selected");
            }
        }
    }

    pub fn view<'a, 'b>(&'a self) -> Element<'b, MainMenuMessages>
    where 'a: 'b
    {
        Column::new()
            .spacing(10)
            .push(
                Button::new(Image::new(SELECT_SINGLE_VID_IMAGE).width(64).height(64))
                    .on_press(MainMenuMessages::SelectVideo)
                    .style(btn_active_style)
            )
            .push(
                Button::new(Image::new(PLAYLISTS_IMAGE).width(64).height(64))
                    .on_press(MainMenuMessages::PlaylistsMenu)
                    .style(btn_active_style)
            )
            .push(Space::new().height(Length::Fill))
            .into()
    }
}
use iced::{
    Element,
    Length,
    Task,
    Alignment,
    widget::{
        Button,
        button,
        Text,
        Column,
        Row,
        Image,
        image,
        Space,
        Container,
        scrollable,
        keyed_column,
        column,
    },
};
use crate::ui::styling::{
    active_large_button_style,
    container_styles::{
        main_section_style,
        playlist_entry_style,
    },
    icons::{
        DELETE_PLAYLIST_ICON,
        EDIT_PLAYLIST_ICON,
        NEW_PLAYLIST_ICON,
        PLAY_PLAYLIST_ICON,
        REMOVE_VIDEO_FILE_ICON,
        SAVE_ICON,
        SELECT_VIDEO_FILE_ICON},
};
use crate::utils::{
    app_state::AppState,
    save_utils::app_directory_path,
    functions::{
        currently_saved_playlists,
    }
};

#[derive(Debug, Clone)]
pub enum PlaylistMenuMessages {
    PlayPlaylist,
    NewPlaylist,
    EditPlaylist,
    DeletePlaylist,
    AddVideo,
    RemoveVideo,
    Save,
    Cancel,
}

#[derive(Debug, Clone)]
enum PlaylistMenuState {
    PlaylistList,
    NewPlaylist,
}

pub struct PlaylistMenuScreen {
    playlist_menu_state: PlaylistMenuState,
}

impl PlaylistMenuScreen {
    pub fn new() -> Self {
        Self {
            playlist_menu_state: PlaylistMenuState::PlaylistList,
        }
    }

    pub fn update(&mut self, message: PlaylistMenuMessages, state: &mut AppState) -> Task<PlaylistMenuMessages> {
        match message {
            PlaylistMenuMessages::PlayPlaylist => {
                /* TODO update with loading existing/new playlist
                state.playlist_manager.load_playlist();
                let video_file = state.playlist_manager.pull_first_file_from_playlist();

                match video_file {
                    Some(video_file) => {
                        state.video = Some(load_video_file(&video_file));
                    }
                    None => {}
                }

                state.btn_struct.main_menu_button.toggle_state(); // to switch to video view
                 */
                println!("Playing playlist");

                Task::none()
            }
            PlaylistMenuMessages::NewPlaylist => {
                self.playlist_menu_state = PlaylistMenuState::NewPlaylist;

                Task::none()
            }
            PlaylistMenuMessages::EditPlaylist => {
                println!("Editing playlist");

                Task::none()
            }
            PlaylistMenuMessages::DeletePlaylist => {
                println!("Deleting playlist");

                Task::none()
            }
            PlaylistMenuMessages::AddVideo => {
                println!("Adding video to playlist");

                Task::none()
            }
            PlaylistMenuMessages::RemoveVideo => {
                println!("Removing video from playlist");

                Task::none()
            }
            PlaylistMenuMessages::Save => {
                println!("Saving playlist");

                Task::none()
            }
            PlaylistMenuMessages::Cancel => {
                self.playlist_menu_state = PlaylistMenuState::PlaylistList;

                Task::none()
            }
        }
    }
    pub fn view(&self) -> Element<'_, PlaylistMenuMessages> {
        match self.playlist_menu_state {
            PlaylistMenuState::PlaylistList => {
                let playlists_found = currently_saved_playlists(app_directory_path());

                match playlists_found {
                    Ok(playlists) => {
                        // to reduce duplicate code, created a starting playlist column and then depending
                        // on the number of playlists found, generate the playlist column and then
                        // pass it out to be inserted into an Iced container.
                        let new_playlist_btn: Button<PlaylistMenuMessages> = button(image(NEW_PLAYLIST_ICON).width(64).height(64))
                            .on_press(PlaylistMenuMessages::NewPlaylist)
                            .style(active_large_button_style);

                        let starting_playlist_column = column![new_playlist_btn]
                            .spacing(10)
                            .width(Length::Fill);

                        let final_playlist_column = if playlists.is_empty() {
                            starting_playlist_column.push(Text::new("No playlists found"))
                        } else {
                            let list_of_playlists = keyed_column(
                                (0..=playlists.len()-1).map(|i|{
                                    (i, playlist_entry_layout(playlists.get(i).unwrap()))
                                }));

                            starting_playlist_column.push(scrollable(list_of_playlists).spacing(20))
                        };

                        Container::new(
                            final_playlist_column
                        )
                            .padding(10)
                            .height(Length::Fill)
                            .style(main_section_style)
                            .into()
                    }
                    Err(_) => {
                        Container::new(
                            Column::new()
                                .spacing(10)
                                .width(Length::Fill)
                                .push(
                                    Text::new("An error occurred while trying to process the playlist folder.")
                                )
                        )
                            .padding(10)
                            .height(Length::Fill)
                            .style(main_section_style)
                            .into()
                    }
                }
            }
            PlaylistMenuState::NewPlaylist => {
                Container::new(
                    Row::new()
                        .spacing(10)
                        .width(Length::Fill)
                        .push(
                            Button::new(Image::new(SELECT_VIDEO_FILE_ICON).width(64).height(64))
                                .on_press(PlaylistMenuMessages::AddVideo)
                                .style(active_large_button_style)
                        )
                        .push(
                            Button::new(Image::new(REMOVE_VIDEO_FILE_ICON).width(64).height(64))
                                .on_press(PlaylistMenuMessages::RemoveVideo)
                                .style(active_large_button_style)
                        )
                        .push(
                            Button::new(Image::new(SAVE_ICON).width(64).height(64))
                                .on_press(PlaylistMenuMessages::Save)
                                .style(active_large_button_style)
                        )
                        .push(
                            Button::new(Text::new("Cancel"))
                                .on_press(PlaylistMenuMessages::Cancel)
                                .style(active_large_button_style)
                        )
                )
                    .padding(10)
                    .height(Length::Fill)
                    .style(main_section_style)
                    .into()
            }
        }
    }
}

fn playlist_entry_layout<'a>(playlist_name: &String) -> Element<'a, PlaylistMenuMessages> {
    let entry_name = format!("{}", playlist_name);

    Container::new(
        Row::new()
            .spacing(10)
            .align_y(Alignment::Center)
            .push(
                Text::new(entry_name).size(16) // TODO add a border around text
            )
            .push(Space::new().width(Length::Fill))
            .push(
                Button::new(Image::new(PLAY_PLAYLIST_ICON).width(32).height(32))
                    .on_press(PlaylistMenuMessages::PlayPlaylist)
                    .style(active_large_button_style)
            )
            .push(
                Button::new(Image::new(EDIT_PLAYLIST_ICON).width(32).height(32))
                    .on_press(PlaylistMenuMessages::EditPlaylist)
                    .style(active_large_button_style)
            )
            .push(
                Button::new(Image::new(DELETE_PLAYLIST_ICON).width(32).height(32))
                    .on_press(PlaylistMenuMessages::DeletePlaylist)
                    .style(active_large_button_style)
            )
    )
        .padding(10)
        .width(Length::Fill)
        .style(playlist_entry_style)
        .into()
}
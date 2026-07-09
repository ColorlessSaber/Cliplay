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
        text_input,
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
    functions::load_video_file,
    io_utils::{
        app_directory_path,
        currently_saved_playlists,
        PLAYLIST_FOLDER,
        SaveError,
        LoadError,
    },
    playlist_data_struct::PlaylistData,
};
use rfd::AsyncFileDialog;
use std::path::Path;

#[derive(Debug, Clone)]
pub enum PlaylistMenuMessages {
    LoadPlaylist(String),
    PlayPlaylist(Result<PlaylistData, LoadError>),
    NewPlaylist,
    EditPlaylist,
    DeletePlaylist,
    SelectVideo,
    AddVideoToPlaylist(Option<String>),
    RemoveVideo(usize),
    Save,
    PlaylistSaved(Result<(), SaveError>),
    Cancel,
    PlaylistNameEdited(String),
}

#[derive(Debug, Clone)]
enum PlaylistMenuState {
    PlaylistList,
    PlaylistEditor,
}

pub struct PlaylistMenuScreen {
    playlist_menu_state: PlaylistMenuState,
    temp_playlist_list: Vec<String>,
    temp_playlist_name: String,
}

impl PlaylistMenuScreen {
    pub fn new() -> Self {
        Self {
            playlist_menu_state: PlaylistMenuState::PlaylistList,
            temp_playlist_list: Vec::new(),
            temp_playlist_name: String::new(),
        }
    }

    pub fn update(&mut self, message: PlaylistMenuMessages, state: &mut AppState) -> Task<PlaylistMenuMessages> {
        match message {
            PlaylistMenuMessages::LoadPlaylist(playlist_name) => {
                Task::perform(
                    PlaylistData::load(playlist_name),
                    PlaylistMenuMessages::PlayPlaylist,
                )
            }
            PlaylistMenuMessages::PlayPlaylist(playlist_data) => {
                if let Ok(playlist_data) = playlist_data {
                    // TODO see about allowing user to see videos in playlist on video player view side
                    state.playlist_manager.load_playlist(playlist_data.list);

                    // TODO write code to handle changes of playlist version and show the playlist name
                    println!("App version playlist was made under: {}", playlist_data.software_version);
                    println!("Playlist name: {}", playlist_data.name);

                    let video_file = state.playlist_manager.pull_first_file_from_playlist();
                    match video_file {
                        Some(video_file) => {
                            state.video = Some(load_video_file(video_file));
                        }
                        None => {}
                    }

                    state.btn_struct.main_menu_button.toggle_state(); // switch to video view
                } else {
                    println!("Failed to load playlist data"); // TODO add an error handling code later
                }

                Task::none()
            }
            PlaylistMenuMessages::NewPlaylist => {
                self.playlist_menu_state = PlaylistMenuState::PlaylistEditor;
                self.temp_playlist_name.clear();
                self.temp_playlist_list.clear();

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
            PlaylistMenuMessages::SelectVideo => {
                println!("Adding video to playlist");

                Task::perform(
                    async {
                        AsyncFileDialog::new()
                            .add_filter("video", &["mp4", "mkv", "m4v"])
                            .pick_file()
                            .await
                            .map(|handle| handle.path().to_string_lossy().into_owned())
                    },
                    PlaylistMenuMessages::AddVideoToPlaylist,
                )
            }
            PlaylistMenuMessages::AddVideoToPlaylist(video_path) => {
                if let Some(video_path) = video_path {
                    self.temp_playlist_list.push(video_path);
                }

                Task::none()
            }
            PlaylistMenuMessages::RemoveVideo(video_index) => {
                self.temp_playlist_list.remove(video_index);

                Task::none()
            }
            PlaylistMenuMessages::Save => {
                println!("Saving playlist");
                Task::perform(
                    PlaylistData{
                        software_version: "0.1".to_string(),
                        name: self.temp_playlist_name.clone(),
                        list: self.temp_playlist_list.clone(),
                    }.save(),
                    PlaylistMenuMessages::PlaylistSaved,
                )
            }
            PlaylistMenuMessages::Cancel => {
                self.playlist_menu_state = PlaylistMenuState::PlaylistList;
                self.temp_playlist_name.clear();
                self.temp_playlist_list.clear();
                
                Task::none()
            }
            PlaylistMenuMessages::PlaylistSaved(_result) => {
                self.playlist_menu_state = PlaylistMenuState::PlaylistList;
                self.temp_playlist_name.clear();
                self.temp_playlist_list.clear();

                Task::none()
            }
            PlaylistMenuMessages::PlaylistNameEdited(playlist_name) => {
                self.temp_playlist_name = playlist_name;

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
            PlaylistMenuState::PlaylistEditor => {

                let video_list = if self.temp_playlist_list.is_empty() {
                    Column::new()
                        .spacing(10)
                        .width(Length::Fill)
                        .push(Text::new("Add videos to the playlist!"))
                } else {
                    let list_of_videos = keyed_column(
                        (0..=self.temp_playlist_list.len()-1).map(|i|{
                            let file_name = Path::new(self.temp_playlist_list.get(i).unwrap())
                                .file_name()
                                .unwrap()
                                .to_string_lossy()
                                .to_string();

                            (i, video_entry_layout(file_name, i))
                        }));

                    Column::new()
                        .spacing(10)
                        .width(Length::Fill)
                        .push(scrollable(list_of_videos).spacing(20))
                };

                Container::new(
                    Column::new()
                        .spacing(10)
                        .width(Length::Fill)
                        .push(
                            Row::new()
                                .spacing(10)
                                .push(
                                    text_input("playlist name...", &self.temp_playlist_name)
                                        .on_input(PlaylistMenuMessages::PlaylistNameEdited)
                                        .padding(10)
                                )
                                .width(Length::Fill)
                                .push(
                                    Button::new(Image::new(SELECT_VIDEO_FILE_ICON).width(64).height(64))
                                        .on_press(PlaylistMenuMessages::SelectVideo)
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
                        .push(
                            video_list
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
                Text::new(entry_name).size(16)
            )
            .push(Space::new().width(Length::Fill))
            .push(
                Button::new(Image::new(PLAY_PLAYLIST_ICON).width(32).height(32))
                    .on_press(PlaylistMenuMessages::LoadPlaylist(playlist_name.clone()))
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

fn video_entry_layout<'a>(video_name: String, index: usize) -> Element<'a, PlaylistMenuMessages> {
    let entry_name = format!("{}", video_name);

    Container::new(
        Row::new()
            .spacing(10)
            .align_y(Alignment::Center)
            .push(
                Text::new(entry_name).size(16)
            )
            .push(Space::new().width(Length::Fill))
            .push(
                Button::new(Image::new(REMOVE_VIDEO_FILE_ICON).width(32).height(32))
                    .on_press(PlaylistMenuMessages::RemoveVideo(index))
                    .style(active_large_button_style)
            )
    )
        .padding(10)
        .width(Length::Fill)
        .style(playlist_entry_style)
        .into()
}
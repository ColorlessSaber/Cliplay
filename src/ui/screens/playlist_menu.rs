use crate::ui::{
    app::AppState,
    styling::{
        button_styles::active_large_button_style,
        container_styles::{
            main_section_style,
            playlist_entry_style,
        },
        icons::playlist_menu_icons,
    },
};
use crate::utils::{
    create_url_from_file_path::{
        LoadVideoFileError,
        create_url_from_file_path,
    }
    ,
    io_utils::{
        LoadError,
        SaveError,
        app_directory_path::app_directory_path,
        playlist_crud_cmds::{
            PlaylistData
            ,
            delete_selected_playlist,
            scan_playlist_folder},
    }};
use iced::{
    Alignment,
    Element,
    Length,
    Task,
    widget::{
        Button,
        Column,
        Container,
        Image,
        Row,
        Space,
        Text,
        button,
        column,
        image,
        keyed_column,
        row,
        scrollable,
        text,
        text_input,
    },
};
use iced_video_player::Video;
use rfd::AsyncFileDialog;
use std::path::Path;

#[derive(Debug, Clone)]
pub enum PlaylistMenuMessages {
    LoadPlaylist(String, AfterLoadingPlaylistProcess),
    PlayPlaylist(Result<PlaylistData, LoadError>),
    NewPlaylist,
    EditPlaylist(Result<PlaylistData, LoadError>),
    DeletePlaylist(String),
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

// handle the different processes after loading playlist
#[derive(Debug, Clone)]
pub enum AfterLoadingPlaylistProcess {
    Play,
    Edit,
}

pub struct PlaylistMenuScreen {
    playlist_menu_state: PlaylistMenuState,
    temp_playlist_list: Vec<String>,
    temp_playlist_name: String,
    enable_editor_mode: bool,
}

impl PlaylistMenuScreen {
    pub fn new() -> Self {
        Self {
            playlist_menu_state: PlaylistMenuState::PlaylistList,
            temp_playlist_list: Vec::new(),
            temp_playlist_name: String::new(),
            enable_editor_mode: false,
        }
    }

    pub fn update(&mut self, message: PlaylistMenuMessages, state: &mut AppState) -> Task<PlaylistMenuMessages> {
        match message {
            PlaylistMenuMessages::LoadPlaylist(playlist_name, after_process) => {
                Task::perform(
                    PlaylistData::load(playlist_name),
                    match after_process {
                        AfterLoadingPlaylistProcess::Play => PlaylistMenuMessages::PlayPlaylist,
                        AfterLoadingPlaylistProcess::Edit => PlaylistMenuMessages::EditPlaylist,
                    },
                )
            }
            PlaylistMenuMessages::PlayPlaylist(playlist_data) => {
                if let Ok(playlist_data) = playlist_data {
                    state.playlist_manager.load_playlist(playlist_data.list, playlist_data.name);

                    // TODO write code to handle changes of playlist version
                    println!("App version playlist was made under: {}", playlist_data.software_version);

                    // Check to see if the first video file will load. If not keep looping through
                    // the playlist until a video file is successfully loaded
                    // TODO pass any error(s) to a pop-up window
                    let mut try_again = false;
                    loop {

                        let video_file = if !try_again {
                            state.playlist_manager.pull_first_file_from_playlist()
                        } else {
                            // the loop all parameter is set to false for if all video file(s)
                            // are unable to load no point to keep trying.
                            // Similar logic for shuffling the playlist.
                            state.playlist_manager.next_file_in_playlist(false, false)
                        };
                        
                        match video_file {
                            Some(video_file) => {
                                match create_url_from_file_path(video_file) {
                                    Ok(video_url_path) => {
                                        match Video::new(&video_url_path) {
                                            Ok(mut loaded_video) => {
                                                loaded_video.set_looping(
                                                    state.btn_struct.loop_button.is_state_set_to_loop_single(),
                                                );
                                                state.video = Some(loaded_video);
                                                break;
                                            },
                                            Err(video_error) => {
                                                println!("Failed to load video: {:?}, Iced video error: {:?}",
                                                         video_file,
                                                         video_error
                                                );
                                                try_again = true;
                                            }
                                        }
                                    },
                                    Err(url_error) => match url_error {
                                        LoadVideoFileError::NotAbsolutePath => {
                                            let error_message = "the BufPath failed to generate absolute path.".to_string();
                                            println!("failed to create URL path from video file: {:?}, url error: {:?}",
                                                     video_file,
                                                     error_message
                                            );
                                        },
                                        LoadVideoFileError::Io(io_error) => {
                                            let error_message = format!("{:?}", io_error);
                                            println!("failed to create URL path from video file: {:?}, url error: {:?}",
                                                     video_file,
                                                     error_message
                                            );
                                        }
                                    }
                                }
                            },
                            None => break, // reached end of playlist, meaning all video file(s) in playlist failed to load
                        }
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
                self.enable_editor_mode = false;

                Task::none()
            }
            PlaylistMenuMessages::EditPlaylist(playlist_data) => {
                if let Ok(playlist_data) = playlist_data {
                    self.temp_playlist_name = playlist_data.name;
                    self.temp_playlist_list = playlist_data.list;
                    self.enable_editor_mode = true;
                    self.playlist_menu_state = PlaylistMenuState::PlaylistEditor;
                }

                Task::none()
            }
            PlaylistMenuMessages::DeletePlaylist(playlist_name) => {
                let result = delete_selected_playlist(app_directory_path(), playlist_name);

                match result {
                    Ok(_) => {
                        println!("Playlist deleted successfully");
                    }
                    Err(e) => {
                        println!("Failed to delete playlist: {:?}", e);
                    }
                }

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
            PlaylistMenuMessages::PlaylistSaved(_result) => {
                self.playlist_menu_state = PlaylistMenuState::PlaylistList;
                self.temp_playlist_name.clear();
                self.temp_playlist_list.clear();

                Task::none()
            }
            PlaylistMenuMessages::Cancel => {
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

    pub fn view<'a, 'b>(&'a self) -> Element<'b, PlaylistMenuMessages>
    where 'a: 'b
    {
        match self.playlist_menu_state {
            PlaylistMenuState::PlaylistList => {
                let playlists_found = scan_playlist_folder(app_directory_path());

                match playlists_found {
                    Ok(playlists) => {
                        // to reduce duplicate code, created a starting playlist column and then depending
                        // on the number of playlists found, generate the playlist column and then
                        // pass it out to be inserted into an Iced container.
                        let new_playlist_btn: Button<PlaylistMenuMessages> = button(image(playlist_menu_icons::NEW_PLAYLIST_ICON).width(64).height(64))
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

                        // The final container showing the playlists, if any
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
                                .width(Length::Fill)
                                .push(
                                    match self.enable_editor_mode {
                                        true => { // show playlist name but disable the ability to change it
                                            text_input("", &self.temp_playlist_name)
                                                .padding(10)
                                        }
                                        false => {
                                            text_input("playlist name...", &self.temp_playlist_name)
                                                .on_input(PlaylistMenuMessages::PlaylistNameEdited)
                                                .padding(10)
                                        }
                                    }
                                )
                                .push(
                                    Button::new(Image::new(playlist_menu_icons::editor::SELECT_VIDEO_FILE_ICON).width(64).height(64))
                                        .on_press(PlaylistMenuMessages::SelectVideo)
                                        .style(active_large_button_style)
                                )
                                .push(
                                    Button::new(Image::new(playlist_menu_icons::editor::SAVE_ICON).width(64).height(64))
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

fn playlist_entry_layout<'a>(
    playlist_name: &String
) -> Element<'a, PlaylistMenuMessages> {

    let default_btn = |image, message| {
        button(image).on_press(message).style(active_large_button_style)
    };

    Container::new(
        row![
            text!("{}", playlist_name.clone()).size(16),
            Space::new().width(Length::Fill),
            default_btn(Image::new(playlist_menu_icons::entry::PLAY_PLAYLIST_ICON).width(32).height(32), PlaylistMenuMessages::LoadPlaylist(playlist_name.clone(), AfterLoadingPlaylistProcess::Play)),
            default_btn(Image::new(playlist_menu_icons::entry::EDIT_PLAYLIST_ICON).width(32).height(32), PlaylistMenuMessages::LoadPlaylist(playlist_name.clone(), AfterLoadingPlaylistProcess::Edit)),
            default_btn(Image::new(playlist_menu_icons::entry::DELETE_PLAYLIST_ICON).width(32).height(32), PlaylistMenuMessages::DeletePlaylist(playlist_name.clone()))
        ]
            .spacing(10)
            .align_y(Alignment::Center)
    )
        .padding(10)
        .width(Length::Fill)
        .style(playlist_entry_style)
        .into()
}

fn video_entry_layout<'a>(
    video_name: String,
    index: usize
) -> Element<'a, PlaylistMenuMessages> {

    Container::new(
        Row::new()
            .spacing(10)
            .align_y(Alignment::Center)
            .push(
                Text::new(video_name).size(16)
            )
            .push(Space::new().width(Length::Fill))
            .push(
                Button::new(Image::new(playlist_menu_icons::editor::REMOVE_VIDEO_FILE_ICON).width(32).height(32))
                    .on_press(PlaylistMenuMessages::RemoveVideo(index))
                    .style(active_large_button_style)
            )
    )
        .padding(10)
        .width(Length::Fill)
        .style(playlist_entry_style)
        .into()
}
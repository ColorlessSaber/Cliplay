use iced::{
    Element,
    Length,
    Task,
    Alignment,
    widget::{
        Button,
        Text,
        Column,
        Row,
        Image,
        Space,
        Container,
        scrollable,
        keyed_column,
        column,
    },
};
use iced::widget::button;
use rfd::AsyncFileDialog;
use crate::ui::styling::{
    active_large_button_style,
    container_styles::{
        main_section_style,
        playlist_entry_style
    },
    icons::{
        SELECT_VID_FROM_COMPUTER_ICON,
        PLAYLISTS_ICON,
        NEW_PLAYLIST_ICON,
        EDIT_PLAYLIST_ICON,
        DELETE_PLAYLIST_ICON,
        PLAY_PLAYLIST_ICON,
        SELECT_VIDEO_FILE_ICON,
        REMOVE_VIDEO_FILE_ICON,
        SAVE_ICON,
    }
};
use crate::utils::{
    app_state::AppState,
    save_utils::app_directory_path,
    functions::{
        load_video_file,
        currently_saved_playlists,
    }
};

#[derive(Debug, Clone)]
pub enum MainMenuMessages {
    SelectVideo,
    PlaylistsMenu,
    SettingsMenu,
    VideoFileSelected(Option<String>),
    NewPlaylist,
    EditPlaylist,
    DeletePlaylist,
    PlayPlaylist,
    AddVideoToPlaylist,
    RemoveVideoFromPlaylist,
    SavePlaylist,
}

// Keep track of what button was last pressed
enum MenuSelectedState {
    SettingsMenu,
    PlaylistsMenu,
}
#[derive(Debug, Clone)]
enum PlaylistMenuState {
    NewPlaylist,
    PlaylistsMenu,
}

// Main Menu GUI Struct
pub struct MainMenuScreen {
    currently_selected_main_menu_btn: MenuSelectedState,
    playlist_menu_state: PlaylistMenuState
}

impl MainMenuScreen {

    pub fn new() -> Self {
        Self {
            currently_selected_main_menu_btn: MenuSelectedState::PlaylistsMenu,
            playlist_menu_state: PlaylistMenuState::PlaylistsMenu
        }
    }

    pub fn update(&mut self, message: MainMenuMessages, state: &mut AppState) -> Task<MainMenuMessages> {
        match message {
            MainMenuMessages::SelectVideo => {
                Task::perform(
                    async {
                        AsyncFileDialog::new()
                            .add_filter("video", &["mp4", "mkv", "m4v"])
                            .pick_file()
                            .await
                            .map(|handle| handle.path().to_string_lossy().into_owned())
                    },
                    MainMenuMessages::VideoFileSelected,
                )
            }
            MainMenuMessages::VideoFileSelected(path) => {
                if let Some(path) = path {
                    state.playlist_manager.play_single_video_file(path);
                    let video_file = state.playlist_manager.pull_first_file_from_playlist();
                    
                    match video_file {
                        Some(video_file) => {
                            state.video = Some(load_video_file(video_file));
                        }
                        None => {}
                    }

                    state.btn_struct.main_menu_button.toggle_state();
                }
                Task::none()
            }
            MainMenuMessages::PlaylistsMenu => {
                self.currently_selected_main_menu_btn = MenuSelectedState::PlaylistsMenu;

                Task::none()
            }
            MainMenuMessages::SettingsMenu => {
                self.currently_selected_main_menu_btn = MenuSelectedState::SettingsMenu;

                Task::none()
            }
            MainMenuMessages::NewPlaylist => {
                self.playlist_menu_state = PlaylistMenuState::NewPlaylist;

                Task::none()
            }
            MainMenuMessages::EditPlaylist => {
                println!("Editing playlist");

                Task::none()
            }
            MainMenuMessages::DeletePlaylist => {
                println!("Deleting playlist");

                Task::none()
            }
            MainMenuMessages::PlayPlaylist => {
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
            MainMenuMessages::AddVideoToPlaylist => {
                println!("Adding video to playlist");

                Task::none()
            }
            MainMenuMessages::RemoveVideoFromPlaylist => {
                println!("Removing video from playlist");

                Task::none()
            }
            MainMenuMessages::SavePlaylist => {
                println!("Saving playlist");
                self.playlist_menu_state = PlaylistMenuState::PlaylistsMenu;

                Task::none()
            }
        }
    }

    pub fn view(&self) -> Element<'_, MainMenuMessages> {
        Row::new()
            .push(
                // the main menu buttons: video select, playlist, and settings
                Container::new(
                    Column::new()
                        .spacing(10)
                        .push(
                            Button::new(Image::new(SELECT_VID_FROM_COMPUTER_ICON).width(64).height(64))
                                .on_press(MainMenuMessages::SelectVideo)
                                .style(active_large_button_style)
                        )
                        .push(
                            Button::new(Image::new(PLAYLISTS_ICON).width(64).height(64))
                                .on_press(MainMenuMessages::PlaylistsMenu)
                                .style(active_large_button_style)
                        )
                        .push(
                            Button::new(Text::new("Settings").width(64).height(64))
                                .on_press(MainMenuMessages::SettingsMenu)
                                .style(active_large_button_style)
                        )
                        .push(Space::new().height(Length::Fill))
                )
                    .padding(10)
                    .style(main_section_style)
            )
            .push(
                // The changeable view based on currently selected menu button
                match self.currently_selected_main_menu_btn {
                    MenuSelectedState::SettingsMenu => Container::new(Text::new("Settings Menu")), // TODO create a fleshed out settings menu
                    MenuSelectedState::PlaylistsMenu => playlist_menu(&self.playlist_menu_state)
                }
            ).into()
    }
}

fn playlist_menu<'a>(
    playlist_menu_state: &PlaylistMenuState,
) -> Container<'a, MainMenuMessages> {
    match playlist_menu_state {
        PlaylistMenuState::PlaylistsMenu => {
            let playlists_found = currently_saved_playlists(app_directory_path());

            match playlists_found {
                Ok(playlists) => {
                    // to reduce duplicate code, created a starting playlist column and then depending
                    // on the number of playlists found, generate the playlist column and then
                    // pass it out to be inserted into an Iced container.
                    let new_playlist_btn: Button<MainMenuMessages> = button(iced::widget::image(NEW_PLAYLIST_ICON).width(64).height(64))
                        .on_press(MainMenuMessages::NewPlaylist)
                        .style(active_large_button_style);

                    let starting_playlist_column = column![new_playlist_btn]
                        .spacing(10)
                        .width(Length::Fill);

                    let finalized_playlist_column = if playlists.is_empty() {
                        starting_playlist_column.push(Text::new("No playlists found"))
                    } else {
                        let list_of_playlists = keyed_column(
                            (0..=playlists.len()-1).map(|i|{ // minus one for .len() counts with 1
                                (i, playlist_entry_layout(playlists.get(i).unwrap()))
                            }));

                        starting_playlist_column.push(scrollable(list_of_playlists).spacing(20))
                    };

                    Container::new(
                        finalized_playlist_column
                    )
                        .padding(10)
                        .height(Length::Fill)
                        .style(main_section_style)
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
                            .on_press(MainMenuMessages::AddVideoToPlaylist)
                            .style(active_large_button_style)
                    )
                    .push(
                        Button::new(Image::new(REMOVE_VIDEO_FILE_ICON).width(64).height(64))
                            .on_press(MainMenuMessages::RemoveVideoFromPlaylist)
                            .style(active_large_button_style)
                    )
                    .push(
                        Button::new(Image::new(SAVE_ICON).width(64).height(64))
                            .on_press(MainMenuMessages::SavePlaylist)
                            .style(active_large_button_style)
                    )
            )
                .padding(10)
                .height(Length::Fill)
                .style(main_section_style)
        }
    }
}

fn playlist_entry_layout<'a>(playlist_name: &String) -> Element<'a, MainMenuMessages> {
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
                    .on_press(MainMenuMessages::PlayPlaylist)
                    .style(active_large_button_style)
            )
            .push(
                Button::new(Image::new(EDIT_PLAYLIST_ICON).width(32).height(32))
                    .on_press(MainMenuMessages::EditPlaylist)
                    .style(active_large_button_style)
            )
            .push(
                Button::new(Image::new(DELETE_PLAYLIST_ICON).width(32).height(32))
                    .on_press(MainMenuMessages::DeletePlaylist)
                    .style(active_large_button_style)
            )
    )
        .padding(10)
        .width(Length::Fill)
        .style(playlist_entry_style)
        .into()
}
use crate::ui::styling::{
    button_styles::active_large_button_style,
    container_styles::main_section_style,
    icons::main_menu_icons::*,
};
use crate::ui::{
    app::AppState,
    screens::{
        playlist_menu::{
            PlaylistMenuMessages,
            PlaylistMenuScreen,
        },
        settings_menu::{
            SettingsMenu,
            SettingsMenuMessage,
        },
    },
};
use crate::utils::
create_url_from_file_path::{
    LoadVideoFileError,
    create_url_from_file_path,
}
;
use iced::{
    Element,
    Length,
    Task,
    widget::{
        Container,
        Image,
        Row,
        Space,
        button,
        column,
    },
};
use iced_video_player::Video;
use rfd::AsyncFileDialog;

#[derive(Debug, Clone)]
pub enum MainMenuMessages {
    SelectVideo,
    TogglePlaylistMenu,
    PlaylistsMenu(PlaylistMenuMessages),
    SettingsMenu(SettingsMenuMessage),
    ToggleSettingsMenu,
    VideoFileSelected(Option<String>),
}

// Keep track of what button was last pressed
enum MenuSelectedState {
    SettingsMenu,
    PlaylistsMenu,
}

// Main Menu GUI Struct
pub struct MainMenuScreen {
    currently_selected_main_menu_btn: MenuSelectedState,
    playlist_menu: PlaylistMenuScreen,
    settings_menu: SettingsMenu,
}

impl MainMenuScreen {

    pub fn new() -> Self {
        Self {
            currently_selected_main_menu_btn: MenuSelectedState::PlaylistsMenu,
            playlist_menu: PlaylistMenuScreen::new(),
            settings_menu: SettingsMenu::new(),
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
                    state.playlist_manager.load_single_video_file(path);
                    let video_file = state.playlist_manager.pull_first_file_from_playlist().unwrap();

                    // TODO pass error to a pop-up window
                    match create_url_from_file_path(video_file) {
                        Ok(video_url_path) => {
                            match Video::new(&video_url_path) {
                                Ok(mut loaded_video) => {
                                    loaded_video.set_looping(
                                        state.btn_struct.loop_button.is_state_set_to_loop_single(),
                                    );
                                    state.video = Some(loaded_video);
                                },
                                Err(video_error) => {
                                    println!("Failed to load video: {:?}, Iced video error: {:?}",
                                             video_file,
                                             video_error
                                    );
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

                    state.btn_struct.main_menu_button.toggle_state();
                }
                Task::none()
            }
            MainMenuMessages::TogglePlaylistMenu => {
                self.currently_selected_main_menu_btn = MenuSelectedState::PlaylistsMenu;

                Task::none()
            }
            MainMenuMessages::PlaylistsMenu(message) => {
                self.playlist_menu
                    .update(message, state)
                    .map(|message| MainMenuMessages::PlaylistsMenu(message))
            }
            MainMenuMessages::ToggleSettingsMenu => {
                self.currently_selected_main_menu_btn = MenuSelectedState::SettingsMenu;

                Task::none()
            }
            MainMenuMessages::SettingsMenu(message) => {
                self.settings_menu
                    .update(message, state)
                    .map(|message| MainMenuMessages::SettingsMenu(message))
            }
        }
    }

    pub fn view<'a, 'b>(&'a self, state: &'a AppState) -> Element<'b, MainMenuMessages>
    where 'a: 'b
    {

        let default_btn = |image, message| {
            button(image).on_press(message).style(active_large_button_style)
        };

        Row::new()
            .push(
                // the main menu buttons: video select, playlist, and settings
                Container::new(
                    column![
                        default_btn(Image::new(SELECT_VID_FROM_COMPUTER_ICON).width(64).height(64), MainMenuMessages::SelectVideo),
                        default_btn(Image::new(PLAYLISTS_ICON).width(64).height(64), MainMenuMessages::TogglePlaylistMenu),
                        default_btn(Image::new(SETTINGS_ICON).width(64).height(64), MainMenuMessages::ToggleSettingsMenu),
                        Space::new().height(Length::Fill)
                    ].spacing(10)
                )
                    .padding(10)
                    .style(main_section_style)
            )
            .push(
                // The changeable view based on currently selected menu button
                match self.currently_selected_main_menu_btn {
                    MenuSelectedState::SettingsMenu => Container::new(self.settings_menu.view(state).map(MainMenuMessages::SettingsMenu)),
                    MenuSelectedState::PlaylistsMenu => Container::new(self.playlist_menu.view().map(MainMenuMessages::PlaylistsMenu)),
                }
            ).into()
    }
}
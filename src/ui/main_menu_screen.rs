use iced::{
    Element,
    Length,
    Task,
    widget::{
        Button,
        Text,
        Column,
        Row,
        Image,
        Space,
        Container,
    },
};
use rfd::AsyncFileDialog;
use crate::ui::playlist_menu_screen::{
    PlaylistMenuScreen,
    PlaylistMenuMessages,
};
use crate::ui::styling::{
    active_large_button_style,
    container_styles::{
        main_section_style,
    },
    icons::{
        SELECT_VID_FROM_COMPUTER_ICON,
        PLAYLISTS_ICON,
    }
};
use crate::utils::{
    app_state::AppState,
    functions::{
        load_video_file,
    }
};

#[derive(Debug, Clone)]
pub enum MainMenuMessages {
    SelectVideo,
    PlaylistsMenu(PlaylistMenuMessages),
    SettingsMenu,
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
}

impl MainMenuScreen {

    pub fn new() -> Self {
        Self {
            currently_selected_main_menu_btn: MenuSelectedState::PlaylistsMenu,
            playlist_menu: PlaylistMenuScreen::new(),
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
            MainMenuMessages::PlaylistsMenu(message) => {
                self.currently_selected_main_menu_btn = MenuSelectedState::PlaylistsMenu;

                self.playlist_menu
                    .update(message, state)
                    .map(|message| MainMenuMessages::PlaylistsMenu(message))
            }
            MainMenuMessages::SettingsMenu => {
                self.currently_selected_main_menu_btn = MenuSelectedState::SettingsMenu;

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
                                .on_press(MainMenuMessages::PlaylistsMenu(PlaylistMenuMessages::PlayPlaylist))
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
                    MenuSelectedState::PlaylistsMenu => Container::new(self.playlist_menu.view().map(MainMenuMessages::PlaylistsMenu)),
                }
            ).into()
    }
}
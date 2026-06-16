use iced::{
    Element,
    Length,
    Task,
    widget::{
        Button,
        text_input,
        Column,
        Row,
        Grid,
        Image,
        Space,
        Container
    },
};
use rfd::AsyncFileDialog;
use crate::ui::styling::{
    active_large_button_style,
    container_styles::main_section_style,
    icons::{
        SELECT_SINGLE_VID_ICON,
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
    functions::load_video_file
};

#[derive(Debug, Clone)]
pub enum MainMenuMessages {
    SelectVideo,
    PlaylistsMenu,
    VideoFileSelected(Option<String>),
    NewPlaylist,
    EditPlaylist,
    DeletePlaylist,
    PlayPlaylist,
    PlaylistNameEdited(String),
    AddVideoToPlaylist,
    RemoveVideoFromPlaylist,
    SavePlaylist,
}

// Keep track of what button was last pressed
enum MenuSelectedState {
    SelectVideo,
    PlaylistsMenu,
}

pub struct MainMenuScreen {
    currently_selected_button: MenuSelectedState,
    temp_playlist_name: String,
}

impl MainMenuScreen {

    pub fn new() -> Self {
        Self {
            currently_selected_button: MenuSelectedState::SelectVideo,
            temp_playlist_name: String::new(),
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
                self.currently_selected_button = MenuSelectedState::SelectVideo;

                if let Some(path) = path {
                    state.playlist_manager.clear_playlist();
                    state.playlist_manager.add_file_to_playlist(path);
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
                self.currently_selected_button = MenuSelectedState::PlaylistsMenu;

                Task::none()
            }
            MainMenuMessages::NewPlaylist => {
                println!("New playlist");

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
                state.playlist_manager.load_playlist();
                let video_file = state.playlist_manager.pull_first_file_from_playlist();

                match video_file {
                    Some(video_file) => {
                        state.video = Some(load_video_file(&video_file));
                    }
                    None => {}
                }

                state.btn_struct.main_menu_button.toggle_state(); // to switch to video view
                Task::none()
            }
            MainMenuMessages::PlaylistNameEdited(playlist_name) => {
                self.temp_playlist_name = playlist_name;

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

                Task::none()
            }
        }
    }

    pub fn view(&self) -> Element<'_, MainMenuMessages>
    {
        Row::new()
            .push(
                // the main menu buttons: video select, playlist, and settings
                Container::new(
                    Column::new()
                        .spacing(10)
                        .push(
                            Button::new(Image::new(SELECT_SINGLE_VID_ICON).width(64).height(64))
                                .on_press(MainMenuMessages::SelectVideo)
                                .style(active_large_button_style)
                        )
                        .push(
                            Button::new(Image::new(PLAYLISTS_ICON).width(64).height(64))
                                .on_press(MainMenuMessages::PlaylistsMenu)
                                .style(active_large_button_style)
                        )
                        .push(Space::new().height(Length::Fill))
                )
                    .padding(10)
                    .style(main_section_style)
            )
            .push(
                // The changeable view based on currently selected menu button
                match self.currently_selected_button {
                    MenuSelectedState::SelectVideo => Container::new(Image::new(SELECT_SINGLE_VID_ICON)),
                    MenuSelectedState::PlaylistsMenu => playlist_menu(self.temp_playlist_name.as_str())
                }
            ).into()
    }
}

fn playlist_menu(
    playlist_name: &str,
) -> Container<'_, MainMenuMessages> {
    Container::new(
        Row::new()
            .push(
                Container::new( // Playlist Menu buttons and playlist selection
                    Grid::new()
                        .spacing(10)
                        .columns(2)
                        .width(200) // It also controls the size of the widgets
                        .push(
                            Button::new(Image::new(NEW_PLAYLIST_ICON).width(64).height(64))
                                .on_press(MainMenuMessages::NewPlaylist)
                                .style(active_large_button_style)
                        )
                        .push(
                            Button::new(Image::new(EDIT_PLAYLIST_ICON).width(64).height(64))
                                .on_press(MainMenuMessages::EditPlaylist)
                                .style(active_large_button_style)
                        )
                        .push(
                            Button::new(Image::new(DELETE_PLAYLIST_ICON).width(64).height(64))
                                .on_press(MainMenuMessages::DeletePlaylist)
                                .style(active_large_button_style)
                        )
                        .push(
                            Button::new(Image::new(PLAY_PLAYLIST_ICON).width(64).height(64))
                                .on_press(MainMenuMessages::PlayPlaylist)
                                .style(active_large_button_style)
                        )
                )
                    .padding(10)
                    .height(Length::Fill)
                    .style(main_section_style)
            )
            .push(
                Container::new( // Playlist editor buttons and name input
                    Column::new()
                        .spacing(10)
                        .push(
                            Row::new()
                                .spacing(10)
                                .push(
                                    text_input("Enter playlist name...", playlist_name)
                                        .on_input(MainMenuMessages::PlaylistNameEdited)
                                )
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
                )
                    .padding(10)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .style(main_section_style)
            )
    )
}
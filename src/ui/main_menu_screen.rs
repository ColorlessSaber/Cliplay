use iced::{
    Element,
    Length,
    Task,
    widget::{
        Button,
        Column,
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
        PLAYLISTS_ICON
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
}

pub struct MainMenuScreen {}

impl MainMenuScreen {

    pub fn new() -> Self {
        Self {}
    }

    pub fn update(&self, message: MainMenuMessages, state: &mut AppState) -> Task<MainMenuMessages> {
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
        }
    }

    pub fn view(&self) -> Element<'_, MainMenuMessages>
    {
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
            .into()
    }
}
use crate::ui::{
    dynamic_buttons::{
        DynamicButtons,
        StyleState,
        dynamic_loop_button::DynamicLoopBtnState,
        dynamic_main_menu_button::DynamicMainMenuBtnState,
        dynamic_playlist_info_button::DynamicPlaylistInfoBtnState,
        dynamic_shuffle_button::DynamicShuffleBtnState,
    },
    screens::main_menu::{
        MainMenuMessages,
        MainMenuScreen,
    },
    styling::container_styles::{
        control_bar_style,
        splash_screen_style,
        video_playing_style,
    },
    styling::icons::{
        logo_icons::CLIPLAY_LOGO_GREY_ICON,
        video_player_icons::*,
    },
    styling::{
        button_styles::{
            active_large_button_style,
            inactive_large_button_style,
            player_control_button_style,
        },
        slider_styles::volume_slider_style,
    },
};
use crate::utils::{
    create_url_from_file_path::{
        LoadVideoFileError,
        create_url_from_file_path,
    },
    io_utils::{
        app_directory_path::app_directory_path,
        app_settings_data_struct::{
            AppSettings,
            PlayerSettings,
        },
        create_application_directory,
    },
    playlist_manager::PlaylistManager,
};
use iced::{
    Element,
    Length,
    Subscription,
    Task,
    alignment::{Alignment, Horizontal, Vertical},
    keyboard,
    widget::{Button, Column, Container, Image, Row, Slider, Space, Stack, Text},
};
use iced_video_player::{Video, VideoPlayer};
use std::time::Duration;

// Holds the state, or information, of the app that can be shared between different views
pub struct AppState {
    pub video: Option<Video>,
    pub btn_struct: DynamicButtons,
    pub playlist_manager: PlaylistManager,
    pub settings: AppSettings,
}

#[derive(Clone, Debug)]
pub enum Message {
    TogglePause,
    ToggleLoop,
    ToggleShuffle,
    StopVideo,
    VideoSeek(f64),
    VideoSeekRelease,
    Forward(f64),
    Backward(f64),
    SkipForward,
    SkipBackward,
    VolumeSeek(f64),
    EndOfStream,
    NewFrame,
    ToggleMainMenu,
    MainMenu(MainMenuMessages),
    TogglePlaylistInfo,
}

pub struct App {
    position: f64,
    dragging: bool,
    state: AppState,
    main_menu_screen: MainMenuScreen,
}

// handles the different ways the next video will be loaded
enum NextVideoLoadingProcess {
    EndOfVideo, // when reaching the end of the video currently being played
    AfterVideoStopped, // when the video was stopped
    SkipForward, // when the skip forward button is pressed
    SkipBackward, // when the skip backward button is pressed
}

impl App {

    // Struct methods; IE, methods private to the struct.
    fn load_next_video(&mut self, loading_process: NextVideoLoadingProcess) {

        // Check to see if the next video file can be successfully load. If not,
        // loop through the playlist until a video file is found to load successfully.
        //
        // This does mean if only one video file in an n+1 playlist is only playable it will check
        // all other video files before coming back to the only one that can be played.
        // This is fine, until otherwise.
        let shuffle_state = self.state.btn_struct.shuffle_button.is_shuffle_on();
        loop {
            let video_file = match loading_process {
                NextVideoLoadingProcess::EndOfVideo => {
                    let loop_entire_playlist = self.state.btn_struct.loop_button.is_state_set_to_loop_all();
                    let video_file = self.state.playlist_manager.next_file_in_playlist(loop_entire_playlist, shuffle_state);
                    video_file
                }
                NextVideoLoadingProcess::AfterVideoStopped => {
                    let video_file = self.state.playlist_manager.pull_current_index_file_from_playlist();
                    video_file
                }
                NextVideoLoadingProcess::SkipForward => {
                    // Passing true into .next_file_in_playlist to loop back to beginning of playlist
                    // if we reached the end, regardless if loop button is set to "loop all"
                    let video_file = self.state.playlist_manager.next_file_in_playlist(true, shuffle_state);
                    video_file
                }
                NextVideoLoadingProcess::SkipBackward => {
                    let video_file = self.state.playlist_manager.previous_file_in_playlist();
                    video_file
                }
            };

            // TODO pass error to a pop-up window
            match video_file {
                Some(video_file) => {
                    match create_url_from_file_path(video_file) {
                        Ok(video_url_path) => {
                            match Video::new(&video_url_path) {
                                Ok(mut loaded_video) => {
                                    loaded_video.set_looping(
                                        self.state.btn_struct.loop_button.is_state_set_to_loop_single(),
                                    );
                                    self.state.video = Some(loaded_video);
                                    break;
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
                }
                None => {
                    self.state.video = None;
                    self.position = 0.0;
                    break;
                }
            }
        }
    }

    // Iced methods; IE, methods used by the Iced crate
    pub fn new() -> Self {
        // Start of "loading" stage logic
        // IE, creating the application directory, loading settings file, etc. before
        // launching the application.
        create_application_directory(app_directory_path());
        
        let settings_data = AppSettings::load().ok().unwrap();
        // End of "loading" stage logic

        Self {
            position: 0.0,
            dragging: false,
            state: AppState{
                video: None,
                btn_struct: DynamicButtons::default(),
                playlist_manager: PlaylistManager::new(),
                settings: settings_data,
            },
            main_menu_screen: MainMenuScreen::new(),
        }
    }

    pub fn title(&self) -> String {
        // The title of the GUI; shows at the top
        "Cliplay - Video Player".to_string()
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::TogglePause => {
                if let Some(video) = self.state.video.as_mut() {
                    video.set_paused(!video.paused());
                } else {
                    // if the user clicked stopped while the video is playing,
                    // replay the video from the beginning
                    if !self.state.playlist_manager.is_playlist_empty() {
                        self.load_next_video(NextVideoLoadingProcess::AfterVideoStopped);
                    }
                }
                Task::none()
            }
            Message::ToggleLoop => {
                self.state.btn_struct.loop_button.toggle_state_and_style();
                if self.state.btn_struct.loop_button.is_state_set_to_loop_single() {

                    // Handles the situation when no video is "loaded"
                    if let Some(video) = self.state.video.as_mut() {
                        video.set_looping(true);
                    }
                } else {

                    // Handles the situation when no video is "loaded"
                    if let Some(video) = self.state.video.as_mut() {
                        video.set_looping(false);
                    }
                }
                Task::none()
            }
            Message::ToggleShuffle => {
                self.state.btn_struct.shuffle_button.toggle_state_and_style();
                match self.state.btn_struct.shuffle_button.current_state() {
                    DynamicShuffleBtnState::On => {
                        if !self.state.playlist_manager.is_playlist_empty() {
                            self.state.playlist_manager.generate_shuffle_order()
                        }
                    },
                    DynamicShuffleBtnState::Off => self.state.playlist_manager.clear_shuffle_order()
                }
                Task::none()
            }
            Message::StopVideo => {
                self.state.video = None;
                self.position = 0.0;

                Task::none()
            }
            Message::VideoSeek(secs) => {
                self.dragging = true;
                self.state.video.as_mut().unwrap().set_paused(true); // Will remove unwrap once ready
                self.position = secs;
                Task::none()
            }
            Message::VideoSeekRelease => {
                self.dragging = false;
                self.state.video
                    .as_mut()
                    .unwrap()// will remove unwrap once ready
                    .seek(Duration::from_secs_f64(self.position), false)
                    .expect("seek");
                self.state.video.as_mut().unwrap().set_paused(false); // will remove unwrap once ready
                Task::none()
            }
            Message::Forward(secs) => {
                if self.state.video.is_some() {
                    self.position += secs;
                    self.state.video
                        .as_mut()
                        .unwrap() // will remove unwrap once ready
                        .seek(Duration::from_secs_f64(self.position), false)
                        .expect("forward");
                }
                Task::none()
            }
            Message::Backward(secs) => {
                if self.state.video.is_some() {
                    self.position -= secs;
                    self.state.video
                        .as_mut()
                        .unwrap() // will remove unwrap once ready
                        .seek(Duration::from_secs_f64(self.position), false)
                        .expect("backward");
                }
                Task::none()
            }
            Message::SkipForward => {
                if self.state.video.is_some() {
                    self.load_next_video(NextVideoLoadingProcess::SkipForward);
                }
                Task::none()
            }
            Message::SkipBackward => {
                if self.state.video.is_some() {
                    self.load_next_video(NextVideoLoadingProcess::SkipBackward);
                }
                Task::none()
            }
            Message::VolumeSeek(vol) => {
                if self.state.video.is_some() {
                    self.state.video.as_mut().unwrap().set_volume(vol);
                }
                Task::none()
            }
            Message::EndOfStream => {
                if !self.state.btn_struct.loop_button.is_state_set_to_loop_single() {
                    self.load_next_video(NextVideoLoadingProcess::EndOfVideo)
                }
                Task::none()
            }
            Message::NewFrame => {
                if !self.dragging {
                    self.position = self.state.video.as_ref().unwrap().position().as_secs_f64(); // will remove unwrap when ready
                }
                Task::none()
            }
            Message::ToggleMainMenu => {
                self.state.btn_struct.main_menu_button.toggle_state();
                Task::none()
            }
            Message::MainMenu(message) => {
                self.main_menu_screen
                    .update(message, &mut self.state)
                    .map(|message| Message::MainMenu(message))
            }
            Message::TogglePlaylistInfo => {
                self.state.btn_struct.playlist_info_button.toggle_state();
                Task::none()
            }
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        Column::new()
            .push(
                match self.state.btn_struct.main_menu_button.current_state() {
                    DynamicMainMenuBtnState::Closed => {
                        video_player_area(
                            &self.state.video,
                            &self.state.btn_struct,
                            &self.state.playlist_manager,
                        )
                    }
                    DynamicMainMenuBtnState::Open => {
                        Container::new(
                            self.main_menu_screen.view(&self.state).map(Message::MainMenu)
                        )
                    }
                }
            )
            .push(
                if let Some(video) = self.state.video.as_ref() {
                    control_bar(
                        self.position,
                        video.duration().as_secs_f64(),
                        video.duration().as_secs(),
                        video.volume(),
                        video.paused(),
                        &self.state.btn_struct,
                        &self.state.settings.player_settings
                    )
                } else {
                    control_bar(
                        self.position,
                        0.0,
                        0,
                        1.0,
                        true,
                        &self.state.btn_struct,
                        &self.state.settings.player_settings
                    )
                }
            )
            .into()
    }

    pub fn subscription(&self) -> Subscription<Message> {
        // Runs specific tasks in the background and or "listens" for a specific thing.

        // Listen for specific key presses
        keyboard::listen().filter_map(|event| match event {
            keyboard::Event::KeyPressed {
                key: keyboard::Key::Named(key),
                modifiers, ..
            } => match (key, modifiers) {
                (keyboard::key::Named::Space, _) => Some(Message::TogglePause),
                (keyboard::key::Named::AudioVolumeUp, _) => Some(Message::VolumeSeek(0.1)),
                (keyboard::key::Named::AudioVolumeDown, _) => Some(Message::VolumeSeek(-0.1)),
                _ => None,
            },
            _ => None,
        })
    }
}

// The video player area of the interface
fn video_player_area<'a, 'b>(
    video: &'a Option<Video>,
    btn_struct: &'a DynamicButtons,
    playlist_manager: &'a PlaylistManager,
) -> Container<'b, Message> where 'a: 'b {
    Container::new(
        Stack::new()
            .push(
                if let Some(video) = video.as_ref() {
                    Container::new(
                        VideoPlayer::new(video)
                            .width(Length::Fill)
                            .height(Length::Fill)
                            .content_fit(iced::ContentFit::Contain)
                            .on_end_of_stream(Message::EndOfStream)
                            .on_new_frame(Message::NewFrame)
                    )
                        .align_x(Alignment::Center)
                        .align_y(Alignment::Center)
                        .width(Length::Fill)
                        .height(Length::Fill)
                        .style(video_playing_style)
                } else {
                    // Splash screen when no video is playing
                    Container::new(
                        Image::new(CLIPLAY_LOGO_GREY_ICON)
                    )
                        .align_x(Alignment::Center)
                        .align_y(Alignment::Center)
                        .width(Length::Fill)
                        .height(Length::Fill)
                        .style(splash_screen_style)
                }
            )
            .push(
                // video info layover view
                if video.is_some() {
                    match btn_struct.playlist_info_button.current_state() {
                        DynamicPlaylistInfoBtnState::Closed => {
                            Column::new()
                                .push(
                                    Text::new("")
                                )
                        },
                        DynamicPlaylistInfoBtnState::Open => {
                            Column::new()
                                .push(
                                    Row::new()
                                        .push(Text::new("Playlist: "))
                                        .push(Text::new(playlist_manager.playlist_name()))
                                )
                                .push(
                                    Row::new()
                                        .push(Text::new("Video File: "))
                                        .push(
                                            Text::new(
                                                playlist_manager
                                                    .extract_file_name()
                                                    .unwrap_or_else(|| "<unknown>".to_owned())
                                            )
                                        )
                                )
                                .padding(iced::Padding::new(5.0).right(10.0))
                        }
                    }
                } else {
                    Column::new()
                        .push(
                            Text::new("")
                        )
                }
            )
    )
}

// the controls at the bottom of the interface: main menu, scrub bar, etc.
fn control_bar<'a>(
    scrub_bar_positon: f64,
    upper_scrub_position: f64,
    video_duration: u64,
    current_video_volume: f64,
    is_video_currently_paused: bool,
    btn_struct: &'a DynamicButtons,
    player_settings: &'a PlayerSettings,
) -> Element<'a, Message> {
    Container::new(
        Column::new()
            .push(
                // row for scrub bar and time stamp
                Row::new()
                    .spacing(5)
                    .align_y(Alignment::Center)
                    .padding(iced::Padding::new(5.0).left(10.0).right(10.0))
                    .push(
                        // slider
                        Container::new(
                            Slider::new(
                                0.0..=upper_scrub_position,
                                scrub_bar_positon,
                                Message::VideoSeek,
                            )
                                .step(0.1)
                                .on_release(Message::VideoSeekRelease),
                        )
                    )
                    .push(
                        // Video time stamp
                        Text::new(format!(
                            "{}:{:02}s / {}:{:02}s",
                            scrub_bar_positon as u64 / 60, // current minute marker
                            scrub_bar_positon as u64 % 60, // current second marker
                            video_duration / 60, // video total length, minute marker
                            video_duration % 60, // video total length, second marker
                        ))
                            .width(Length::Fixed(100.0))
                            .align_x(Horizontal::Right),
                    )
            )
            .push(
                // row for main menu, loop/shuffle, player controls, set start/stop points and volume
                Row::new()
                    .spacing(5)
                    .align_y(Vertical::Center)
                    .padding(iced::Padding::new(10.0).top(0.0))
                    .push(
                        // main menu
                        Button::new(
                            match btn_struct.main_menu_button.current_state() {
                                DynamicMainMenuBtnState::Closed => Image::new(MAIN_MENU_CLOSED_ICON).width(32).height(32),
                                DynamicMainMenuBtnState::Open => Image::new(MAIN_MENU_OPEN_ICON).width(32).height(32),
                            }
                        )
                            .on_press(Message::ToggleMainMenu)
                            .style(active_large_button_style)
                    )
                    .push(
                        // Current playlist playing info
                        Button::new("Vid Info") // TODO icon needed. Video with "info" icon
                            .on_press(Message::TogglePlaylistInfo)
                            .style(active_large_button_style)
                    )
                    .push(Space::new().width(Length::Fill))
                    .push(
                        // shuffle and loop keys
                        Container::new(
                            Row::new()
                                .spacing(5)
                                .push(
                                    Button::new(Image::new(SHUFFLE_ICON).width(32).height(32))
                                        .on_press(Message::ToggleShuffle)
                                        .style(
                                            match btn_struct.shuffle_button.current_style() {
                                                StyleState::ActiveStyle => active_large_button_style,
                                                StyleState::InactiveStyle => inactive_large_button_style,
                                            }
                                        )
                                )
                                .push(
                                    Button::new(
                                        match btn_struct.loop_button.current_state() {
                                            DynamicLoopBtnState::LoopAll => Image::new(LOOP_INFINITE_ICON).width(32).height(32),
                                            DynamicLoopBtnState::LoopSingle => Image::new(LOOP_ONE_ICON).width(32).height(32),
                                            DynamicLoopBtnState::LoopOff => Image::new(LOOP_OFF_ICON).width(32).height(32),
                                        }
                                    )
                                        .on_press(Message::ToggleLoop)
                                        .style(
                                            match btn_struct.loop_button.current_style() {
                                                StyleState::ActiveStyle => active_large_button_style,
                                                StyleState::InactiveStyle => inactive_large_button_style,
                                            }
                                        )
                                )
                        )
                    )
                    .push(
                        // back/forward keys, skip back/forward keys, and play/pause/stop buttons
                        Container::new(
                            Row::new()
                                .spacing(5)
                                .push(
                                    Button::new(Image::new(SKIP_BACKWARD_ICON).width(32).height(32))
                                        .on_press(Message::SkipBackward)
                                        .style(player_control_button_style)
                                )
                                .push(
                                    Button::new(Image::new(BACKWARD_ICON).width(32).height(32))
                                        .on_press(Message::Backward(player_settings.skip_backward_value as f64))
                                        .style(player_control_button_style)
                                )
                                .push(
                                    Button::new(
                                        match is_video_currently_paused {
                                            true => Image::new(PLAY_ICON).width(32).height(32),
                                            false => Image::new(PAUSE_ICON).width(32).height(32)
                                        }
                                    )
                                        .on_press(Message::TogglePause)
                                        .style(player_control_button_style),
                                )
                                .push(
                                    Button::new(Image::new(STOP_ICON).width(32).height(32))
                                        .on_press(Message::StopVideo)
                                        .style(player_control_button_style)
                                )
                                .push(
                                    Button::new(Image::new(FORWARD_ICON).width(32).height(32))
                                        .on_press(Message::Forward(player_settings.skip_forward_value as f64))
                                        .style(player_control_button_style)
                                )
                                .push(
                                    Button::new(Image::new(SKIP_FORWARD_ICON).width(32).height(32))
                                        .on_press(Message::SkipForward)
                                        .style(player_control_button_style)
                                )
                        )
                    )
                    .push(Space::new().width(Length::Fill))
                    .push(
                        // volume controls
                        Container::new(
                            Row::new()
                                .spacing(5)
                                .align_y(Vertical::Center)
                                .push(Image::new(VOLUME_ICON).width(32).height(32))
                                .push(
                                    Slider::new(
                                        0.0..=1.5,
                                        current_video_volume,
                                        Message::VolumeSeek
                                    )
                                        .style(volume_slider_style)
                                        .step(0.1)
                                )
                                .push(
                                    Text::new(format!(
                                        "{:.0}%",
                                        current_video_volume * 100.0, // turn the value into a percentage
                                    ))
                                        .width(Length::Fill),
                                )
                        )
                    )
            )
    )
        .style(control_bar_style)
        .into()
}
use crate::ui::{
    styling::icons::{
        logo_icons::CLIPLAY_LOGO_GREY_ICON,
        video_player_icons::*
    },
    styling::container_styles::{
        splash_screen_style,
        control_bar_style,
        video_playing_style
    },
    styling::{
        button_styles::{
            player_control_button_style,
            active_large_button_style,
            inactive_large_button_style,
        },
        slider_styles::volume_slider_style,
    },
    dynamic_buttons::{
        DynamicButtons,
        StyleState,
        dynamic_loop_button::DynamicLoopBtnState,
        dynamic_shuffle_button::DynamicShuffleBtnState,
        dynamic_main_menu_button::DynamicMainMenuBtnState,
    },
    screens::main_menu::{
        MainMenuScreen,
        MainMenuMessages,
    }
};
use crate::utils::{
    load_video_file::load_video_file,
    app_state::AppState,
    io_utils::{
        app_directory_path::app_directory_path,
        create_application_directory,
    },
    io_utils::app_settings_data_struct::{
        AppSettings,
        PlayerSettings
    },
    playlist_manager::PlaylistManager,
};
use iced::{
    keyboard,
    Element,
    Length,
    Subscription,
    Task,
    alignment::{Alignment, Horizontal, Vertical},
    widget::{Button, Column, Container, Image, Row, Slider, Text, Space},
};
use iced_video_player::{VideoPlayer};
use std::time::Duration;

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

        let video_file = match loading_process {
            NextVideoLoadingProcess::EndOfVideo => {
                let loop_entire_playlist = self.state.btn_struct.loop_button.is_state_set_to_loop_all();
                let video_file = self.state.playlist_manager.next_file_in_playlist(loop_entire_playlist);
                video_file
            }
            NextVideoLoadingProcess::AfterVideoStopped => {
                let video_file = self.state.playlist_manager.pull_current_index_file_from_playlist();
                video_file
            }
            NextVideoLoadingProcess::SkipForward => {
                let video_file = self.state.playlist_manager.next_file_in_playlist(true);
                video_file
            }
            NextVideoLoadingProcess::SkipBackward => {
                let video_file = self.state.playlist_manager.previous_file_in_playlist();
                video_file
            }
        };

        match video_file {
            Some(video_file) => {
                self.state.video = Some(load_video_file(&video_file));
                self.position = 0.0;

                // Set the new video to single loop if the loop button is set as such
                if self.state.btn_struct.loop_button.is_state_set_to_loop_single() {
                    self.state.video.as_mut().unwrap().set_looping(true);
                } else {
                    self.state.video.as_mut().unwrap().set_looping(false);
                }
            }
            None => {
                self.state.video = None;
                self.position = 0.0;
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
                    DynamicShuffleBtnState::ShuffleOn => println!("Shuffle on"),
                    DynamicShuffleBtnState::ShuffleOff => println!("Shuffle off"),
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
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        Column::new()
            .push(
                match self.state.btn_struct.main_menu_button.current_state() {
                    DynamicMainMenuBtnState::MainMenuClosed => {
                        // TODO future look into. Have the scrub bar update when video is playing and on main menu screen
                        // video view
                        if let Some(video) = self.state.video.as_ref() {
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
                    }
                    DynamicMainMenuBtnState::MainMenuOpen => {
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

// the controls at the bottom of the interface: main menu, scrub bar, etc.
fn control_bar<'a>(
    scrub_bar_positon: f64,
    upper_scrub_position: f64,
    video_duration: u64,
    current_video_volume: f64,
    is_video_currently_paused: bool,
    btn_struct: &DynamicButtons,
    player_settings: &PlayerSettings,
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
                                DynamicMainMenuBtnState::MainMenuClosed => Image::new(MAIN_MENU_CLOSED_ICON).width(32).height(32),
                                DynamicMainMenuBtnState::MainMenuOpen => Image::new(MAIN_MENU_OPEN_ICON).width(32).height(32),
                            }
                        )
                            .on_press(Message::ToggleMainMenu)
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
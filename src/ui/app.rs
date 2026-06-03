use crate::ui::{
    styling::static_images::*,
    styling::{
        btn_inactive_style,
        btn_active_style,
        StyleState
    },
    buttons::{
        ButtonStruct,
        loop_button::LoopStates,
        shuffle_button::ShuffleStates,
        main_menu_button::MainMenuStates,
    },
    main_menu_screen::{
        MainMenuScreen,
        MainMenuMessages,
    }
};
use crate::utils::{
    functions::load_video_file,
    app_state::AppState,
};
use iced::{
    keyboard,
    Element,
    Length,
    Subscription,
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
    position: f64, // TODO might have to put this into the AppState to allow changes to it
    dragging: bool,
    state: AppState,
    main_menu_screen: MainMenuScreen,
}

impl App {

    // Struct methods; IE, methods private to the struct.
    fn load_next_video(&mut self) {
        let loop_entire_playlist = self.state.btn_struct.loop_button.is_state_set_to_loop_all();
        let video_file = self.state.playlist_manager.next_file_in_playlist(loop_entire_playlist);

        match video_file {
            Some(video_file) => {
                self.state.video = Some(load_video_file(&video_file));
                self.position = 0.0;
            }
            None => {
                println!("reach end of playlist")
            }
        }
    }

    // Iced methods; IE, methods used by the Iced crate
    pub fn new() -> Self {
        Self {
            position: 0.0,
            dragging: false,
            state: AppState::default(),
            main_menu_screen: MainMenuScreen::new(),
        }
    }
    pub fn title(&self) -> String {
        // The title of the GUI; shows at the top
        "Cliplay - Video Player".to_string()
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::TogglePause => {
                if let Some(video) = self.state.video.as_mut() {
                    video.set_paused(!video.paused());
                } else {
                    println!("No video selected");
                }
            }
            Message::ToggleLoop => {
                self.state.btn_struct.loop_button.toggle_state_and_style();
                if self.state.btn_struct.loop_button.is_state_set_to_loop_single() {
                    if let Some(video) = self.state.video.as_mut() {
                        video.set_looping(!video.looping());
                    }
                }
            }
            Message::ToggleShuffle => {
                self.state.btn_struct.shuffle_button.toggle_state_and_style();
                match self.state.btn_struct.shuffle_button.current_state() {
                    ShuffleStates::ShuffleOn => println!("Shuffle on"),
                    ShuffleStates::ShuffleOff => println!("Shuffle off"),
                }
            }
            Message::VideoSeek(secs) => {
                self.dragging = true;
                self.state.video.as_mut().unwrap().set_paused(true); // Will remove unwrap once ready
                self.position = secs;
            }
            Message::VideoSeekRelease => {
                self.dragging = false;
                self.state.video
                    .as_mut()
                    .unwrap()// will remove unwrap once ready
                    .seek(Duration::from_secs_f64(self.position), false)
                    .expect("seek");
                self.state.video.as_mut().unwrap().set_paused(false); // will remove unwrap once ready
            }
            Message::Forward(secs) => {
                self.position += secs;
                self.state.video
                    .as_mut()
                    .unwrap() // will remove unwrap once ready
                    .seek(Duration::from_secs_f64(self.position), false)
                    .expect("forward");
            }
            Message::Backward(secs) => {
                self.position -= secs;
                self.state.video
                    .as_mut()
                    .unwrap() // will remove unwrap once ready
                    .seek(Duration::from_secs_f64(self.position), false)
                    .expect("backward");
            }
            Message::SkipForward => {
                self.load_next_video();
            }
            Message::SkipBackward => {
                println!("Skip backward"); // TODO create the logic to go backwards in a playlist
            }
            Message::VolumeSeek(vol) => {
                self.state.video.as_mut().unwrap().set_volume(vol);
            }
            Message::EndOfStream => {
                if !self.state.btn_struct.loop_button.is_state_set_to_loop_single() {
                    self.load_next_video()
                }
            }
            Message::NewFrame => {
                if !self.dragging {
                    self.position = self.state.video.as_ref().unwrap().position().as_secs_f64(); // will remove unwrap when ready
                }
            }
            Message::ToggleMainMenu => {
                self.state.btn_struct.main_menu_button.toggle_state();
            }
            Message::MainMenu(message) => {
                self.main_menu_screen.update(message, &mut self.state);
            }
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        Column::new()
            .push(
                match self.state.btn_struct.main_menu_button.current_state() {
                    MainMenuStates::MainMenuClosed => {
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
                        } else {
                            // Splash screen when no video is playing
                            Container::new(
                                Image::new(CLIPLAY_LOGO_GREY_IMAGE)
                            )
                                .align_x(Alignment::Center)
                                .align_y(Alignment::Center)
                                .width(Length::Fill)
                                .height(Length::Fill)
                        }
                    }
                    MainMenuStates::MainMenuOpen => {
                        Container::new(
                            self.main_menu_screen.view().map(Message::MainMenu)
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
                        self.state.btn_struct,
                    )
                } else {
                    control_bar(
                        self.position,
                        0.0,
                        0,
                        1.0,
                        false,
                        self.state.btn_struct,
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
    btn_struct: ButtonStruct,
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
                                MainMenuStates::MainMenuClosed => Image::new(MAIN_MENU_CLOSED_IMAGE).width(32).height(32),
                                MainMenuStates::MainMenuOpen => Image::new(MAIN_MENU_OPEN_IMAGE).width(32).height(32),
                            }
                        )
                            .on_press(Message::ToggleMainMenu)
                            .style(btn_active_style)
                    )
                    .push(Space::new().width(Length::Fill))
                    .push(
                        // shuffle and loop keys
                        Container::new(
                            Row::new()
                                .spacing(5)
                                .push(
                                    Button::new(Image::new(SHUFFLE_IMAGE).width(32).height(32))
                                        .on_press(Message::ToggleShuffle)
                                        .style(
                                            match btn_struct.shuffle_button.current_style() {
                                                StyleState::ActiveStyle => btn_active_style,
                                                StyleState::InactiveStyle => btn_inactive_style,
                                            }
                                        )
                                )
                                .push(
                                    Button::new(
                                        match btn_struct.loop_button.current_state() {
                                            LoopStates::LoopAll => Image::new(LOOP_INFINITE_IMAGE).width(32).height(32),
                                            LoopStates::LoopSingle => Image::new(LOOP_ONE_IMAGE).width(32).height(32),
                                            LoopStates::LoopOff => Image::new(LOOP_OFF_IMAGE).width(32).height(32),
                                        }
                                    )
                                        .on_press(Message::ToggleLoop)
                                        .style(
                                            match btn_struct.loop_button.current_style() {
                                                StyleState::ActiveStyle => btn_active_style,
                                                StyleState::InactiveStyle => btn_inactive_style,
                                            }
                                        )
                                )
                        )
                    )
                    .push(
                        // back/forward keys, skip back/forward keys, and play/pause buttons
                        Container::new(
                            Row::new()
                                .spacing(5)
                                .push(
                                    Button::new(Image::new(SKIP_BACKWARD_IMAGE).width(32).height(32))
                                        .on_press(Message::SkipBackward)
                                        .style(btn_active_style)
                                )
                                .push(
                                    Button::new(Image::new(BACKWARD_IMAGE).width(32).height(32))
                                        .on_press(Message::Backward(10.0))
                                        .style(btn_active_style)
                                )
                                .push(
                                    Button::new(
                                        match is_video_currently_paused { // will remove unwrap when ready
                                            true => Image::new(PLAY_IMAGE).width(32).height(32),
                                            false => Image::new(PAUSE_IMAGE).width(32).height(32)
                                        }
                                    )
                                        .on_press(Message::TogglePause)
                                        .style(btn_active_style),
                                )
                                .push(
                                    Button::new(Image::new(FORWARD_IMAGE).width(32).height(32))
                                        .on_press(Message::Forward(10.0))
                                        .style(btn_active_style)
                                )
                                .push(
                                    Button::new(Image::new(SKIP_FORWARD_IMAGE).width(32).height(32))
                                        .on_press(Message::SkipForward)
                                        .style(btn_active_style)
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
                                .push(Image::new(VOLUME_IMAGE).width(32).height(32))
                                .push(
                                    Slider::new(
                                        0.0..=1.5,
                                        current_video_volume,
                                        Message::VolumeSeek
                                    )
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
    ).into()
}
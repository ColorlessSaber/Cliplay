mod styling;
mod buttons;

use styling::static_images::{
    MAIN_MENU_IMAGE, PLAY_IMAGE, PAUSE_IMAGE, FORWARD_IMAGE, BACKWARD_IMAGE, LOOP_IMAGE,
    SHUFFLE_IMAGE, VOLUME_IMAGE
};
use styling::{
    btn_inactive_style,
    btn_active_style,
    StyleState
};
use buttons::{ButtonStruct};
use crate::utils::{
    functions::load_video_file,
    playlist_manager::{PlayListManager},
};
use iced::{
    Element,
    Length,
    alignment::{Alignment, Horizontal, Vertical},
    widget::{Button, Column, Container, Image, Row, Slider, Text, Space},
};
use iced_video_player::{Video, VideoPlayer};
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
    VolSeek(f64),
    EndOfStream,
    NewFrame,
    MainMenu,
}

pub struct App {
    video: Video,
    position: f64,
    dragging: bool,
    btn_struct: ButtonStruct,
    playlist_manager: PlayListManager,
    loop_on: bool,
}

impl Default for App {
    fn default() -> Self {
        App {
            video: load_video_file("/home/admin/Videos/Misc Videos/Zenless Zone Zero/ZZZ WIT Studio Animation.mkv"),
            position: 0.0,
            dragging: false,
            btn_struct: ButtonStruct::default(),
            playlist_manager: PlayListManager::default(),
            loop_on: false,
        }
    }
}

impl App {

    // Iced methods
    pub fn update(&mut self, message: Message) {
        match message {
            Message::TogglePause => {
                self.video.set_paused(!self.video.paused());
            }
            Message::ToggleLoop => {
                self.btn_struct.loop_button.toggle_style();
                self.loop_on = !self.loop_on;
            }
            Message::ToggleShuffle => {
                self.btn_struct.shuffle_button.toggle_style();
                println!("Toggle Shuffle");
            }
            Message::VideoSeek(secs) => {
                self.dragging = true;
                self.video.set_paused(true);
                self.position = secs;
            }
            Message::VideoSeekRelease => {
                self.dragging = false;
                self.video
                    .seek(Duration::from_secs_f64(self.position), false)
                    .expect("seek");
                self.video.set_paused(false);
            }
            Message::Forward(secs) => {
                self.position += secs;
                self.video
                    .seek(Duration::from_secs_f64(self.position), false)
                    .expect("seek");
            }
            Message::Backward(secs) => {
                self.position -= secs;
                self.video
                    .seek(Duration::from_secs_f64(self.position), false)
                    .expect("seek");
            }
            Message::VolSeek(vol) => {
                self.video.set_volume(vol);
            }
            Message::EndOfStream => {
                loop {
                    let video_file = self.playlist_manager.next_file_in_playlist();
                    match video_file {
                        Some(video_file) => {
                            self.video = load_video_file(video_file.as_str());
                            self.position = 0.0;
                            break;
                        }
                        None => {
                            if self.loop_on {
                                self.playlist_manager.reset_index();
                            } else {
                                println!("Reached end of playlist");
                                break;
                            }
                        }
                    }
                }
            }
            Message::NewFrame => {
                if !self.dragging {
                    self.position = self.video.position().as_secs_f64();
                }
            }
            Message::MainMenu => {
                println!("Main Menu");
            }
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        Column::new()
            .push(
                // video view
                Container::new(
                    VideoPlayer::new(&self.video)
                        .width(Length::Fill)
                        .height(Length::Fill)
                        .content_fit(iced::ContentFit::Contain)
                        .on_end_of_stream(Message::EndOfStream)
                        .on_new_frame(Message::NewFrame),
                )
                    .align_x(Alignment::Center)
                    .align_y(Alignment::Center)
                    .width(Length::Fill)
                    .height(Length::Fill),
            )
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
                                0.0..=self.video.duration().as_secs_f64(),
                                self.position,
                                Message::VideoSeek,
                            )
                                .step(0.1)
                                .on_release(Message::VideoSeekRelease),
                        ),
                    )
                    .push(
                        // Video time stamp
                        Text::new(format!(
                            "{}:{:02}s / {}:{:02}s",
                            self.position as u64 / 60, // current minute marker
                            self.position as u64 % 60, // current second marker
                            self.video.duration().as_secs() / 60, // video total length, minute marker
                            self.video.duration().as_secs() % 60, // video total length, second marker
                        ))
                            .width(Length::Fixed(100.0))
                            .align_x(Horizontal::Right),
                    ),
            )
            .push(
                // row for main menu, loop/shuffle, player controls, set start/stop points and volume
                Row::new()
                    .spacing(5)
                    .align_y(Vertical::Center)
                    .padding(iced::Padding::new(10.0).top(0.0))
                    .push(
                        // main menu
                        Button::new(Image::new(MAIN_MENU_IMAGE).width(32).height(32))
                            .on_press(Message::MainMenu)
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
                                            match self.btn_struct.shuffle_button.current_style {
                                                StyleState::ActiveStyle => btn_active_style,
                                                StyleState::InactiveStyle => btn_inactive_style,
                                            }
                                        )
                                )
                                .push(
                                    Button::new(Image::new(LOOP_IMAGE).width(32).height(32))
                                        .on_press(Message::ToggleLoop)
                                        .style(
                                            match self.btn_struct.loop_button.current_style {
                                                StyleState::ActiveStyle => btn_active_style,
                                                StyleState::InactiveStyle => btn_inactive_style,
                                            }
                                        )
                                ),
                        ),
                    )
                    .push(
                        // back, play/pause, forward keys
                        Container::new(
                            Row::new()
                                .spacing(5)
                                .push(
                                    Button::new(Image::new(BACKWARD_IMAGE).width(32).height(32))
                                        .on_press(Message::Backward(10.0))
                                        .style(btn_active_style)
                                )
                                .push(
                                    Button::new(
                                        match self.video.paused() {
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
                                ),
                        ),
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
                                        self.video.volume(),
                                        Message::VolSeek
                                    )
                                        .step(0.1)
                                )
                                .push(
                                    Text::new(format!(
                                        "{:.0}%",
                                        self.video.volume() * 100.0, // turn the value into a percentage
                                    ))
                                        .width(Length::Fill),
                                ),
                        ),
                    ),
            )
            .into()
    }
}
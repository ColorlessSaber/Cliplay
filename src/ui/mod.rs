mod styling;
use styling::static_images::{
    MAIN_MENU_IMAGE, PLAY_IMAGE, PAUSE_IMAGE, FORWARD_IMAGE, BACKWARD_IMAGE, LOOP_IMAGE,
    SHUFFLE_IMAGE, VOLUME_IMAGE
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
    Seek(f64),
    SeekRelease,
    VolChange(f64),
    EndOfStream,
    NewFrame,
}

pub struct App {
    video: Video,
    position: f64,
    dragging: bool,
}

impl Default for App {
    fn default() -> Self {
        App {
            video: Video::new(
                &url::Url::from_file_path(
                    std::path::PathBuf::from(file!())
                        .parent()
                        .unwrap()
                        .join("/home/admin/Videos/Misc Videos/Zenless Zone Zero/ZZZ WIT Studio Animation.mkv")
                        .canonicalize()
                        .unwrap(),
                )
                    .unwrap(),
            )
                .unwrap(),
            position: 0.0,
            dragging: false,
        }
    }
}

impl App {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::TogglePause => {
                self.video.set_paused(!self.video.paused());
            }
            Message::ToggleLoop => {
                self.video.set_looping(!self.video.looping());
            }
            Message::Seek(secs) => {
                self.dragging = true;
                self.video.set_paused(true);
                self.position = secs;
            }
            Message::SeekRelease => {
                self.dragging = false;
                self.video
                    .seek(Duration::from_secs_f64(self.position), false)
                    .expect("seek");
                self.video.set_paused(false);
            }
            Message::VolChange(vol) => {
                let current_vol = self.video.volume();
                let new_vol = current_vol + vol;
                // limit volume between 0% and 150% (upper bound set to 1.6 to get 150%)
                if new_vol >= 0.0 && new_vol <= 1.6 {
                    self.video.set_volume(new_vol);
                }
            }
            Message::EndOfStream => {
                println!("end of stream");
            }
            Message::NewFrame => {
                if !self.dragging {
                    self.position = self.video.position().as_secs_f64();
                }
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
                                Message::Seek,
                            )
                                .step(0.1)
                                .on_release(Message::SeekRelease),
                        ),
                    )
                    .push(
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
                    )
                    .push(Space::new().width(Length::Fill))
                    .push(
                        // shuffle and loop keys
                        Container::new(
                            Row::new()
                                .spacing(5)
                                .push(Button::new(Image::new(SHUFFLE_IMAGE).width(32).height(32)))
                                .push(
                                    Button::new(Image::new(LOOP_IMAGE).width(32).height(32))
                                        .on_press(Message::ToggleLoop),
                                ),
                        ),
                    )
                    .push(
                        // back, play/pause, forward keys
                        Container::new(
                            Row::new()
                                .spacing(5)
                                .push(Button::new(Image::new(BACKWARD_IMAGE).width(32).height(32)))
                                .push(
                                    Button::new(if self.video.paused() {
                                        Image::new(PLAY_IMAGE).width(32).height(32)
                                    } else {
                                        Image::new(PAUSE_IMAGE).width(32).height(32)
                                    })
                                        .on_press(Message::TogglePause),
                                )
                                .push(Button::new(Image::new(FORWARD_IMAGE).width(32).height(32))),
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
                                    Button::new(Text::new("-"))
                                        .width(50.0)
                                        .on_press(Message::VolChange(-0.1)),
                                )
                                .push(
                                    Button::new(Text::new("+"))
                                        .width(50.0)
                                        .on_press(Message::VolChange(0.1)),
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
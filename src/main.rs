use iced::{
    Element,
    widget::{Button, Column, Container, Row, Slider, Text, Image},
};
use iced_video_player::{Video, VideoPlayer};
use std::time::Duration;

// static images
static PLAY_IMAGE: &str = "icons/play.png";
static PAUSE_IMAGE: &str = "icons/pause.png";
static FORWARD_IMAGE: &str = "icons/forward.png";
static BACKWARD_IMAGE: &str = "icons/backward.png";
static LOOP_IMAGE: &str = "icons/loop.png";
static SHUFFLE_IMAGE: &str = "icons/shuffle.png";

fn main() -> iced::Result {
    iced::run(App::update, App::view)
}

#[derive(Clone, Debug)]
enum Message {
    TogglePause,
    ToggleLoop,
    Seek(f64),
    SeekRelease,
    VolChange(f64),
    EndOfStream,
    NewFrame,
}

struct App {
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
    fn update(&mut self, message: Message) {
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

    fn view(&self) -> Element<'_, Message> {
        Column::new()
            .push(
                // video view
                Container::new(
                    VideoPlayer::new(&self.video)
                        .width(iced::Length::Fill)
                        .height(iced::Length::Fill)
                        .content_fit(iced::ContentFit::Contain)
                        .on_end_of_stream(Message::EndOfStream)
                        .on_new_frame(Message::NewFrame),
                )
                .align_x(iced::Alignment::Center)
                .align_y(iced::Alignment::Center)
                .width(iced::Length::Fill)
                .height(iced::Length::Fill),
            )
            .push(
                // row for scrub bar and time stamp
                Row::new()
                    .spacing(0)
                    .align_y(iced::Alignment::Center)
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
                        .width(iced::Length::Fixed(100.0))
                        .align_x(iced::alignment::Horizontal::Right),
                    ),
            )
            .push(
                // row for main menu, loop/shuffle, player controls, set start/stop points and volume
                Row::new()
                    .spacing(5)
                    .align_y(iced::alignment::Vertical::Center)
                    .padding(iced::Padding::new(10.0).top(0.0))
                    .push(
                        // main menu
                        Button::new(Text::new("Main Menu"))
                            .width(80.0)
                            .padding(iced::Padding::new(5.0).right(10.0)),
                    )
                    .push(
                        // shuffle and loop keys
                        Container::new(
                            Row::new()
                                .spacing(5)
                                .padding(iced::Padding::new(5.0).left(10.0).right(10.0))
                                .push(
                                    Button::new(
                                        Image::new(SHUFFLE_IMAGE)
                                            .width(32)
                                            .height(32),
                                    )
                                )
                                .push(
                                    Button::new(
                                        Image::new(LOOP_IMAGE)
                                            .width(32)
                                            .height(32)
                                    )
                                .width(80.0)
                                .on_press(Message::ToggleLoop),
                            ),
                        ),
                    )
                    .push(
                        // back, play/pause, forward keys
                        Container::new(
                            Row::new()
                                .spacing(5)
                                .padding(iced::Padding::new(5.0).left(10.0).right(10.0))
                                .push(
                                    Button::new(
                                        Image::new(BACKWARD_IMAGE)
                                            .width(32)
                                            .height(32)
                                    )
                                )
                                .push(
                                    Button::new(
                                        if self.video.paused() {
                                            Image::new(PLAY_IMAGE)
                                                .width(32)
                                                .height(32)
                                        } else {
                                            Image::new(PAUSE_IMAGE)
                                                .width(32)
                                                .height(32)
                                        }
                                    )
                                    .width(64)
                                    .on_press(Message::TogglePause),
                                )
                                .push(
                                    Button::new(
                                        Image::new(FORWARD_IMAGE)
                                            .width(32)
                                            .height(32)
                                    )
                                ),
                        ),
                    )
                    .push(
                        Row::new()
                            .spacing(5)
                            .align_y(iced::alignment::Vertical::Center)
                            .padding(iced::Padding::new(10.0).right(0.0))
                            .push(
                                Button::new(Text::new("Vol+"))
                                    .width(50.0)
                                    .on_press(Message::VolChange(0.1)),
                            )
                            .push(
                                Button::new(Text::new("Vol-"))
                                    .width(50.0)
                                    .on_press(Message::VolChange(-0.1)),
                            )
                            .push(
                                Text::new(format!(
                                    "{:.0}%",
                                    self.video.volume() * 100.0, // turn the value into a percentage
                                ))
                                .width(iced::Length::Fill),
                            ),
                    ),
            )
            .into()
    }
}

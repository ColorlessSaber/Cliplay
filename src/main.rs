use iced::{
    Element,
    widget::{Button, Column, Container, Row, Slider, Text},
};
use iced_video_player::{Video, VideoPlayer};
use std::time::Duration;

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
                Container::new(
                    Slider::new(
                        0.0..=self.video.duration().as_secs_f64(),
                        self.position,
                        Message::Seek,
                    )
                        .step(0.1)
                        .on_release(Message::SeekRelease),
                )
                    .padding(iced::Padding::new(5.0).left(10.0).right(10.0)),
            )
            .push(
                Row::new()
                    .spacing(5)
                    .align_y(iced::alignment::Vertical::Center)
                    .padding(iced::Padding::new(10.0).top(0.0))
                    .push(
                        Button::new(Text::new(if self.video.paused() {
                            "Play"
                        } else {
                            "Pause"
                        }))
                            .width(80.0)
                            .on_press(Message::TogglePause),
                    )
                    .push(
                        Button::new(Text::new(if self.video.looping() {
                            "Disable Loop"
                        } else {
                            "Enable Loop"
                        }))
                            .width(120.0)
                            .on_press(Message::ToggleLoop),
                    )
                    .push(
                        Row::new()
                            .spacing(5)
                            .align_y(iced::alignment::Vertical::Center)
                            .padding(iced::Padding::new(10.0).right(0.0))
                            .push(
                                Button::new(Text::new("Vol+"))
                                    .width(50.0)
                                    .on_press(Message::VolChange(0.1))
                            )
                            .push(
                                Button::new(Text::new("Vol-"))
                                    .width(50.0)
                                    .on_press(Message::VolChange(-0.1))
                            )
                            .push(Text::new(format!(
                                "{:.0}%",
                                self.video.volume() * 100.0, // turn the value into a percentage
                            ))
                                .width(iced::Length::Fill)
                            )
                    )
                    .push(
                        Text::new(format!(
                            "{}:{:02}s / {}:{:02}s",
                            self.position as u64 / 60, // current minute marker
                            self.position as u64 % 60, // current second marker
                            self.video.duration().as_secs() / 60, // video total length, minute marker
                            self.video.duration().as_secs() % 60, // video total length, second marker
                        ))
                            .width(iced::Length::Fill)
                            .align_x(iced::alignment::Horizontal::Right),
                    ),
            )
            .into()
    }
}
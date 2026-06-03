/*
Using the rfd crate, creates a file dialog window to allow user to select file(s)
 */
use iced::{
    widget::{Button, Text},
    Task, Element,
};
use iced::widget::Container;
use rfd::AsyncFileDialog;


pub struct FileDialog {}

#[derive(Debug, Clone)]
pub enum Message {
    OpenFileDialog,
    FileSelected(Option<String>),
}

impl FileDialog {

    pub fn new() -> (Self, Task<Message>) {
        (
            Self {},
            Task::none(),
        )
    }

    pub fn title(&self) -> String {
        String::from("File Picker Example (Iced 0.14)")
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::OpenFileDialog => {
                Task::perform(
                    async {
                        AsyncFileDialog::new()
                            .add_filter("video", &[".mp4", ".mkv", ".m4v"])
                            .pick_file()
                            .await
                            .map(|handle| handle.path().to_string_lossy().into_owned())
                    },
                    Message::FileSelected,
                )
            }
            Message::FileSelected(path) => {
                if let Some(path) = path {
                    println!("Selected file: {}", path);
                }
                Task::none()
            }
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        Container::new(
            Button::new(Text::new("Open File"))
                .on_press(Message::OpenFileDialog)
        ).into()
    }
}
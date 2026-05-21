mod ui;
mod utils;

use ui::app::{App};

fn main() -> iced::Result {
    iced::run(App::update, App::view)
}

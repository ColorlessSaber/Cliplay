mod ui;
mod utils;

use ui::App;

fn main() -> iced::Result {
    iced::run(App::update, App::view)
}

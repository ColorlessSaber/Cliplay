mod ui;
mod functions;

use ui::App;

fn main() -> iced::Result {
    iced::run(App::update, App::view)
}

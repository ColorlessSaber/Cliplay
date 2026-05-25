mod ui;
mod utils;

use ui::app::{App};

fn main() -> iced::Result {
    iced::application(App::new, App::update, App::view)
        .subscription(App::subscription)
        .title(App::title)
        .window_size((1024.0, 700.0))
        .run()
}

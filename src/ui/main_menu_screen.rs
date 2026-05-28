use iced::{
    Element,
    widget::{Button, Column, Image,},
};
use crate::ui::styling::btn_active_style;
use crate::ui::styling::static_images::{
    SELECT_SINGLE_VID_IMAGE,
    PLAYLISTS_IMAGE
};

#[derive(Debug, Copy, Clone)]
pub enum MainMenuMessages {
    SelectVideo,
    PlaylistsMenu,
}

pub struct MainMenuScreen {}

impl MainMenuScreen {

    pub fn new() -> Self {
        Self {}
    }

    pub fn update(&self, message: MainMenuMessages) {
        match message {
            MainMenuMessages::SelectVideo => {println!("Video selected");}
            MainMenuMessages::PlaylistsMenu => {println!("Playlists selected");}
        }
    }

    pub fn view<'a, 'b>(&'a self) -> Element<'b, MainMenuMessages>
    where 'a: 'b
    {
        Column::new()
            .push(
                Button::new(Image::new(SELECT_SINGLE_VID_IMAGE).width(32).height(32))
                    .on_press(MainMenuMessages::SelectVideo)
                    .style(btn_active_style)
            )
            .push(
                Button::new(Image::new(PLAYLISTS_IMAGE).width(32).height(32))
                    .on_press(MainMenuMessages::PlaylistsMenu)
                    .style(btn_active_style)
            ).into()
    }
}
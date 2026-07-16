use iced::{
    Element,
    Task,
    Length,
    widget::{
        Text,
        Column,
        Container,
        Space,
        Button,
    },
};
use iced::widget::Row;
use crate::utils::{
    app_state::AppState,
    io_utils::{
        LoadError,
        SaveError,
    },
    io_utils::app_settings_data_struct::AppSettings
};
use crate::ui::{
    styling::{
        container_styles::{
            main_section_style
        }
    }
};

#[derive(Debug, Clone)]
pub enum SettingsMenuMessage {
    Save,
    Cancel,
    ValidateSettingsDif,
}

pub struct SettingsMenu {
    settings_have_changed: bool, // keep track of when user changes the settings
}

impl SettingsMenu {
    pub fn new() -> Self {
        Self {
            settings_have_changed: false,
        }
    }

    pub fn update(&mut self, message: SettingsMenuMessage, state: &mut AppState) -> Task<SettingsMenuMessage> {
        match message {
            SettingsMenuMessage::Save => {
                self.settings_have_changed = false;
                println!("Saving settings");

                Task::none()
            }
            SettingsMenuMessage::Cancel => {
                self.settings_have_changed = false;
                println!("Canceling settings");

                Task::none()
            }
            SettingsMenuMessage::ValidateSettingsDif => {
                self.settings_have_changed = true;

                Task::none()
            }
        }
    }

    pub fn view(&self, state: &AppState) -> Element<'_, SettingsMenuMessage> {

        Container::new(
            Column::new()
                .spacing(10)
                .width(Length::Fill)
                .push(Text::new("Settings"))
                .push(Space::new().height(Length::Fill))
                .push(
                    Row::new()
                        .spacing(10)
                        .width(Length::Fill)
                        .push(Space::new().width(Length::Fill))
                        .push(
                            Button::new(Text::new("Save"))
                            .on_press(SettingsMenuMessage::Save)
                        )
                        .push(
                            Button::new(Text::new("Cancel"))
                            .on_press(SettingsMenuMessage::Cancel)
                        )
                )
        )
            .padding(10)
            .height(Length::Fill)
            .style(main_section_style)
            .into()

    }
}
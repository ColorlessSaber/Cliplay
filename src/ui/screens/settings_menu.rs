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
        Row,
    },
};
use iced_aw::{
    number_input,
    widget::{
        LabeledFrame,
    }
};
use crate::utils::{
    app_state::AppState,
    io_utils::{
        SaveError,
        app_settings_data_struct::{
            AppSettings,
            PlayerSettings
        },
    },
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
    SettingsSaved(Result<(), SaveError>),
    Cancel,
    UpdateSkipForwardValue(usize),
    UpdateSkipBackwardValue(usize),
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
                Task::perform(
                    AppSettings{
                        version: state.settings.version.clone(),
                        player_settings: PlayerSettings{
                            skip_forward_value: state.settings.player_settings.skip_forward_value,
                            skip_backward_value: state.settings.player_settings.skip_backward_value,
                        }
                    }.save(),
                    SettingsMenuMessage::SettingsSaved
                )
            }
            SettingsMenuMessage::SettingsSaved(_result) => {
                self.settings_have_changed = false;

                Task::none()
            }
            SettingsMenuMessage::Cancel => {
                self.settings_have_changed = false;
                let settings_data = AppSettings::load().ok().unwrap();
                state.settings = settings_data;

                Task::none()
            }
            SettingsMenuMessage::UpdateSkipForwardValue(val) => {
                self.settings_have_changed = true;
                state.settings.player_settings.skip_forward_value = val;

                Task::none()
            }
            SettingsMenuMessage::UpdateSkipBackwardValue(val) => {
                self.settings_have_changed = true;
                state.settings.player_settings.skip_backward_value = val;

                Task::none()
            }
        }
    }

    pub fn view(&self, state: &AppState) -> Element<'_, SettingsMenuMessage> {

        Container::new(
            Column::new()
                .spacing(10)
                .width(Length::Fill)
                .push(
                    player_settings_layout(&state.settings.player_settings)
                )
                .push(Space::new().height(Length::Fill))
                .push(
                    Row::new()
                        .spacing(10)
                        .width(Length::Fill)
                        .push(Space::new().width(Length::Fill))
                        .push(
                            match self.settings_have_changed {
                                true => {Text::new("Unsaved Changes!")}
                                false => {Text::new("")}
                            }
                        )
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

fn player_settings_layout<'a>(
    player_settings: &PlayerSettings,
) -> Element<'a, SettingsMenuMessage> {
    LabeledFrame::new(
        "Player Settings",
        Column::new()
            .spacing(10)
            .width(Length::Fill)
            .push(
                Row::new()
                    .spacing(10)
                    .width(Length::Fill)
                    .push(Text::new("Skip Forward Value:"))
                    .push(
                        number_input(
                            &player_settings.skip_forward_value,
                            5..=30,
                            SettingsMenuMessage::UpdateSkipForwardValue,
                        )
                            .step(1)
                    )
            )
            .push(
                Row::new()
                    .spacing(10)
                    .width(Length::Fill)
                    .push(Text::new("Skip Backward Value:"))
                    .push(
                        number_input(
                            &player_settings.skip_backward_value,
                            5..=30,
                            SettingsMenuMessage::UpdateSkipBackwardValue,
                        )
                            .step(1)
                    )
            ),
    )
        .width(Length::Fill)
        .into()
}
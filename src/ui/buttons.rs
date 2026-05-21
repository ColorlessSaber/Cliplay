use crate::ui::styling::{
    StyleState,
};

// holds information about each button that have dynamic information
pub struct ButtonStruct {
    pub loop_button: ButtonInfo,
    pub shuffle_button: ButtonInfo,
}

impl Default for ButtonStruct {
    fn default() -> Self {
        Self {
            loop_button: ButtonInfo::default(),
            shuffle_button: ButtonInfo::default(),
        }
    }
}

// Keeps track of the different states of a buttons. IE, On or off.
#[derive(Copy, Clone)]
pub enum ButtonState {
    OffState,
    OnState,
}

// Holds information about a button widget: style state, etc.
pub struct ButtonInfo {
    pub current_style: StyleState,
    current_state: ButtonState,
}

impl Default for ButtonInfo {
    fn default() -> Self {
        Self {
            current_style: StyleState::InactiveStyle,
            current_state: ButtonState::OffState,
        }
    }
}

impl ButtonInfo {

    pub fn state(&self) -> ButtonState {
        self.current_state
    }

    pub fn toggle_state(&mut self) {
        match self.current_state {
            ButtonState::OffState => self.current_state = ButtonState::OnState,
            ButtonState::OnState => self.current_state = ButtonState::OffState,
        }
    }

    pub fn toggle_style(&mut self) {
        match self.current_style {
            StyleState::InactiveStyle => {
                self.current_style = StyleState::ActiveStyle;
            }
            StyleState::ActiveStyle => {
                self.current_style = StyleState::InactiveStyle;
            }
        }
    }
}
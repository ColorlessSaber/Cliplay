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


// Holds information about a button widget: style state, etc.
pub struct ButtonInfo {
    pub current_style: StyleState,
}

impl Default for ButtonInfo {
    fn default() -> Self {
        Self {
            current_style: StyleState::InactiveStyle,
        }
    }
}

impl ButtonInfo {

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
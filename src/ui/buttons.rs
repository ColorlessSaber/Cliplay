use crate::ui::buttons::loop_button::LoopButton;
use crate::ui::buttons::shuffle_button::ShuffleButton;

// holds information about each button that have dynamic information
pub struct ButtonStruct {
    pub loop_button: LoopButton,
    pub shuffle_button: ShuffleButton,
}

impl Default for ButtonStruct {
    fn default() -> Self {
        Self {
            loop_button: LoopButton::default(),
            shuffle_button: ShuffleButton::default(),
        }
    }
}

// the struct, enum and methods for the loop button
pub mod loop_button {
    use crate::ui::styling::StyleState;

    #[derive(Copy, Clone)]
    pub enum LoopStates {
        LoopAll,
        LoopSingle,
        LoopOff,
    }

    pub struct LoopButton {
        pub current_style: StyleState,
        current_state: LoopStates,
    }

    impl Default for LoopButton {
        fn default() -> Self {
            Self {
                current_style: StyleState::InactiveStyle,
                current_state: LoopStates::LoopOff,
            }
        }
    }

    impl LoopButton {
        pub fn state(&self) -> LoopStates {
            self.current_state
        }

        pub fn toggle_state_and_style(&mut self) {
            match self.current_state {
                LoopStates::LoopAll => {
                    self.current_state = LoopStates::LoopSingle;
                }
                LoopStates::LoopSingle => {
                    self.current_state = LoopStates::LoopOff;
                }
                LoopStates::LoopOff => {
                    self.current_state = LoopStates::LoopAll;
                }
            }
            match self.current_state {
                LoopStates::LoopAll => self.current_style = StyleState::ActiveStyle,
                LoopStates::LoopSingle => self.current_style = StyleState::ActiveStyle,
                LoopStates::LoopOff => self.current_style = StyleState::InactiveStyle,
            }
        }

        pub fn is_state_set_to_loop_all(&self) -> bool {
            match self.current_state {
                LoopStates::LoopAll => true,
                _ => false,
            }
        }

        pub fn is_state_set_to_loop_single(&self) -> bool {
            match self.current_state {
                LoopStates::LoopSingle => true,
                _ => false,
            }
        }

    }
}

// the struct, enum and methods for the shuffle button
pub mod shuffle_button {
    use crate::ui::styling::StyleState;

    #[derive(Copy, Clone)]
    pub enum ShuffleStates {
        ShuffleOn,
        ShuffleOff,
    }

    pub struct ShuffleButton {
        pub current_style: StyleState,
        current_state: ShuffleStates,
    }

    impl Default for ShuffleButton {
        fn default() -> Self {
            Self {
                current_style: StyleState::InactiveStyle,
                current_state: ShuffleStates::ShuffleOff
            }
        }
    }

    impl ShuffleButton {
        pub fn state(&self) -> ShuffleStates {
            self.current_state
        }

        pub fn toggle_state_and_style(&mut self) {
            match self.current_state {
                ShuffleStates::ShuffleOn => {
                    self.current_state = ShuffleStates::ShuffleOff;
                }
                ShuffleStates::ShuffleOff => {
                    self.current_state = ShuffleStates::ShuffleOn;
                }
            }
            match self.current_state {
                ShuffleStates::ShuffleOn => self.current_style = StyleState::ActiveStyle,
                ShuffleStates::ShuffleOff => self.current_style = StyleState::InactiveStyle,
            }
        }
    }
}
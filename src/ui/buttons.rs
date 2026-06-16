use crate::ui::buttons::loop_button::LoopButton;
use crate::ui::buttons::shuffle_button::ShuffleButton;
use crate::ui::buttons::main_menu_button::MainMenuButton;

// holds information about each button that have dynamic information
#[derive(Copy, Clone)]
pub struct ButtonStruct {
    pub loop_button: LoopButton,
    pub shuffle_button: ShuffleButton,
    pub main_menu_button: MainMenuButton,
}

impl Default for ButtonStruct {
    fn default() -> Self {
        Self {
            loop_button: LoopButton::default(),
            shuffle_button: ShuffleButton::default(),
            main_menu_button: MainMenuButton::default(),
        }
    }
}

// Module to hold struct, enum and methods for the loop button
pub mod loop_button {
    use crate::ui::styling::StyleState;

    #[derive(Copy, Clone)]
    pub enum LoopBtnStates {
        LoopAll,
        LoopSingle,
        LoopOff,
    }

    #[derive(Copy, Clone)]
    pub struct LoopButton {
        current_style: StyleState,
        current_state: LoopBtnStates,
    }

    impl Default for LoopButton {
        fn default() -> Self {
            Self {
                current_style: StyleState::InactiveStyle,
                current_state: LoopBtnStates::LoopOff,
            }
        }
    }

    impl LoopButton {
        pub fn current_state(&self) -> LoopBtnStates {
            self.current_state
        }

        pub fn current_style(&self) -> StyleState {self.current_style}

        pub fn toggle_state_and_style(&mut self) {
            match self.current_state {
                LoopBtnStates::LoopAll => {
                    self.current_state = LoopBtnStates::LoopSingle;
                }
                LoopBtnStates::LoopSingle => {
                    self.current_state = LoopBtnStates::LoopOff;
                }
                LoopBtnStates::LoopOff => {
                    self.current_state = LoopBtnStates::LoopAll;
                }
            }
            match self.current_state {
                LoopBtnStates::LoopAll | LoopBtnStates::LoopSingle => self.current_style = StyleState::ActiveStyle,
                LoopBtnStates::LoopOff => self.current_style = StyleState::InactiveStyle,
            }
        }

        pub fn is_state_set_to_loop_all(&self) -> bool {
            match self.current_state {
                LoopBtnStates::LoopAll => true,
                _ => false,
            }
        }

        pub fn is_state_set_to_loop_single(&self) -> bool {
            match self.current_state {
                LoopBtnStates::LoopSingle => true,
                _ => false,
            }
        }

    }
}

// Module to hold struct, enum and methods for the shuffle button
pub mod shuffle_button {
    use crate::ui::styling::StyleState;

    #[derive(Copy, Clone)]
    pub enum ShuffleBtnStates {
        ShuffleOn,
        ShuffleOff,
    }

    #[derive(Copy, Clone)]
    pub struct ShuffleButton {
        current_style: StyleState,
        current_state: ShuffleBtnStates,
    }

    impl Default for ShuffleButton {
        fn default() -> Self {
            Self {
                current_style: StyleState::InactiveStyle,
                current_state: ShuffleBtnStates::ShuffleOff
            }
        }
    }

    impl ShuffleButton {
        pub fn current_state(&self) -> ShuffleBtnStates {
            self.current_state
        }

        pub fn current_style(&self) -> StyleState {self.current_style}

        pub fn toggle_state_and_style(&mut self) {
            match self.current_state {
                ShuffleBtnStates::ShuffleOn => {
                    self.current_state = ShuffleBtnStates::ShuffleOff;
                }
                ShuffleBtnStates::ShuffleOff => {
                    self.current_state = ShuffleBtnStates::ShuffleOn;
                }
            }
            match self.current_state {
                ShuffleBtnStates::ShuffleOn => self.current_style = StyleState::ActiveStyle,
                ShuffleBtnStates::ShuffleOff => self.current_style = StyleState::InactiveStyle,
            }
        }
    }
}

// Module to hold the struct, enum and methods for the menu button
pub mod main_menu_button {
    #[derive(Copy, Clone)]
    pub enum MainMenuBtnStates {
        MainMenuClosed,
        MainMenuOpen,
    }

    #[derive(Copy, Clone)]
    pub struct MainMenuButton {
        current_state: MainMenuBtnStates,
    }

    impl Default for MainMenuButton {
        fn default() -> Self {
            Self {
                current_state: MainMenuBtnStates::MainMenuClosed,
            }
        }
    }

    impl MainMenuButton {
        pub fn current_state(&self) -> MainMenuBtnStates {self.current_state}

        pub fn toggle_state(&mut self) {
            match self.current_state {
                MainMenuBtnStates::MainMenuOpen => {
                    self.current_state = MainMenuBtnStates::MainMenuClosed;
                }
                MainMenuBtnStates::MainMenuClosed => {
                    self.current_state = MainMenuBtnStates::MainMenuOpen;
                }
            }
        }
    }
}
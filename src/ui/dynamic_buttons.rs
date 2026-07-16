// holds information about each button that have dynamic information; IE, can have
// different states.
#[derive(Copy, Clone)]
pub struct DynamicButtons {
    pub loop_button: dynamic_loop_button::DynamicLoopBtn,
    pub shuffle_button: dynamic_shuffle_button::DynamicShuffleBtn,
    pub main_menu_button: dynamic_main_menu_button::DynamicMainMenuBtn,
}

impl Default for DynamicButtons {
    fn default() -> Self {
        Self {
            loop_button: dynamic_loop_button::DynamicLoopBtn::default(),
            shuffle_button: dynamic_shuffle_button::DynamicShuffleBtn::default(),
            main_menu_button: dynamic_main_menu_button::DynamicMainMenuBtn::default(),
        }
    }
}

// Help differentiate the state of the widget; IE, active, inactive, etc.
#[derive(Copy, Clone)]
pub enum StyleState {
    ActiveStyle,
    InactiveStyle,
}

// Module to hold struct, enum and methods for the loop button
pub mod dynamic_loop_button {
    use crate::ui::dynamic_buttons::StyleState;

    #[derive(Copy, Clone)]
    pub enum DynamicLoopBtnState {
        LoopAll,
        LoopSingle,
        LoopOff,
    }

    #[derive(Copy, Clone)]
    pub struct DynamicLoopBtn {
        current_style: StyleState,
        current_state: DynamicLoopBtnState,
    }

    impl Default for DynamicLoopBtn {
        fn default() -> Self {
            Self {
                current_style: StyleState::InactiveStyle,
                current_state: DynamicLoopBtnState::LoopOff,
            }
        }
    }

    impl DynamicLoopBtn {
        pub fn current_state(&self) -> DynamicLoopBtnState {
            self.current_state
        }

        pub fn current_style(&self) -> StyleState {self.current_style}

        pub fn toggle_state_and_style(&mut self) {
            match self.current_state {
                DynamicLoopBtnState::LoopAll => {
                    self.current_state = DynamicLoopBtnState::LoopSingle;
                }
                DynamicLoopBtnState::LoopSingle => {
                    self.current_state = DynamicLoopBtnState::LoopOff;
                }
                DynamicLoopBtnState::LoopOff => {
                    self.current_state = DynamicLoopBtnState::LoopAll;
                }
            }
            match self.current_state {
                DynamicLoopBtnState::LoopAll | DynamicLoopBtnState::LoopSingle => self.current_style = StyleState::ActiveStyle,
                DynamicLoopBtnState::LoopOff => self.current_style = StyleState::InactiveStyle,
            }
        }

        pub fn is_state_set_to_loop_all(&self) -> bool {
            match self.current_state {
                DynamicLoopBtnState::LoopAll => true,
                _ => false,
            }
        }

        pub fn is_state_set_to_loop_single(&self) -> bool {
            match self.current_state {
                DynamicLoopBtnState::LoopSingle => true,
                _ => false,
            }
        }

    }
}

// Module to hold struct, enum and methods for the shuffle button
pub mod dynamic_shuffle_button {
    use crate::ui::dynamic_buttons::StyleState;

    #[derive(Copy, Clone)]
    pub enum DynamicShuffleBtnState {
        ShuffleOn,
        ShuffleOff,
    }

    #[derive(Copy, Clone)]
    pub struct DynamicShuffleBtn {
        current_style: StyleState,
        current_state: DynamicShuffleBtnState,
    }

    impl Default for DynamicShuffleBtn {
        fn default() -> Self {
            Self {
                current_style: StyleState::InactiveStyle,
                current_state: DynamicShuffleBtnState::ShuffleOff
            }
        }
    }

    impl DynamicShuffleBtn {
        pub fn current_state(&self) -> DynamicShuffleBtnState {
            self.current_state
        }

        pub fn current_style(&self) -> StyleState {self.current_style}

        pub fn toggle_state_and_style(&mut self) {
            match self.current_state {
                DynamicShuffleBtnState::ShuffleOn => {
                    self.current_state = DynamicShuffleBtnState::ShuffleOff;
                }
                DynamicShuffleBtnState::ShuffleOff => {
                    self.current_state = DynamicShuffleBtnState::ShuffleOn;
                }
            }
            match self.current_state {
                DynamicShuffleBtnState::ShuffleOn => self.current_style = StyleState::ActiveStyle,
                DynamicShuffleBtnState::ShuffleOff => self.current_style = StyleState::InactiveStyle,
            }
        }
    }
}

// Module to hold the struct, enum and methods for the menu button
pub mod dynamic_main_menu_button {
    #[derive(Copy, Clone)]
    pub enum DynamicMainMenuBtnState {
        MainMenuClosed,
        MainMenuOpen,
    }

    #[derive(Copy, Clone)]
    pub struct DynamicMainMenuBtn {
        current_state: DynamicMainMenuBtnState,
    }

    impl Default for DynamicMainMenuBtn {
        fn default() -> Self {
            Self {
                current_state: DynamicMainMenuBtnState::MainMenuClosed,
            }
        }
    }

    impl DynamicMainMenuBtn {
        pub fn current_state(&self) -> DynamicMainMenuBtnState {self.current_state}

        pub fn toggle_state(&mut self) {
            match self.current_state {
                DynamicMainMenuBtnState::MainMenuOpen => {
                    self.current_state = DynamicMainMenuBtnState::MainMenuClosed;
                }
                DynamicMainMenuBtnState::MainMenuClosed => {
                    self.current_state = DynamicMainMenuBtnState::MainMenuOpen;
                }
            }
        }
    }
}
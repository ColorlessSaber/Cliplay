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

// Module to hold struct, enum and methods for the loop button
pub mod dynamic_loop_button {
    use crate::ui::styling::StyleState;

    #[derive(Copy, Clone)]
    pub enum DynamicLoopBtnStates {
        LoopAll,
        LoopSingle,
        LoopOff,
    }

    #[derive(Copy, Clone)]
    pub struct DynamicLoopBtn {
        current_style: StyleState,
        current_state: DynamicLoopBtnStates,
    }

    impl Default for DynamicLoopBtn {
        fn default() -> Self {
            Self {
                current_style: StyleState::InactiveStyle,
                current_state: DynamicLoopBtnStates::LoopOff,
            }
        }
    }

    impl DynamicLoopBtn {
        pub fn current_state(&self) -> DynamicLoopBtnStates {
            self.current_state
        }

        pub fn current_style(&self) -> StyleState {self.current_style}

        pub fn toggle_state_and_style(&mut self) {
            match self.current_state {
                DynamicLoopBtnStates::LoopAll => {
                    self.current_state = DynamicLoopBtnStates::LoopSingle;
                }
                DynamicLoopBtnStates::LoopSingle => {
                    self.current_state = DynamicLoopBtnStates::LoopOff;
                }
                DynamicLoopBtnStates::LoopOff => {
                    self.current_state = DynamicLoopBtnStates::LoopAll;
                }
            }
            match self.current_state {
                DynamicLoopBtnStates::LoopAll | DynamicLoopBtnStates::LoopSingle => self.current_style = StyleState::ActiveStyle,
                DynamicLoopBtnStates::LoopOff => self.current_style = StyleState::InactiveStyle,
            }
        }

        pub fn is_state_set_to_loop_all(&self) -> bool {
            match self.current_state {
                DynamicLoopBtnStates::LoopAll => true,
                _ => false,
            }
        }

        pub fn is_state_set_to_loop_single(&self) -> bool {
            match self.current_state {
                DynamicLoopBtnStates::LoopSingle => true,
                _ => false,
            }
        }

    }
}

// Module to hold struct, enum and methods for the shuffle button
pub mod dynamic_shuffle_button {
    use crate::ui::styling::StyleState;

    #[derive(Copy, Clone)]
    pub enum DynamicShuffleBtnStates {
        ShuffleOn,
        ShuffleOff,
    }

    #[derive(Copy, Clone)]
    pub struct DynamicShuffleBtn {
        current_style: StyleState,
        current_state: DynamicShuffleBtnStates,
    }

    impl Default for DynamicShuffleBtn {
        fn default() -> Self {
            Self {
                current_style: StyleState::InactiveStyle,
                current_state: DynamicShuffleBtnStates::ShuffleOff
            }
        }
    }

    impl DynamicShuffleBtn {
        pub fn current_state(&self) -> DynamicShuffleBtnStates {
            self.current_state
        }

        pub fn current_style(&self) -> StyleState {self.current_style}

        pub fn toggle_state_and_style(&mut self) {
            match self.current_state {
                DynamicShuffleBtnStates::ShuffleOn => {
                    self.current_state = DynamicShuffleBtnStates::ShuffleOff;
                }
                DynamicShuffleBtnStates::ShuffleOff => {
                    self.current_state = DynamicShuffleBtnStates::ShuffleOn;
                }
            }
            match self.current_state {
                DynamicShuffleBtnStates::ShuffleOn => self.current_style = StyleState::ActiveStyle,
                DynamicShuffleBtnStates::ShuffleOff => self.current_style = StyleState::InactiveStyle,
            }
        }
    }
}

// Module to hold the struct, enum and methods for the menu button
pub mod dynamic_main_menu_button {
    #[derive(Copy, Clone)]
    pub enum DynamicMainMenuBtnStates {
        MainMenuClosed,
        MainMenuOpen,
    }

    #[derive(Copy, Clone)]
    pub struct DynamicMainMenuBtn {
        current_state: DynamicMainMenuBtnStates,
    }

    impl Default for DynamicMainMenuBtn {
        fn default() -> Self {
            Self {
                current_state: DynamicMainMenuBtnStates::MainMenuClosed,
            }
        }
    }

    impl DynamicMainMenuBtn {
        pub fn current_state(&self) -> DynamicMainMenuBtnStates {self.current_state}

        pub fn toggle_state(&mut self) {
            match self.current_state {
                DynamicMainMenuBtnStates::MainMenuOpen => {
                    self.current_state = DynamicMainMenuBtnStates::MainMenuClosed;
                }
                DynamicMainMenuBtnStates::MainMenuClosed => {
                    self.current_state = DynamicMainMenuBtnStates::MainMenuOpen;
                }
            }
        }
    }
}
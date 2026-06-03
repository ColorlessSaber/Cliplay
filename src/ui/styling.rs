pub mod icons;
mod button_styles;
pub mod container_styles;

pub use button_styles::*;

// Help differentiate the state of the widget; IE, active, inactive, etc.
#[derive(Copy, Clone)]
pub enum StyleState {
    ActiveStyle,
    InactiveStyle,
}

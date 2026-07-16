pub mod icons;
pub mod button_styles;
pub mod container_styles;
pub mod slider_styles;

// Help differentiate the state of the widget; IE, active, inactive, etc.
#[derive(Copy, Clone)]
pub enum StyleState {
    ActiveStyle,
    InactiveStyle,
}

pub mod negotiation;
pub mod utils;

pub use negotiation::negotiation;

/// Options
#[rustfmt::skip]
mod option {
    pub const ECHO:                u8 =  1;
    pub const SUPPRESS_GO_AHEAD:   u8 =  3;
    pub const STATUS:              u8 =  5;
    pub const TERMINAL_TYPE:       u8 = 24;
    pub const WINDOW_SIZE:         u8 = 31;
    pub const TERMINAL_SPEED:      u8 = 32;
    pub const REMOTE_FLOW_CONTROL: u8 = 33;
    pub const LINE_MODE:           u8 = 34;
    pub const X_DISPLAY_LOCATION:  u8 = 35;
    pub const ENVIRONMENT:         u8 = 36;
    pub const AUTHENTICATION:      u8 = 37;
    pub const ENCRYPT:             u8 = 38;
    pub const NEW_ENVIRONMENT:     u8 = 39;
}

/// Commands
#[rustfmt::skip]
mod command {
    pub const SE:   u8 = 240;
    pub const SB:   u8 = 250;
    pub const WILL: u8 = 251;
    pub const WONT: u8 = 252;
    pub const DO:   u8 = 253;
    pub const DONT: u8 = 254;
    pub const IAC:  u8 = 255;
}

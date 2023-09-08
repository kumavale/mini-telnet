pub mod negotiation;

pub use negotiation::negotiation;

/// Options
#[rustfmt::skip]
mod option {
    pub const SUPPRESS_GO_AHEAD: u8 =  3;
    pub const WINDOW_SIZE:       u8 = 31;
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

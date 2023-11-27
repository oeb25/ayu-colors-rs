#![allow(non_snake_case, non_upper_case_globals)]

mod colors;

pub use colors::*;

pub struct Shade {
    pub fg: Color,
    pub bg: Color,
}

impl Shade {
    pub const fn new(fg: [u8; 4], bg: [u8; 4]) -> Self {
        Self {
            fg: Color(fg),
            bg: Color(bg),
        }
    }
}

pub struct Color([u8; 4]);

impl Color {
    pub fn hex(self) -> String {
        format!("#{:08x}", u32::from_be_bytes(self.0))
    }
}

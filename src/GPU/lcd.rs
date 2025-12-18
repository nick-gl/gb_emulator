
use crate::GPU::lcdc::LCDC;

pub struct LCD {
    pub control: LCDC,    // master control register ($FF40)
    pub status: u8,     // LCD status ($FF41)
    pub scroll_y: u8,   // SCY ($FF42)
    pub scroll_x: u8,   // SCX ($FF43)
    pub ly: u8,         // Current scanline ($FF44)
    pub lyc: u8,        // LY Compare ($FF45)
    pub window_y: u8,   // WY ($FF48)
    pub window_x: u8,   // WX ($FF49)
    pub bg_palette: u8,    // BGP ($FF47)
    pub obj_palette_0: u8, // OBP0 ($FF48)
    pub obj_palette_1: u8, // OBP1 ($FF49)
}

impl LCD {
    pub fn new() -> Self {
        LCD {
            control: LCDC::new(),
            status: 0,
            scroll_y: 0,
            scroll_x: 0,
            ly: 0,
            lyc: 0,
            window_y: 0,
            window_x: 0,
            bg_palette: 0,
            obj_palette_0: 0,
            obj_palette_1: 0,
        }
    }

    /// Update the control register and all its flags at once
    pub fn set_lcd_control(&mut self, value: u8) {
        self.control.raw = value;
    }
}
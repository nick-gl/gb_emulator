#[derive(Debug, Clone, Copy)]
pub struct LCDC {
    pub raw: u8,
}

impl LCDC {
    pub fn new() -> Self {
        LCDC { raw: 0 }
    }

    pub fn lcd_ppu_enable(&self) -> bool {
        self.raw & 0b1000_0000 != 0
    }

    pub fn window_tile_map_area(&self) -> u16 {
        if self.raw & 0b0100_0000 != 0 {
            0x9C00
        } else {
            0x9800
        }
    }

    pub fn window_enable(&self) -> bool {
        self.raw & 0b0010_0000 != 0
    }

    pub fn bg_window_tile_data_area(&self) -> u16 {
        if self.raw & 0b0001_0000 != 0 {
            0x8000
        } else {
            0x8800
        }
    }

    pub fn bg_tile_map_area(&self) -> u16 {
        if self.raw & 0b0000_1000 != 0 {
            0x9C00
        } else {
            0x9800
        }
    }

    pub fn obj_size(&self) -> (u8, u8) {
        if self.raw & 0b0000_0100 != 0 {
            (8, 16)
        } else {
            (8, 8)
        }
    }

    pub fn obj_enable(&self) -> bool {
        self.raw & 0b0000_0010 != 0
    }

    pub fn bg_window_enable(&self) -> bool {
        self.raw & 0b0000_0001 != 0
    }
}

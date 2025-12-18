
pub struct Tile {
    pub color: u128
}

impl Tile {
    pub fn new() -> Tile {
        Tile { color: 0 }
    }
    pub fn new_with_color(color: u128) -> Self {
        Tile { color }
    }

    pub fn set_color(&mut self, color: u128) {
        self.color = color;
    }

    /// Grabs the 2-bit color ID for a specific pixel within the 8x8 tile.
    pub fn get_color_id(&self, row: usize, column: usize) -> u8 {
        // bit_index = (row * 8 + column) * 2
        let bit_index = (row * 8 + column) * 2;
        ((self.color >> bit_index) & 0b11) as u8
    }
}
//test
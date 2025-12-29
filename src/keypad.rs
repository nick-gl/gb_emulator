enum Keypad {
    A, B, SELECT, START,    // Row 2 (Buttons)
    RIGHT, LEFT, UP, DOWN,  // Row 1 (Directions)
}

pub struct Joypad {
    pub row0: u8,      // Directions (Right, Left, Up, Down)
    pub row1: u8,      // Buttons (A, B, Select, Start)
    pub data: u8,      // The $FF00 register
    pub interrupt: u8, // Use 0x10 to align with the IF register bit
}

impl Joypad {
    pub fn new() -> Self {
        Self {
            row0: 0x0F,
            row1: 0x0F,
            data: 0xCF, // Top bits 6-7 are usually 1, bits 4-5 are 1 (unselected)
            interrupt: 0,
        }
    }

    // Your provided update logic
    pub fn update(&mut self) {
        let old_values = self.data & 0xF;
        let mut new_values = 0xF;

        if self.data & 0x10 == 0x00 { // Row 0 selected
            new_values &= self.row0;
        }
        if self.data & 0x20 == 0x00 { // Row 1 selected
            new_values &= self.row1;
        }

        // Detect falling edge (1 to 0 transition)
        if old_values == 0xF && new_values != 0xF {
            self.interrupt |= 0x10;
        }

        self.data = (self.data & 0xF0) | new_values;
    }

    pub fn write_byte(&mut self, val: u8) {
        // Only bits 4 and 5 are writable
        self.data = (val & 0x30) | (self.data & 0xCF);
        self.update();
    }

    pub fn read_byte(&self) -> u8 {
        self.data | 0xC0 // Ensure bits 6-7 are always 1
    }


    pub fn set_key_down(&mut self, key: Keypad) {
        match key {
            Keypad::RIGHT => self.row0 &= !(1 << 0),
            Keypad::LEFT => self.row0 &= !(1 << 1),
            Keypad::UP => self.row0 &= !(1 << 2),
            Keypad::DOWN => self.row0 &= !(1 << 3),
            Keypad::A => self.row1 &= !(1 << 0),
            Keypad::B => self.row1 &= !(1 << 1),
            Keypad::SELECT => self.row1 &= !(1 << 2),
            Keypad::START => self.row1 &= !(1 << 3),
        }
        self.update();
    }

    // When a key is RELEASED
    pub fn set_key_up(&mut self, key: Keypad) {
        match key {
            Keypad::RIGHT => self.row0 |= (1 << 0),
            Keypad::LEFT => self.row0 |= (1 << 1),
            Keypad::UP => self.row0 |= (1 << 2),
            Keypad::DOWN => self.row0 |= (1 << 3),
            Keypad::A => self.row1 |= (1 << 0),
            Keypad::B => self.row1 |= (1 << 1),
            Keypad::SELECT => self.row1 |= (1 << 2),
            Keypad::START => self.row1 |= (1 << 3),
        }
        self.update();
    }
}
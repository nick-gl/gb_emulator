pub enum Frequency {
    F4096 = 0,   // 1024 cycles
    F262144 = 1, // 16 cycles
    F65536 = 2,  // 64 cycles
    F16384 = 3,  // 256 cycles
}

impl From<u8> for Frequency {
    fn from(val: u8) -> Self {
        match val & 0b11 {
            0 => Frequency::F4096,
            1 => Frequency::F262144,
            2 => Frequency::F65536,
            _ => Frequency::F16384,
        }
    }
}

pub struct Timer {
    pub divider: u16,    // Internal 16-bit counter (top 8 bits are DIV 0xFF04)
    pub tima: u8,       // Timer Counter 0xFF05
    pub tma: u8,        // Timer Modulo 0xFF06
    pub tac: u8,        // Timer Control 0xFF07
    internal_cycle: usize,
}

impl Timer {
    pub fn new() -> Timer {
        Timer {
            divider: 0,
            tima: 0,
            tma: 0,
            tac: 0,
            internal_cycle: 0,
        }
    }

    pub fn update(&mut self, cycles: u8) -> bool {
        let cycles = cycles as usize;

        // 1. The Divider (DIV) always increments (16384Hz)
        // It's effectively the 16-bit divider register's upper 8 bits.
        self.divider = self.divider.wrapping_add(cycles as u16);

        // 2. Check if Timer is enabled (Bit 2 of TAC)
        if self.tac & 0b100 == 0 {
            return false;
        }

        self.internal_cycle += cycles;

        // 3. Get threshold based on Frequency bits (Bit 0-1 of TAC)
        let threshold = match self.tac & 0b11 {
            0 => 1024, // 4096 Hz
            1 => 16,   // 262144 Hz
            2 => 64,   // 65536 Hz
            _ => 256,  // 16384 Hz
        };

        let mut interrupt = false;

        // 4. Increment TIMA based on internal cycle count
        while self.internal_cycle >= threshold {
            self.internal_cycle -= threshold;

            if self.tima == 0xFF {
                // Overflow occurred
                self.tima = self.tma; // Reload from Modulo
                interrupt = true;     // Trigger Timer Interrupt
            } else {
                self.tima += 1;
            }
        }

        interrupt
    }
}
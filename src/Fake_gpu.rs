use crate::GPU::gpu::Interrupt;


pub struct FakeGPU {
    pub cycles: u16,
    pub line: u8,
}

impl FakeGPU {
    pub fn new() -> Self {
        Self {
            cycles: 0,
            line: 0,
        }
    }

    /// Step the GPU by a number of CPU cycles
    /// Returns which interrupt(s) should be triggered
    pub fn step(&mut self, cycles: u8) -> Interrupt {
        self.cycles += cycles as u16;

        // Each scanline takes 456 cycles
        while self.cycles >= 456 {
            self.cycles -= 456;
            self.line = self.line.wrapping_add(1);

            if self.line == 144 {
                // VBLANK starts at line 144
                return Interrupt::VBlank;
            } else if self.line > 153 {
                // End of VBLANK, reset to line 0
                self.line = 0;
            }
        }

        Interrupt::None
    }

    pub fn current_line(&self) -> u8 {
        self.line
    }
}

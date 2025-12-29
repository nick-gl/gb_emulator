
pub(crate) enum Frequency {
    F4096,
    F262144,
    F65536,
    F16384,
}//test
impl Frequency {
    fn find_cycle(&self) -> usize {
        match self {
            Frequency::F4096 => 1024,
            Frequency::F16384 => 256,
            Frequency::F262144 => 16,
            Frequency::F65536 => 64,
        }
    }
}
pub struct Timer {
    pub(crate) frequency: Frequency,
    cycle: usize,
    tima: u8,
    tma: u8,
    pub(crate) on: bool,
}
impl Timer {
    pub fn get_tima(&self) -> u8 { self.tima }
    pub fn set_tima(&mut self, val: u8) { self.tima = val; }
    pub fn get_tma(&self) -> u8 { self.tma }
    pub fn set_tma(&mut self, val: u8) { self.tma = val; }

    pub fn new(frequency: Frequency) -> Timer {
        Timer {
            frequency: frequency,
            cycle: 0,
            tima: 0,
            tma: 0,
            on: false,
        }
    }
    pub fn step(&mut self, cycles: u8) -> bool {
        if !self.on { return false; }

        self.cycle += cycles as usize;
        let tick = self.frequency.find_cycle();

        if self.cycle >= tick {
            self.cycle -= tick; // Better than modulo for accuracy
            let (new_tima, overflow) = self.tima.overflowing_add(1);
            self.tima = new_tima;

            if overflow {
                self.tima = self.tma; // Reset TIMA to TMA on overflow
                return true; // Trigger interrupt
            }
        }
        false
    }
}

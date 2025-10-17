
enum Frequency {
    F4096,
    F262144,
    F65536,
    F16384,
}
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
    frequency: Frequency,
    cycle: usize,
    tima: u8,
    tma: u8,
    on: bool,
}
impl Timer {

    pub fn new(frequency: Frequency) -> Timer {
        Timer {
            frequency:frequency,
            cycle: 0,
            tima: 0,
            tma: 0,
            on: false,
        }
    }
    pub fn update(&mut self, cycles: u8) -> bool {
        if !self.on {
            return false;
        }
        self.cycle += cycles as usize;
        let tick = self.frequency.find_cycle();
        let overflow =  if tick > self.cycle {
                self.cycle = self.cycle % tick;
                let (new, overflow) = self.tma.overflowing_add(1);
                self.tima = new;
                overflow
        } else {
            false
        };
        if overflow {
            self.tima = self.tma;
        }
        overflow
    }
}

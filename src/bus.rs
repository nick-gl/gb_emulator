use crate::GPU::gpu::GPU; // Adjust path as needed
use crate::cartride::Cartridge;
use crate::timer::Timer;

pub const VRAM_BEGIN: usize = 0x8000;
pub const VRAM_END: usize = 0x9FFF;
pub const VRAM_SIZE: usize = VRAM_END - VRAM_BEGIN + 1;

pub const WRAM_BEGIN: usize = 0xC000;
pub const WRAM_END: usize = 0xCFFF;
pub const WRAM_SIZE: usize = WRAM_END - WRAM_BEGIN + 1;

pub const SWITCH_BEGIN: usize = 0xA000;
pub const SWITCH_END: usize = 0xBFFF;
pub const SWITCH_SIZE: usize = SWITCH_END - SWITCH_BEGIN + 1;

pub const OAM_BEGIN: usize = 0xFE00;
pub const OAM_END: usize = 0xFE9F;
pub const OAM_SIZE: usize = OAM_END - OAM_BEGIN + 1;

pub const IO_BEGIN: usize = 0xFF00;
pub const IO_END: usize = 0xFF7F;
pub const IO_SIZE: usize = IO_END - IO_BEGIN + 1;

pub const HRAM_BEGIN: usize = 0xFF80;
pub const HRAM_END: usize = 0xFFFE;
pub const HRAM_SIZE: usize = HRAM_END - HRAM_BEGIN + 1; //test

pub struct MemoryBus {
    pub gpu: GPU,         // Bus now owns the GPU
    pub timer: Timer,
    pub cartridge: Cartridge,
    pub wram_bank: [u8; WRAM_SIZE],
    pub hram: [u8; HRAM_SIZE],
    pub io: [u8; IO_SIZE],
    pub interrupt_enable: u8,
}

impl MemoryBus {
    pub fn new(cartridge: Cartridge) -> Self {
        Self {
            gpu: GPU::new(),              // Initializes VRAM, OAM, and Tiles
            timer: Timer::new(),          // Your Timer implementation
            cartridge: cartridge,         // The loaded game ROM
            wram_bank: [0; WRAM_SIZE],
            hram: [0; HRAM_SIZE],
            io: [0; IO_SIZE],
            interrupt_enable: 0,
        }
    }
    pub fn read_byte(&self, address: u16) -> u8 {
        let addr = address as usize; // Convert once here

        match addr {
            0xFF04 => (self.timer.divider >> 8) as u8,
            0xFF05 => self.timer.tima,
            0xFF06 => self.timer.tma,
            0xFF07 => self.timer.tac,
            // Cartridge ROM Banks
            0x0000..=0x7FFF => self.cartridge.read_rom(address),

            // GPU VRAM
            VRAM_BEGIN..=VRAM_END => self.gpu.vram[addr - VRAM_BEGIN],

            // External RAM (Cartridge)
            SWITCH_BEGIN..=SWITCH_END => self.cartridge.read_ram(address),

            // Work RAM (WRAM)
            WRAM_BEGIN..=WRAM_END => self.wram_bank[addr - WRAM_BEGIN],

            // Echo RAM (Mirror of WRAM - common in GB games)
            0xE000..=0xFDFF => self.wram_bank[addr - 0xE000],

            // GPU OAM (Object Attribute Memory)
            OAM_BEGIN..=OAM_END => self.gpu.oam[addr - OAM_BEGIN],

            // GPU I/O Registers (Direct mapping)
            0xFF40 => self.gpu.lcd.control.raw,
            0xFF42 => self.gpu.lcd.scroll_y,
            0xFF43 => self.gpu.lcd.scroll_x,
            0xFF44 => self.gpu.lcd.ly,
            0xFF45 => self.gpu.lcd.lyc,
            0xFF47 => self.gpu.lcd.bg_palette,
            0xFF48 => self.gpu.lcd.obj_palette_0,
            0xFF49 => self.gpu.lcd.obj_palette_1,
            0xFF4A => self.gpu.lcd.window_y,
            0xFF4B => self.gpu.lcd.window_x,

            // Other I/O and Timer
            IO_BEGIN..=IO_END => self.io[addr - IO_BEGIN],

            // High RAM (HRAM)
            HRAM_BEGIN..=HRAM_END => self.hram[addr - HRAM_BEGIN],

            // Interrupt Enable Register
            0xFFFF => self.interrupt_enable,


            _ => 0xFF,
        }
    }

    pub fn write_byte(&mut self, address: u16, value: u8) {
        let addr = address as usize;

        match addr {
            0xFF04 => self.timer.divider = 0, // Writing to DIV resets it to 0
            0xFF05 => self.timer.tima = value,
            0xFF06 => self.timer.tma = value,
            0xFF07 => self.timer.tac = value,
            0x0000..=0x7FFF => self.cartridge.write_rom(address, value),

            VRAM_BEGIN..=VRAM_END => self.gpu.write_vram(addr - VRAM_BEGIN, value),

            SWITCH_BEGIN..=SWITCH_END => self.cartridge.write_ram(address, value),

            WRAM_BEGIN..=WRAM_END => self.wram_bank[addr - WRAM_BEGIN] = value,

            0xE000..=0xFDFF => self.wram_bank[addr - 0xE000] = value,

            OAM_BEGIN..=OAM_END => self.gpu.write_oam(addr - OAM_BEGIN, value),

            // DMA Transfer (Very important for sprites!)
            0xFF46 => self.perform_dma(value),

            // GPU I/O Registers
            0xFF40 => self.gpu.lcd.control.raw = value,
            0xFF42 => self.gpu.lcd.scroll_y = value,
            0xFF43 => self.gpu.lcd.scroll_x = value,
            0xFF45 => self.gpu.lcd.lyc = value,
            0xFF47 => self.gpu.lcd.bg_palette = value,
            0xFF48 => self.gpu.lcd.obj_palette_0 = value,
            0xFF49 => self.gpu.lcd.obj_palette_1 = value,
            0xFF4A => self.gpu.lcd.window_y = value,
            0xFF4B => self.gpu.lcd.window_x = value,

            IO_BEGIN..=IO_END => self.io[addr - IO_BEGIN] = value,
            HRAM_BEGIN..=HRAM_END => self.hram[addr - HRAM_BEGIN] = value,
            0xFFFF => self.interrupt_enable = value,
            _ => {}
        }
    }
    fn perform_dma(&mut self, value: u8) {
        let base_address = (value as u16) << 8;
        for i in 0..0xA0 { // OAM is 160 bytes
            let data = self.read_byte(base_address + i);
            self.gpu.write_oam(i as usize, data);
        }
    }
}
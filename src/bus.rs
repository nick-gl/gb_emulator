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
    vram: [u8; VRAM_SIZE],
    timer: Timer,
    hram: [u8; HRAM_SIZE],
    io: [u8; IO_SIZE],
    interruptenable: u8,
    rom_bank: [u8; 0x4000],     // 0x0000–0x3FFF
    rom_bank_n: [u8; 0x4000],   // 0x4000–0x7FFF
    oam: [u8; OAM_SIZE],
    switch_bank: [u8; SWITCH_SIZE],
    wram_bank: [u8; WRAM_SIZE],
    cartridge: Cartridge,
}

impl MemoryBus {
    /* pub fn new() -> Self {
        Self {
            vram: [0; VRAM_SIZE],
            // timer: Timer,
            hram: [0; HRAM_SIZE],
            io: [0; IO_SIZE],
            interruptenable: 0,
            rom_bank: [0; 0x4000],
            rom_bank_n: [0; 0x4000],
            oam: [0; OAM_SIZE],
            switch_bank: [0; SWITCH_SIZE],
            wram_bank: [0; WRAM_SIZE],
        }
    } */
    pub fn update(&mut self, cycle: u8) {
        if self.timer.update(cycle) {
            
        }
    }

    pub fn read_byte(&self, address: u16) -> u8 {
        match address as usize {
            0x0000..=0x3FFF => self.rom_bank[address as usize],
            0x4000..=0x7FFF => self.rom_bank_n[address as usize - 0x4000],
            VRAM_BEGIN..=VRAM_END => self.vram[address as usize - VRAM_BEGIN],
            SWITCH_BEGIN..=SWITCH_END => self.switch_bank[address as usize - SWITCH_BEGIN],
            WRAM_BEGIN..=WRAM_END => self.wram_bank[address as usize - WRAM_BEGIN],
            OAM_BEGIN..=OAM_END => self.oam[address as usize - OAM_BEGIN],
            IO_BEGIN..=IO_END => self.io[address as usize - IO_BEGIN],
            HRAM_BEGIN..=HRAM_END => self.hram[address as usize - HRAM_BEGIN],
            0xFFFF => self.interruptenable,
            _ => panic!("Cannot read from address {:#X}", address),
        }
    }

    pub fn write_byte(&mut self, address: u16, value: u8) {
        match address as usize {
            0x0000..=0x3FFF => self.rom_bank[address as usize] = value, // typically ROM is read-only; here it’s placeholder
            0x4000..=0x7FFF => self.rom_bank_n[address as usize - 0x4000] = value,
            VRAM_BEGIN..=VRAM_END => self.vram[address as usize - VRAM_BEGIN] = value,
            SWITCH_BEGIN..=SWITCH_END => self.switch_bank[address as usize - SWITCH_BEGIN] = value,
            WRAM_BEGIN..=WRAM_END => self.wram_bank[address as usize - WRAM_BEGIN] = value,
            OAM_BEGIN..=OAM_END => self.oam[address as usize - OAM_BEGIN] = value,
            IO_BEGIN..=IO_END => self.io[address as usize - IO_BEGIN] = value,
            HRAM_BEGIN..=HRAM_END => self.hram[address as usize - HRAM_BEGIN] = value,
            0xFFFF => self.interruptenable = value,
            _ => panic!("Cannot write to address {:#X}", address),
        }
    }
}

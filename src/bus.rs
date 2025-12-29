use crate::Fake_gpu::FakeGPU;
use crate::GPU::gpu;
use crate::GPU::gpu::{GPU, Interrupt}; // Simplified for now
use crate::InterruptFlags::InterruptFlags;
// use crate::GPU::{BackgroundAndWindowDataSelect, ObjectSize, tile::TileMap}; // Commented out unused imports
use crate::keypad::Joypad;

use crate::timer::{Frequency, Timer};
pub const OAM_BEGIN: usize = 0xFE00;
pub const OAM_END: usize = 0xFE9F;
pub const OAM_SIZE: usize = OAM_END - OAM_BEGIN + 1;
pub const BOOT_ROM_SIZE: usize = 0x100;
pub const ROM_BANK_0_SIZE: usize = 0x4000;
pub const ROM_BANK_N_SIZE: usize = 0x4000;
pub const VRAM_BEGIN: usize = 0x8000;
pub const VRAM_END: usize = 0x9FFF;
pub const VRAM_SIZE: usize = VRAM_END - VRAM_BEGIN + 1;
pub const EXTERNAL_RAM_SIZE: usize = 0x2000;
pub const WORKING_RAM_SIZE: usize = 0x2000;
pub const ZERO_PAGE_SIZE: usize = 0x7F;
pub const VBLANK_VECTOR: u16 = 0x40;
pub const LCDSTAT_VECTOR: u16 = 0x48;
pub const TIMER_VECTOR: u16 = 0x50;
pub struct MemoryBus {
    boot_rom: Option<[u8; BOOT_ROM_SIZE]>,
    rom_bank_0: [u8; ROM_BANK_0_SIZE],
    rom_bank_n: [u8; ROM_BANK_N_SIZE],
    external_ram: [u8; EXTERNAL_RAM_SIZE],
    working_ram: [u8; WORKING_RAM_SIZE],
    zero_page: [u8; ZERO_PAGE_SIZE],
    pub gpu: GPU,
    pub interrupt_enable: InterruptFlags,
    pub interrupt_flag: InterruptFlags,
    timer: Timer,
    divider_timer: Timer,
    pub joypad: Joypad,
    pub serial_data: u8,
    pub serial_control: u8,
}

impl MemoryBus {
    pub(crate) fn switch_speed(&self) {
        return;//todo
    }
}

impl MemoryBus {
    pub fn new(boot_rom_buffer: Option<Vec<u8>>, game_rom: Vec<u8>) -> MemoryBus {
        let boot_rom = boot_rom_buffer.map(|buffer| {
            let mut br = [0; BOOT_ROM_SIZE];
            br.copy_from_slice(&buffer[..BOOT_ROM_SIZE]);
            br
        });

        let mut rom_bank_0 = [0; ROM_BANK_0_SIZE];
        rom_bank_0.copy_from_slice(&game_rom[0..0x4000]);

        let mut rom_bank_n = [0; ROM_BANK_N_SIZE];
        let end_idx = if game_rom.len() < 0x8000 { game_rom.len() } else { 0x8000 };
        rom_bank_n.copy_from_slice(&game_rom[0x4000..end_idx]);

        let mut divider_timer = Timer::new(Frequency::F16384);
        divider_timer.on = true;

        MemoryBus {
            boot_rom,
            rom_bank_0,
            rom_bank_n,
            external_ram: [0; EXTERNAL_RAM_SIZE],
            working_ram: [0; WORKING_RAM_SIZE],
            zero_page: [0; ZERO_PAGE_SIZE],
            gpu: GPU::new(),
            interrupt_enable: InterruptFlags::new(),
            interrupt_flag: InterruptFlags::new(),
            timer: Timer::new(Frequency::F4096),
            divider_timer,
            joypad: Joypad::new(),
            serial_data: 0x00,
            serial_control: 0x00,
        }
    }
    pub fn new_test(test: Vec<u8>) -> MemoryBus {
        // Pad test ROM to minimum GB size (32 KB)
        let mut game_rom = vec![0u8; 0x8000];
        let copy_len = test.len().min(0x8000);
        game_rom[..copy_len].copy_from_slice(&test[..copy_len]);

        let mut rom_bank_0 = [0; ROM_BANK_0_SIZE];
        rom_bank_0.copy_from_slice(&game_rom[0..0x4000]);

        let mut rom_bank_n = [0; ROM_BANK_N_SIZE];
        rom_bank_n.copy_from_slice(&game_rom[0x4000..0x8000]);

        let mut divider_timer = Timer::new(Frequency::F16384);
        divider_timer.on = true;

        MemoryBus {
            boot_rom: None,
            rom_bank_0,
            rom_bank_n,
            external_ram: [0; EXTERNAL_RAM_SIZE],
            working_ram: [0; WORKING_RAM_SIZE],
            zero_page: [0; ZERO_PAGE_SIZE],
            interrupt_enable: InterruptFlags::new(),
            interrupt_flag: InterruptFlags::new(),
            timer: Timer::new(Frequency::F4096),
            divider_timer,
            joypad: Joypad::new(),
            serial_data: 0x00,
            serial_control: 0x00,
            gpu: GPU::new(),
        }
    }
    pub fn step(&mut self, cycles: u8) {
        if self.timer.step(cycles) {
            self.interrupt_flag.timer = true;
        }
        self.divider_timer.step(cycles);
        let (vblank, lcd) = match self.gpu.step(cycles) {
            Interrupt::Both => (true, true),
            Interrupt::VBlank => (true, false),
            Interrupt::LCDStat => (false, true),
            Interrupt::None => (false, false),
        };

        if vblank {
            self.interrupt_flag.vblank = true;
        }
        if lcd {
            self.interrupt_flag.lcdstat = true;
        }
    }

    pub fn read_byte(&self, address: u16) -> u8 {
        match address {
            0x0000..=0x00FF => {
                if let Some(boot) = self.boot_rom { boot[address as usize] }
                else { self.rom_bank_0[address as usize] }
            }
            0x0000..=0x3FFF => self.rom_bank_0[address as usize],
            0x4000..=0x7FFF => self.rom_bank_n[(address - 0x4000) as usize],

            // --- GPU VRAM READ TODO ---
            // Replace with actual VRAM access once GPU is finished
            0x8000..=0x9FFF => 0xFF,

            0xA000..=0xBFFF => self.external_ram[(address - 0xA000) as usize],
            0xC000..=0xDFFF => self.working_ram[(address - 0xC000) as usize],
            0xE000..=0xFDFF => self.working_ram[(address - 0xE000) as usize],

            // --- GPU OAM READ TODO ---
            0xFE00..=0xFE9F => 0x00,

            0xFF00..=0xFF7F => self.read_io_register(address),
            0xFF80..=0xFFFE => self.zero_page[(address - 0xFF80) as usize],
            0xFFFF => self.interrupt_enable.to_byte(),
            _ => 0xFF,
        }
    }

    pub fn write_byte(&mut self, address: u16, value: u8) {
        match address {
            // --- GPU VRAM WRITE TODO ---
            0x8000..=0x9FFF => {},

            0xA000..=0xBFFF => self.external_ram[(address - 0xA000) as usize] = value,
            0xC000..=0xDFFF => self.working_ram[(address - 0xC000) as usize] = value,

            // --- GPU OAM WRITE TODO ---
            0xFE00..=0xFE9F => {},

            0xFF00..=0xFF7F => self.write_io_register(address, value),
            0xFF80..=0xFFFE => self.zero_page[(address - 0xFF80) as usize] = value,
            0xFFFF => self.interrupt_enable.from_byte(value),
            _ => {}
        }
    }



    pub fn has_interrupt(&self) -> bool {
        (self.interrupt_enable.vblank && self.interrupt_flag.vblank)
            || (self.interrupt_enable.lcdstat && self.interrupt_flag.lcdstat)
            || (self.interrupt_enable.timer && self.interrupt_flag.timer)
            || (self.interrupt_enable.serial && self.interrupt_flag.serial)
            || (self.interrupt_enable.joypad && self.interrupt_flag.joypad)
    }
    pub fn has_pending_interrupt(&self) -> bool {
        self.interrupt_flag.vblank
            || self.interrupt_flag.lcdstat
            || self.interrupt_flag.timer
            || self.interrupt_flag.serial
            || self.interrupt_flag.joypad
    }
    fn read_io_register(&self, address: u16) -> u8 {
        match address {
            0xFF00 => 0xFF,   // Joypad (no buttons pressed)
            0xFF04 => 0x00,   // DIV
            0xFF05 => 0x00,   // TIMA
            0xFF06 => 0x00,   // TMA
            0xFF07 => 0x00,   // TAC
            0xFF0F => self.interrupt_flag.to_byte(),   // IF

            // GPU stub values
            0xFF40 => 0x91,
            0xFF41 => 0x00,
            0xFF42 => 0x00,
            0xFF43 => 0x00,
            0xFF44 => 0x90,
            0xFF45 => 0x00,
            0xFF46 => 0x00,
            0xFF47 => 0xFC,
            0xFF48 => 0xFF,
            0xFF49 => 0xFF,
            0xFF4A => 0x00,
            0xFF4B => 0x00,

            _ => 0xFF,
        }
    }


    fn _read_io_register(&self, address: u16) -> u8 {
        match address {
            0xFF00 => self.joypad.read_byte(),
            0xFF04 => self.divider_timer.get_tima(),
            0xFF05 => self.timer.get_tima(),
            0xFF06 => self.timer.get_tma(),
            0xFF07 => self.read_tac(),
            0xFF0F => self.interrupt_flag.to_byte(),

            // --- GPU READ REGISTERS TODO ---
            /*
            0xFF40 => self.gpu.read_lcdc(),
            0xFF42 => self.gpu.viewport_y_offset,
            0xFF43 => self.gpu.viewport_x_offset,
            0xFF44 => self.gpu.line,
            */
            0xFF40..=0xFF4B => 0x00,

            _ => 0xFF,
        }
    }
    pub fn write_word(&mut self, address: u16, value: u16) {
        self.write_byte(address, (value & 0xFF) as u8);
        self.write_byte(address + 1, (value >> 8) as u8);
    }
    fn write_io_register(&mut self, address: u16, value: u8) {
        match address {
            0xFF00 => {
                self.joypad.write_byte(value);
                if self.joypad.interrupt != 0 {
                    self.interrupt_flag.joypad = true;
                    self.joypad.interrupt = 0;
                }
            }
            // --- SERIAL PORT OUTPUT ---
            0xFF01 => {
                self.serial_data = value;
            }

            0xFF02 => {
                self.serial_control = value;

                // Blargg: transfer requested
                if value == 0x81 {
                    let ch = self.serial_data as char;
                    print!("{}", ch);
                    use std::io::{self, Write};
                    io::stdout().flush().unwrap();

                    // Clear transfer flag (real hardware does this later)
                    self.serial_control = 0x00;
                }
            }


            0xFF04 => self.divider_timer.set_tima(0),
            0xFF05 => self.timer.set_tima(value),
            0xFF06 => self.timer.set_tma(value),
            0xFF07 => self.write_tac(value),
            0xFF0F => self.interrupt_flag.from_byte(value),
            0xFF46 => self.dma_transfer(value),

            // --- GPU WRITE REGISTERS TODO ---
            0xFF40..=0xFF4B => {},

            0xFF50 => self.boot_rom = None,
            _ => {}
        }
    }

    fn read_tac(&self) -> u8 {
        let freq_bits = match self.timer.frequency {
            Frequency::F4096 => 0b00,
            Frequency::F262144 => 0b01,
            Frequency::F65536 => 0b10,
            Frequency::F16384 => 0b11,
        };
        0xF8 | (if self.timer.on { 0x04 } else { 0x00 }) | freq_bits
    }

    fn write_tac(&mut self, value: u8) {
        self.timer.on = (value & 0x04) != 0;
        self.timer.frequency = match value & 0x03 {
            0b00 => Frequency::F4096,
            0b01 => Frequency::F262144,
            0b10 => Frequency::F65536,
            _ => Frequency::F16384,
        };
    }

    fn dma_transfer(&mut self, value: u8) {
        let base = (value as u16) << 8;
        for i in 0..0xA0 {
            let b = self.read_byte(base + i);
            self.write_byte(0xFE00 + i, b);
        }
    }
}
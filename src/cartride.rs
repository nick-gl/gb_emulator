use std::fs::File;
use std::io::Read;

pub struct Cartridge {
    pub rom: Vec<u8>,
    pub ram: Vec<u8>,
    pub has_battery: bool,
    pub mbc_type: MbcType,
    pub rom_bank: u16, // Changed to u16 because MBC5 can have up to 512 banks
    pub ram_bank: u8,
    pub ram_enabled: bool,
    pub title: String,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MbcType {
    RomOnly,
    Mbc1,
    Mbc2,
    Mbc3,
    Mbc5,
    Unknown,
}

impl Cartridge {
    pub fn load(filename: &str) -> Result<Self, String> {
        let mut file = File::open(filename).map_err(|e| e.to_string())?;
        let mut rom = Vec::new();
        file.read_to_end(&mut rom).map_err(|e| e.to_string())?;

        // 1. Parse Title (0x0134 - 0x0143)
        let title = String::from_utf8_lossy(&rom[0x0134..0x0143])
            .trim_matches(char::from(0))
            .to_string();

        // 2. Determine MBC Type (0x0147)
        let mbc_byte = rom[0x0147];
        let mbc_type = match mbc_byte {
            0x00 => MbcType::RomOnly,
            0x01..=0x03 => MbcType::Mbc1,
            0x05..=0x06 => MbcType::Mbc2,
            0x0F..=0x13 => MbcType::Mbc3,
            0x19..=0x1E => MbcType::Mbc5,
            _ => MbcType::Unknown,
        };

        // 3. Battery check (Simplified)
        let has_battery = match mbc_byte {
            0x03 | 0x06 | 0x09 | 0x0D | 0x0F | 0x10 | 0x13 | 0x1B | 0x1E => true,
            _ => false,
        };

        println!("Loaded ROM: {}, MBC: {:?}", title, mbc_type);

        Ok(Cartridge {
            rom,
            ram: vec![0; 0x8000], // Default 32KB RAM, can be resized based on 0x0148
            has_battery,
            mbc_type,
            rom_bank: 1, // Bank 0 is always at 0x0000-0x3FFF, Bank 1 starts at 0x4000
            ram_bank: 0,
            ram_enabled: false,
            title,
        })
    }

    // This is what the MemoryBus calls
    pub fn read_rom(&self, address: u16) -> u8 {
        match address {
            // Fixed Bank 00
            0x0000..=0x3FFF => self.rom[address as usize],
            // Switchable Bank
            0x4000..=0x7FFF => {
                let offset = (self.rom_bank as usize) * 0x4000;
                let actual_addr = offset + (address as usize - 0x4000);
                self.rom[actual_addr % self.rom.len()]
            }
            _ => 0xFF,
        }
    }

    pub fn write_rom(&mut self, address: u16, value: u8) {
        // Writing to ROM area controls the MBC (banking)
        match self.mbc_type {
            MbcType::Mbc1 => self.handle_mbc1_write(address, value),
            MbcType::RomOnly => (), // Do nothing, ROM is read-only
            _ => (), // Implement others as needed
        }
    }

    fn handle_mbc1_write(&mut self, address: u16, value: u8) {
        match address {
            0x0000..=0x1FFF => self.ram_enabled = (value & 0x0F) == 0x0A,
            0x2000..=0x3FFF => {
                // Bank can't be 0, if 0 is written, it maps to 1
                let mut bank = (value & 0x1F) as u16;
                if bank == 0 { bank = 1; }
                self.rom_bank = bank;
            }
            _ => (),
        }
    }

    pub fn read_ram(&self, address: u16) -> u8 {
        if !self.ram_enabled { return 0xFF; }
        let offset = (self.ram_bank as usize) * 0x2000;
        self.ram[offset + (address as usize - 0xA000)]
    }

    pub fn write_ram(&mut self, address: u16, value: u8) {
        if !self.ram_enabled { return; }
        let offset = (self.ram_bank as usize) * 0x2000;
        self.ram[offset + (address as usize - 0xA000)] = value;
    }
}
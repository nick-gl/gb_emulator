pub struct Cartridge {
    pub rom: Vec<u8>,
    pub ram: Vec<u8>,
    pub has_battery: bool,
    pub mbc_type: MbcType,
    pub rom_bank: u8,
    pub ram_bank: u8,
    pub ram_enabled: bool,
    pub title: String,
}//test

#[derive(Debug, Clone, Copy)]
pub enum MbcType {
    RomOnly,
    Mbc1,
    Mbc2,
    Mbc3,
    Mbc5,
    Unknown,
}
impl Cartridge {
    pub fn load(&self, filename: &str) {

    }
}

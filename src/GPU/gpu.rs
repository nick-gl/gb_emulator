use sdl2::{
    event::Event,
    keyboard::Keycode,
    pixels::{Color, PixelFormatEnum},
    render::{Canvas, TextureCreator},
    video::{Window, WindowContext},
    Sdl,
};
use sdl2::keyboard::Mod;
use crate::bus::MemoryBus;
use crate::bus::{OAM_SIZE, VRAM_BEGIN, VRAM_SIZE};
use crate::GPU::lcd::LCD;
use crate::GPU::tile::Tile;

pub const NUM_OBJ: usize = 40;
pub const OAM_BEGIN: usize = 0xFE00;
pub const OAM_END: usize = 0xFE9F;

pub const VRAM_END: usize = 0x9FFF;

pub enum Palette {
    BGP,
    OBJ,
}
#[derive(Eq, PartialEq)]
pub enum Interrupt{
    None,
    VBlank,
    LCDStat,
    Both,
}
pub enum Modes {
    HBlank,
    VBlank,
    OAM,
    Pixel
}
impl From<u8> for Modes {
    fn from(val: u8) -> Self {
        match val {
            0 => Modes::HBlank,
            1 => Modes::VBlank,
            2 => Modes::OAM,
            3 => Modes::Pixel,
            _ => Modes::VBlank
        }
    }
}
impl From<Modes> for u8 {
    fn from(modes: Modes) -> Self {
        match modes {
            Modes::HBlank => 0,
            Modes::VBlank => 1,
            Modes::OAM => 2,
            Modes::Pixel => 3
        }
    }
}
impl Interrupt {
    fn add(&mut self, other: Interrupt) {
        match self {
            Interrupt::None => *self = other,
            Interrupt::VBlank if other == Interrupt::LCDStat => {
                *self = Interrupt::Both
            }
            Interrupt::LCDStat if other == Interrupt::VBlank => {
                *self = Interrupt::Both
            }
            _ => {}
        };
    }
}
impl Default for Object {
    fn default() -> Self {
        Object{
            x: -16,
            y: -8,
            tile_index: Default::default(),
            palette: Default::default(),
            x_flip: Default::default(),
            y_flip: Default::default(),
            priority: Default::default(),
        }
    }
}
impl Default for Palette {
    fn default() -> Self {
        Palette::BGP
    }
}
pub enum Background_color {

}

pub struct Object {
    pub x: i16,
    pub y: i16,
    pub tile_index: usize,
    pub palette: Palette,
    pub x_flip: bool,
    pub y_flip: bool,
    pub priority: bool,
}
impl Palette {
    pub fn map_color(&self, color: u8) -> Option<Color> {
        let gb_colors = [
            Color::RGB(155, 188, 15),
            Color::RGB(139, 172, 15),
            Color::RGB(48, 98, 48),
            Color::RGB(15, 56, 15),
        ];
        let obj_colors = [
            Color::RGB(155, 188, 15),
            Color::RGB(139, 172, 15),
            Color::RGB(48, 98, 48),
            Color::RGB(15, 56, 15),
        ];

        let palette = match self {
            Palette::BGP => &gb_colors,
            Palette::OBJ => &obj_colors,
        };
        palette.get(color as usize).copied()
    }
}
pub struct Position {
   pub x: i32,
    pub y: i32,
}
pub struct GPU {
    renderer: Canvas<Window>,
    pub tiles: Vec<Tile>,
    pub vram: [u8; VRAM_SIZE],
    pub oam: [u8; OAM_SIZE],
    pub palette: Palette,
    pub object_data: [Object;NUM_OBJ],
    pub lcd: LCD,
    pub modes: Modes,
}//test
impl GPU {

    pub fn write_oam(&mut self, index: usize, value: u8) {
        self.oam[index] = value;
        let object_index = index / 4;
        if object_index > NUM_OBJ {
            return;
        }

        let byte = index % 4;

        let data = self.object_data.get_mut(object_index).unwrap();
        match byte {
            0 => data.y = (value as i16) - 0x10,
            1 => data.x = (value as i16) - 0x8,
            2 => data.tile_index = value as usize,
            _ => {
                data.palette = if (value & 0x10) != 0 {
                    Palette::OBJ
                } else {
                    Palette::BGP
                };
                data.x_flip = (value & 0x20) != 0;
                data.y_flip = (value & 0x40) != 0;
                data.priority = (value & 0x80) == 0;
            }
        }
    }
    pub fn write_vram(&mut self, addr: usize, value: u8) {
        self.vram[addr] = value;

        let tile_index = addr / 16;
        if tile_index >= self.tiles.len() {
            return;
        }

        let base = tile_index * 16;
        let mut packed: u128 = 0;

        for row in 0..8 {
            let lo = self.vram[base + row * 2] as u16;
            let hi = self.vram[base + row * 2 + 1] as u16;

            for x in 0..8 {
                let bit = 7 - x;
                let l = (lo >> bit) & 1;
                let h = (hi >> bit) & 1;
                let color_num = ((h << 1) | l) as u128;

                let bit_index = (row * 8 + x) * 2;
                packed |= color_num << bit_index;
            }
        }

        self.tiles[tile_index].set_color(packed);
    }
    pub fn update(&mut self, cycles: u8) -> Interrupt {
        if !self.lcd.enabled {
            return Interrupt::None;
        }
       Interrupt::VBlank
    }
}

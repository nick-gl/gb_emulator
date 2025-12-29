use sdl2::{
    event::Event,
    keyboard::Keycode,
    pixels::{Color, PixelFormatEnum},
    render::{Canvas, TextureCreator},
    video::{Window, WindowContext},
    Sdl,
};
//use crate::bus::{OAM_SIZE, VRAM_BEGIN, VRAM_SIZE};
use crate::Tiles::Tiles;

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
    pub vram: [u8; 1000],
    pub oam: [u8; 10000],
    pub palette: Palette,
    pub object_data: [Object;NUM_OBJ],
    pub modes: Modes,
}//test
impl GPU {
    
}
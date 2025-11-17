use sdl2::{
    event::Event,
    keyboard::Keycode,
    pixels::{Color, PixelFormatEnum},
    render::{Canvas, TextureCreator},
    video::{Window, WindowContext},
    Sdl,
};
use crate::bus::MemoryBus;
use crate::GPU::lcd::LCD;
use crate::GPU::tile::Tile;

pub const OAM_BEGIN: usize = 0xFE00;
pub const OAM_END: usize = 0xFE9F;
pub const OAM_SIZE: usize = crate::bus::OAM_END - crate::bus::OAM_BEGIN + 1;

pub const VRAM_BEGIN: usize = 0x8000;
pub const VRAM_END: usize = 0x9FFF;
pub const VRAM_SIZE: usize = crate::bus::VRAM_END - crate::bus::VRAM_BEGIN + 1;
pub enum Palette {
    BGP,
    OBJ,
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
    pub
    pub lcd: LCD,
}//test
impl GPU {

    pub fn write_to_oam(&mut self, data: u8) {
        object
}
}

use crate::GPU::lcd::LCD;
use crate::GPU::lcdc::LCDC;
use crate::GPU::tile::Tile;
use sdl2::pixels::Color;
use crate::bus::VRAM_SIZE;
use crate::bus::OAM_SIZE;
pub const NUM_OBJ: usize = 40;
pub const SCREEN_WIDTH: usize = 160;
pub const SCREEN_HEIGHT: usize = 144;

#[derive(Eq, PartialEq, Clone, Copy)]
pub enum Palette {
    BGP,
    OBJ,
}

#[derive(Eq, PartialEq)]
pub enum Interrupt {
    None,
    VBlank,
    LCDStat,
    Both,
}

pub enum Modes {
    HBlank,
    VBlank,
    OAM,
    Pixel,
}

impl Interrupt {
    fn add(&mut self, other: Interrupt) {
        match self {
            Interrupt::None => *self = other,
            Interrupt::VBlank if other == Interrupt::LCDStat => *self = Interrupt::Both,
            Interrupt::LCDStat if other == Interrupt::VBlank => *self = Interrupt::Both,
            _ => {}
        };
    }
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

impl Default for Object {
    fn default() -> Self {
        Object {
            x: -16,
            y: -8,
            tile_index: 0,
            palette: Palette::BGP,
            x_flip: false,
            y_flip: false,
            priority: false,
        }
    }
}

impl Palette {
    pub fn map_color(&self, color: u8) -> Option<Color> {
        let colors = [
            Color::RGB(155, 188, 15),
            Color::RGB(139, 172, 15),
            Color::RGB(48, 98, 48),
            Color::RGB(15, 56, 15),
        ];
        colors.get(color as usize).copied()
    }
}

pub struct GPU {
    pub vram: [u8; 8192],
    pub oam: [u8; 160],
    pub canvas_buffer: [u8; SCREEN_WIDTH * SCREEN_HEIGHT * 4],
    pub tiles: Vec<Tile>,
    pub object_data: [Object; NUM_OBJ],
    pub lcd: LCD,
    pub modes: Modes,
    pub cycles: u32,
    pub lyc_flag: bool,
    pub lyc_interrupt_bool: bool, // Renamed to avoid confusion with Interrupt enum
}

impl GPU {
    pub fn new() -> Self {
        // Initialize tiles with a capacity of 384 (standard for Game Boy)
        let mut tiles = Vec::with_capacity(384);
        for _ in 0..384 {
            tiles.push(Tile::new());
        }

        // Initialize object data with default "hidden" values
        let mut object_data = [(); NUM_OBJ].map(|_| Object::default());

        GPU {
            vram: [0; VRAM_SIZE],
            oam: [0; OAM_SIZE],
            canvas_buffer: [0; SCREEN_WIDTH * SCREEN_HEIGHT * 4],
            tiles,
            object_data,
            lcd: LCD::new(),
            modes: Modes::OAM,
            cycles: 0,
            lyc_flag: false,
            lyc_interrupt_bool: false,
        }
    }
    pub fn write_oam(&mut self, index: usize, value: u8) {
        self.oam[index] = value;
        let object_index = index / 4;
        if object_index >= NUM_OBJ { return; }

        let byte = index % 4;
        let data = &mut self.object_data[object_index];
        match byte {
            0 => data.y = (value as i16) - 16,
            1 => data.x = (value as i16) - 8,
            2 => data.tile_index = value as usize,
            _ => {
                data.palette = if (value & 0x10) != 0 { Palette::OBJ } else { Palette::BGP };
                data.x_flip = (value & 0x20) != 0;
                data.y_flip = (value & 0x40) != 0;
                data.priority = (value & 0x80) == 0;
            }
        }
    }

    pub fn write_vram(&mut self, addr: usize, value: u8) {
        self.vram[addr] = value;
        let tile_index = addr / 16;
        if tile_index >= self.tiles.len() { return; }

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

    pub fn update(&mut self, machine_cycles: u8) -> Interrupt {
        let mut interrupt_request = Interrupt::None;
        if !self.lcd.control.lcd_ppu_enable() {
            self.cycles = 0;
            self.lcd.ly = 0;
            self.modes = Modes::VBlank;
            return interrupt_request;
        }

        self.cycles += machine_cycles as u32;

        match self.modes {
            Modes::OAM => {
                if self.cycles >= 80 {
                    self.cycles -= 80;
                    self.modes = Modes::Pixel;
                }
            }
            Modes::Pixel => {
                if self.cycles >= 172 {
                    self.cycles -= 172;
                    self.modes = Modes::HBlank;
                    self.render_scanline();
                }
            }
            Modes::HBlank => {
                if self.cycles >= 204 {
                    self.cycles -= 204;
                    self.lcd.ly += 1;
                    if self.lcd.ly >= 144 {
                        self.modes = Modes::VBlank;
                        interrupt_request.add(Interrupt::VBlank);
                    } else {
                        self.modes = Modes::OAM;
                    }
                    self.check_line_comparison(&mut interrupt_request);
                }
            }
            Modes::VBlank => {
                if self.cycles >= 456 {
                    self.cycles -= 456;
                    self.lcd.ly += 1;
                    if self.lcd.ly > 153 {
                        self.lcd.ly = 0;
                        self.modes = Modes::OAM;
                    }
                    self.check_line_comparison(&mut interrupt_request);
                }
            }
        }
        interrupt_request
    }

    fn check_line_comparison(&mut self, interrupt_request: &mut Interrupt) {
        let coincidence = self.lcd.ly == self.lcd.lyc;
        if coincidence && self.lyc_interrupt_bool {
            interrupt_request.add(Interrupt::LCDStat);
        }
        self.lyc_flag = coincidence;
    }

    fn render_scanline(&mut self) {
        let mut bg_pixel_ids = [0u8; SCREEN_WIDTH];
        let line = self.lcd.ly;
        let lcdc = self.lcd.control;

        // 1. BACKGROUND & WINDOW
        if lcdc.bg_window_enable() {
            for x in 0..SCREEN_WIDTH as u8 {
                let is_window = lcdc.window_enable() && line >= self.lcd.window_y && x >= self.lcd.window_x.wrapping_sub(7);

                let (base_addr, map_x, map_y) = if is_window {
                    (lcdc.window_tile_map_area(), x + 7 - self.lcd.window_x, line - self.lcd.window_y)
                } else {
                    (lcdc.bg_tile_map_area(), x.wrapping_add(self.lcd.scroll_x), line.wrapping_add(self.lcd.scroll_y))
                };

                let map_offset = base_addr - 0x8000;
                let addr = map_offset + ((map_y as u16 / 8) * 32) + (map_x as u16 / 8);
                let raw_idx = self.vram[addr as usize];

                let tile_idx = if lcdc.bg_window_tile_data_area() == 0x8000 {
                    raw_idx as usize
                } else {
                    ((raw_idx as i8 as i16) + 128) as usize + 128
                };

                let color_id = self.tiles[tile_idx].get_color_id((map_y % 8) as usize, (map_x % 8) as usize);
                bg_pixel_ids[x as usize] = color_id;

                let shade = (self.lcd.bg_palette >> (color_id * 2)) & 0b11;
                if let Some(color) = Palette::BGP.map_color(shade) {
                    let i = (line as usize * SCREEN_WIDTH + x as usize) * 4;
                    self.canvas_buffer[i..i+4].copy_from_slice(&[color.r, color.g, color.b, 255]);
                }
            }
        }

        // 2. SPRITES
        if lcdc.obj_enable() {
            let (_, height) = lcdc.obj_size();
            for obj in &self.object_data {
                let obj_y = obj.y;
                if obj_y <= line as i16 && (obj_y + height as i16) > line as i16 {
                    let mut py = if obj.y_flip { (height as i16 - 1) - (line as i16 - obj_y) } else { line as i16 - obj_y };
                    let t_idx = if height == 16 {
                        let base = obj.tile_index & 0xFE;
                        if py >= 8 { py -= 8; base + 1 } else { base }
                    } else { obj.tile_index };

                    let tile = &self.tiles[t_idx];
                    for ox in 0..8 {
                        let px = obj.x + ox;
                        if px >= 0 && px < SCREEN_WIDTH as i16 {
                            let px_off = if obj.x_flip { 7 - ox } else { ox };
                            let cid = tile.get_color_id(py as usize, px_off as usize);
                            if cid != 0 && (obj.priority || bg_pixel_ids[px as usize] == 0) {
                                let pal = if obj.palette == Palette::OBJ { self.lcd.obj_palette_0 } else { self.lcd.obj_palette_1 };
                                let shade = (pal >> (cid * 2)) & 0b11;
                                if let Some(c) = Palette::OBJ.map_color(shade) {
                                    let i = (line as usize * SCREEN_WIDTH + px as usize) * 4;
                                    self.canvas_buffer[i..i+4].copy_from_slice(&[c.r, c.g, c.b, 255]);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
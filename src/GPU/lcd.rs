use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::pixels::Color;
use sdl2::rect::Point;

use crate::GPU::gpu::{Palette};
use crate::GPU::tile::Tile;

pub struct LCD {
    pub canvas: Canvas<Window>,
    pub bg_palette: Palette,
    pub obj_palette: Palette,
    pub enabled: bool,
}

impl LCD {
    pub fn new(canvas: Canvas<Window>) -> Self {
        LCD {
            canvas,
            bg_palette: Palette::BGP,
            obj_palette: Palette::OBJ,
            enabled: true,
        }
    }

    pub fn clear(&mut self) {
        self.canvas.set_draw_color(Color::RGB(155, 188, 15)); // Game Boy background color
        self.canvas.clear();
    }

    pub fn draw_tile(&mut self, tile: &Tile) {
        // Retrieve tile pixel data
        for (color, point) in tile.get_tile_data() {
            self.canvas.set_draw_color(color);
            // draw_point expects an sdl2::rect::Point
            let _ = self.canvas.draw_point(point);
        }
    }

    pub fn present(&mut self) {
        self.canvas.present();
    }
}
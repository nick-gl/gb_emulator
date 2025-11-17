use crate::GPU::gpu::{Palette, Position};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use std::time::Duration;
pub struct Tile {
    palette: Palette,
    position: Position,
    color: u128
}
impl Tile {
     pub fn new(color: u128, position: Position,palette: Palette) -> Tile {
         Tile {
             color,
             position,
             palette
         }
     }
      pub fn get_tile_data(&self) -> Vec<(Color, Point)> {
            let mut pixels = Vec::with_capacity(64);


            for y in 0..8 {
                for x in 0..8 {

                    let bit_index = (y * 8 + x) * 2;

                    let color_num = ((self.color >> bit_index) & 0b11) as u8;

                    let color = self.palette.map_color(color_num).unwrap();

                    let point = Point::new(self.position.x+ x, self.position.y + y);

                    pixels.push((color, point));
                }
            }

            pixels
        }
    }
//test
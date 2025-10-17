use::gpu::{Palette,Position};
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
    pub fn draw(&self) -> Vec<Color::RGB,Point> {
        if self.palette == Palette::BGP {

        }
    }
}
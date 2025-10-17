use sdl2::{event::Event, keyboard::Keycode, pixels::PixelFormatEnum, render::WindowCanvas, Sdl};
pub enum Palette {
    BGP,
    OBJ,
}
pub struct Position {
    x: i32,
    y: i32,
}
pub struct GPU {
    renderer: sdl2::render::Renderer<'static>,
    tiles: Vec<Tile>,
    palette: Palette,

}
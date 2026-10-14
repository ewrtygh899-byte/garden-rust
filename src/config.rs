pub const CHUNK_W: usize = 16;
pub const CHUNK_H: usize = 16;
pub const TILE_SIZE: f32 = 32.0;

use macroquad::prelude::*;

pub fn background_color() -> Color {
    Color::new(135.0 / 255.0, 206.0 / 255.0, 250.0 / 255.0, 1.0)
}

pub fn grass_color() -> Color {
    Color::new(34.0 / 255.0, 139.0 / 255.0, 34.0 / 255.0, 1.0)
}

pub fn border_color() -> Color {
    BLACK
}
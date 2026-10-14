use macroquad::prelude::*;

use crate::config::{background_color, border_color, grass_color, CHUNK_H, CHUNK_W, TILE_SIZE};
use crate::world::Rect;

pub struct Renderer {
    tile_size: f32,
}

impl Renderer {
    pub fn new() -> Self {
        Self { tile_size: TILE_SIZE }
    }

    pub fn draw_background(&self) {
        clear_background(background_color());
    }

    pub fn draw_chunk(&self, rects: &[Rect]) {
        let screen_w = screen_width();
        let screen_h = screen_height();
        let win_w = (CHUNK_W as f32) * self.tile_size;
        let win_h = (CHUNK_H as f32) * self.tile_size;
        let offset_x = (screen_w - win_w) / 2.0;
        let offset_y = (screen_h - win_h) / 2.0;

        draw_rectangle_lines(offset_x - 1.0, offset_y - 1.0, win_w + 2.0, win_h + 2.0, 2.0, BLACK);

        for rect in rects {
            let px = offset_x + (rect.x as f32) * self.tile_size;
            let py = offset_y + (rect.y as f32) * self.tile_size;
            let pw = (rect.w as f32) * self.tile_size;
            let ph = (rect.h as f32) * self.tile_size;

            draw_rectangle(px, py, pw, ph, grass_color());
            draw_rectangle_lines(px, py, pw, ph, 2.0, border_color());
        }
    }
}
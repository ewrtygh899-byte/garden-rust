use macroquad::prelude::*;

use crate::config::{CHUNK_H, CHUNK_W};
use crate::render::renderer::Renderer;
use crate::world::{generate_chunk, greedy_mesh, Chunk};

pub struct App {
    chunk: Chunk,
    rects: Vec<crate::world::Rect>,
    renderer: Renderer,
}

impl App {
    pub fn new() -> Self {
        let mut chunk = Chunk::new(CHUNK_W, CHUNK_H);
        generate_chunk(&mut chunk);
        let rects = greedy_mesh(&chunk);

        Self {
            chunk,
            rects,
            renderer: Renderer::new(),
        }
    }

    pub fn update(&mut self) {
        // Reserved for future game logic / simulation.
    }

    pub fn draw(&self) {
        self.renderer.draw_background();
        self.renderer.draw_chunk(&self.rects);

        draw_text(
            "Greedy meshing: green blocks on blue background. Press ESC to quit.",
            10.0,
            20.0,
            20.0,
            BLACK,
        );
    }

    pub async fn run(&mut self) {
        loop {
            self.update();
            self.draw();

            if is_key_pressed(KeyCode::Escape) {
                break;
            }

            next_frame().await;
        }
    }
}
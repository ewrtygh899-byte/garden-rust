mod app;
mod config;
mod world;
mod render;

use crate::app::App;

#[macroquad::main("Garden Rust - Chunk Viewer")]
async fn main() {
    let mut app = App::new();
    app.run().await;
}

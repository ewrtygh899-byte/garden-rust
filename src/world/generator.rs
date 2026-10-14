use crate::config::{CHUNK_H, CHUNK_W};
use crate::world::chunk::Chunk;

pub fn generate_chunk(chunk: &mut Chunk) {
    chunk.fill_rect(2, 4, 12, 8, true);

    for y in 0..CHUNK_H {
        for x in 0..CHUNK_W {
            if x >= 2 && x < 14 && y >= 4 && y < 12 {
                continue;
            }

            let should_fill = (y >= 8 && x >= 8 && x < 12) || (x >= 6 && x < 10 && y >= 2 && y < 4);
            if should_fill {
                chunk.set_voxel(x, y, true);
            }
        }
    }

    for hole in [(6, 6), (9, 10), (5, 3), (11, 8), (3, 9)] {
        chunk.set_voxel(hole.0, hole.1, false);
    }
}